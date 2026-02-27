use std::{collections::HashMap, fmt::Display, path::{Path, PathBuf}};
use sqlite::{self, Connection, State, Value};
use crate::objects::{playlist::Playlist, track::Track};

macro_rules! handle {
    ($res:expr, $success:expr, $failure:expr) => {
	match $res {
	    Ok(_) => println!("Success: {}", $success),
	    Err(e) => panic!("Error: {e}\nQuery: {}", $failure),
	}
    };
    ($res:expr, $query:expr) => {
	match $res {
	    Ok(_) => println!("Success: {}", $query),
	    Err(e) => panic!("Error: {e}\nquery: {}", $query),
	}
    };
}

pub fn create() {
    let bkp_path = Path::new("/home/george/BULK/spotiver.bkp");
    let mut db = Db::init(&bkp_path);

    let playlists_schema = vec![
	Column::new("id", DT::Text, vec![Cons::Primary_Key]),
	Column::new("name", DT::Text, vec![Cons::Not_Null]),
	Column::new("url", DT::Text, vec![Cons::Not_Null]),
	Column::new("image", DT::Text, vec![]), // empty means nullable
	Column::new("tracks", DT::Integer, vec![Cons::Not_Null]),
    ];
    let p = Table::new(playlists_schema, DbTable::Playlists);
    db.tables.insert(DbTable::Playlists, p);

    let tracks_schema = vec![
	Column::new("id", DT::Text, vec![Cons::Primary_Key]),
	Column::new("name", DT::Text, vec![Cons::Not_Null]),
	Column::new("album", DT::Text, vec![Cons::Not_Null]),
	Column::new("album_id", DT::Text, vec![Cons::Not_Null]),
	Column::new("url", DT::Text, vec![Cons::Not_Null]),
	Column::new("added_at", DT::Text, vec![]), // NOTE: Data Type???
	Column::new("duration", DT::Integer, vec![Cons::Not_Null]),
	Column::new("disc_number", DT::Integer, vec![Cons::Not_Null]),
    ];
    let t = Table::new(tracks_schema, DbTable::Tracks);
    db.tables.insert(DbTable::Tracks, t);

    db.create();

    let pl_path = bkp_path.join("playlists.json");
    let playlists: Vec<Playlist> = spotiver::vec_from_json(&pl_path).unwrap();
    db.fill_playlists(&playlists);

    let mut tracks_map: HashMap<String, Track> = HashMap::new();
    for p in &playlists {
	let mut path = bkp_path.to_path_buf();
	path.push(&p.id);
	path.push("tracks.json");
	if let Ok(tracks) = spotiver::vec_from_json::<Track>(&path) {
	    for t in tracks {
		println!("{:#?}", t);
		let _ = tracks_map.insert(t.id.clone(), t);
	    }
	}
    }
    println!("{:?}", tracks_map);
}

pub struct Db {
    con: Connection,
    tables: HashMap<DbTable, Table>,
    path: PathBuf
}

impl Db {
    pub fn init(p: &Path) -> Self {
	let path = p.to_path_buf();
	let con = match sqlite::open("spotiver.db") {
	    Ok(con) => con,
	    Err(e) => panic!("Failed to open sqlite DB with: {}", e)
	};
	Db {con, tables: HashMap::new(), path}
    }

    fn create(&self) {
	for table in self.tables.values() {
	    let query = format!(
		"DROP TABLE IF EXISTS {}; CREATE TABLE {} {};",
		table.dbt.to_string(), table.dbt.to_string(), table.schema()
	    );
	    let res = self.con.execute(&query);
	    handle!(res, query);
	}
    }

    fn insert(&self, dbt: DbTable, values: &[String]) {
	if let Some(table) = self.tables.get(&dbt) {
	    let query_no_values = format!("INSERT INTO {} {} VALUES ({} rows);", table.dbt.to_string(), table.columns(), values.len());
	    let query = format!("INSERT INTO {} {} VALUES {};", table.dbt.to_string(), table.columns(), values.join(", "));
	    let res = self.con.execute(&query);
	    handle!(res, query_no_values, query);
	}
    }

    fn fill_playlists(&self, json_values: &[Playlist]) {
	let values: Vec<String> = json_values.iter()
	    .map(|p| {
		let image = p.images.first().map_or("NULL", |img| img.url.as_str());
		// TODO: Enforce column order (e.g. name should not be allowed to come after url)
		format!("({:?}, {:?}, {:?}, {:?}, {})", // debug print required to wrap strings with ""
			p.id, p.name, p.external_urls.spotify, image, p.tracks.total.to_string())
	    })
	    .collect();
	self.insert(DbTable::Playlists, &values);
    }
}

struct Table {
    schema: Vec<Column>,
    dbt: DbTable
}

impl Table {
    fn new(schema: Vec<Column>, dbt: DbTable) -> Self {
	Table {schema, dbt}
    }

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

// TODO: Enforce constraint rules (e.g. primary-key and not-null are redundunt)
// TODO: Foreign key
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Cons {
    Primary_Key,
    // Foreign_Key,
    Unique,
    Not_Null,
    Null
}

impl Display for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let str = match self {
	    Cons::Primary_Key => "Primary Key",
	    // Cons::Foreign_Key => "Foreign Key",
	    Cons::Not_Null => "Not Null",
	    Cons::Null => "Null",
	    Cons::Unique => "Unique",
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

impl Display for DbTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let str = format!("{:?}", self);
	write!(f, "{}", str.to_lowercase())
    }
}
