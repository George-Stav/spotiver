use std::{collections::HashMap, fmt::Display, path::{Path, PathBuf}};
use sqlite::{self, Connection, State, Value};
use crate::objects::playlist::Playlist;

pub fn create() {
    let bkp_path = Path::new("/home/george/BULK/spotiver.bkp");
    let pl_path = bkp_path.join("playlists.json");
    let playlists: Vec<Playlist> = spotiver::vec_from_json(&pl_path).unwrap();

    let schema = vec![
	Column::new("id", DT::Text, vec![Cons::Primary_Key]),
	Column::new("name", DT::Text, vec![Cons::Not_Null]),
	Column::new("url", DT::Text, vec![Cons::Not_Null]),
	Column::new("image", DT::Text, vec![]),
	Column::new("tracks", DT::Integer, vec![Cons::Not_Null]),
    ];
    let t = Table::new(schema, "playlists");
    let mut db = Db::init("/home/george/BULK/spotiver.bkp");
    db.tables.insert(DbTable::Playlists, t);
    db.create();
    db.fill_playlists(&playlists);
    // let mut hm: HashMap<String, usize> = HashMap::new();
    // for p in playlists {
    // 	hm.entry(p.id).and_modify(|v| *v+=1).or_insert(1);
    // }
    // println!("{:#?}", hm);
}

struct Table {
    schema: Vec<Column>,
    name: String
}

impl Table {
    fn new(schema: Vec<Column>, name: &str) -> Self {
	Table {schema, name: name.to_string()}
    }

    // TODO: Create macro for Vec<String>.join(...) pattern
    fn schema(&self) -> String {
	let cols: Vec<String> = self.schema.iter()
	    .map(|c| c.to_string())
	    .collect();
	format!("({})", cols.join(", "))
    }

    fn columns(&self) -> String {
	let cols: Vec<String> = self.schema.iter()
	    .map(|c| c.name.clone())
	    .collect();
	format!("({})", cols.join(", "))
    }
}

pub struct Db {
    con: Connection,
    tables: HashMap<DbTable, Table>,
    path: PathBuf
}

impl Db {
    pub fn init(p: &str) -> Self {
	let path = Path::new(p).to_path_buf();
	let con = match sqlite::open("spotiver.db") {
	    Ok(con) => con,
	    Err(e) => panic!("Failed to open sqlite DB with: {}", e)
	};
	Db {con, tables: HashMap::new(), path}
    }

    fn exec(&self, query: String) -> sqlite::Result<()> {
	println!("{}", query.replace(";", ";\n"));
	self.con.execute(query)
    }

    fn create(&self) {
	for table in self.tables.values() {
	    let query = format!(
		"DROP TABLE IF EXISTS {};CREATE TABLE {} {};",
		table.name, table.name, table.schema()
	    );
	    self.exec(query).unwrap();
	}
    }

    fn insert(&self, dbt: DbTable, values: &[String]) {
	if let Some(table) = self.tables.get(&dbt) {
	    let query = format!("INSERT INTO {} {} VALUES {};", table.name, table.columns(), values.join(", "));
	    self.exec(query).unwrap()
	}
    }

    fn fill_playlists(&self, json_values: &[Playlist]) {
	let values: Vec<String> = json_values.iter()
	    .map(|p| {
		let image = p.images.first().map_or("NULL", |img| img.url.as_str());
		format!("({:?}, {:?}, {:?}, {:?}, {})",
			p.id, p.name, p.external_urls.spotify, image, p.tracks.total.to_string())
	    })
	    .collect();
	self.insert(DbTable::Playlists, &values);
    }
}

#[derive(Debug)]
enum DT {
    Text,
    Integer
}

impl Display for DT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let str = format!("{:?}", self);
	write!(f, "{}", str.to_uppercase())
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Cons {
    Primary_Key,
    Unique,
    Not_Null,
    Null
}

impl Display for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let str = match self {
	    Cons::Primary_Key => "Primary Key",
	    Cons::Not_Null => "Not Null",
	    Cons::Null => "Null",
	    Cons::Unique => "Unique"
	};

	write!(f, "{}", str.to_uppercase())
    }
}

#[derive(Debug)]
struct Column {
    name: String,
    datatype: DT,
    constraints: Vec<Cons>
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let constraints: Vec<String> = self.constraints.iter()
	    .map(|c| c.to_string())
	    .collect();
        write!(f, "{} {} {}", self.name, self.datatype, constraints.join(" "))
    } 
}

impl Column {
    fn new(name: &str, datatype: DT, cons: Vec<Cons>) -> Self {
	Column {
	    name: name.to_string(),
	    datatype: datatype,
	    constraints: cons
	}
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum DbTable {
    Playlists,
    Tracks
}
