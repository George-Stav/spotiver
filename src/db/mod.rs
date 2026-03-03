use std::{collections::HashMap, fmt::Display, path::{Path, PathBuf}};
use sqlite::{self, Connection, State, Value};
use crate::objects::{playlist::Playlist, track::Track};

macro_rules! handle {
    ($res:expr, $success:expr, $failure:expr) => {
	match $res {
	    Ok(_) => println!("Success: {}", $success),
	    Err(e) => panic!("Error:   {e}\nQuery: {}", $failure),
	}
    };
    ($res:expr, $query:expr) => {
	match $res {
	    Ok(_) => println!("Success: {}", $query),
	    Err(e) => panic!("Error:   {e}\nquery: {}", $query),
	}
    };
}

type Ttop = (String, String, String, String, String);

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

    // TODO: Artist foreign key + is_local
    let tracks_schema = vec![
	Column::new("id", DT::Text, vec![Cons::Primary_Key]),
	Column::new("name", DT::Text, vec![Cons::Not_Null]),
	Column::new("album", DT::Text, vec![Cons::Not_Null]),
	Column::new("album_id", DT::Text, vec![Cons::Not_Null]),
	Column::new("url", DT::Text, vec![Cons::Not_Null]),
	Column::new("duration", DT::Integer, vec![Cons::Not_Null]),
	Column::new("track_number", DT::Integer, vec![Cons::Not_Null]),
    ];
    let t = Table::new(tracks_schema, DbTable::Tracks);
    db.tables.insert(DbTable::Tracks, t);

    // many-to-many table map between tracks and playlists
    let ttop_schema = vec![
	Column::new("track_id", DT::Text, vec![Cons::Not_Null]),
	Column::new("track_name", DT::Text, vec![Cons::Not_Null]),
	Column::new("playlist_id", DT::Text, vec![Cons::Not_Null]),
	Column::new("playlist_name", DT::Text, vec![Cons::Not_Null]),
	Column::new("added_at", DT::Text, vec![Cons::Not_Null]), // NOTE: Data Type???
    ];
    let m = Table::new(ttop_schema, DbTable::Ttop);
    db.tables.insert(DbTable::Ttop, m);


    db.create();

    let pl_path = bkp_path.join("playlists.json");
    let playlists: Vec<Playlist> = spotiver::vec_from_json(&pl_path).unwrap();
    db.fill_playlists(&playlists);

    // let mut tracks_map: HashMap<String, Track> = HashMap::new();
    // let mut ttop: Vec<Ttop> = Vec::new();
    // for p in &playlists {
    // 	let mut path = bkp_path.to_path_buf();
    // 	path.push(&p.id);
    // 	path.push("tracks.json");
    // 	if let Ok(tracks) = spotiver::vec_from_json::<Track>(&path) {
    // 	    for t in tracks {
    // 		ttop.push((t.id.clone(), t.name.clone(), p.id.clone(), p.name.clone(), t.added_at.clone()));
    // 		let _ = tracks_map.insert(t.id.clone(), t);
    // 	    }
    // 	}
    // }
    // db.fill_tracks(&tracks_map);
    // db.fill_ttop(&ttop);
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

    fn inc_insert(&self, dbt: DbTable, values: &[String]) {
	if let Some(table) = self.tables.get(&dbt) {
	    let num_chunks = values.len()/20;
	    for chunk in values.chunks(num_chunks) {
		let query_truncated = format!(
		    "INSERT INTO {} {} VALUES ({} rows);",
		    table.dbt.to_string(), table.columns(), chunk.len()
		);
		let query = format!(
		    "INSERT INTO {} {} VALUES {};",
		    table.dbt.to_string(), table.columns(), chunk.join(", ")
		);
		let res = self.con.execute(&query);
		handle!(res, query_truncated, query);
	    }
	}
    }


    fn insert(&self, dbt: DbTable, values: &[String]) {
	if let Some(table) = self.tables.get(&dbt) {
	    let query_truncated = format!(
		"INSERT INTO {} {} VALUES ({} rows);",
		table.dbt.to_string(), table.columns(), values.len()
	    );
	    let query = format!(
		"INSERT INTO {} {} VALUES {};",
		table.dbt.to_string(), table.columns(), values.join(", ")
	    );
	    let res = self.con.execute(&query);
	    handle!(res, query_truncated, query);
	}
    }

    fn fill_playlists(&self, json_values: &[Playlist]) {
	if let Some(table) = self.tables.get(&DbTable::Playlists) {
	    // playlist -> Iter<Item=&str>
	    let values: Vec<String> = json_values.iter()
		.map(|p| {
		    let image = p.images.first().map_or("NULL", |img| img.url.as_str());

		    table.bind_iter([
			&p.id, &p.name, &p.external_urls.spotify, &image.to_string(), &p.tracks.total.to_string()
		    ].map())
		})
		.collect();
	    self.insert(DbTable::Playlists, &values);
	}
    }

    fn fill_tracks(&self, json_values: &HashMap<String, Track>) {
	let values: Vec<String> = json_values.iter()
	    .map(|(_, t)| {
		let track_name = t.name.replace("\"", "'");
		let album_id = t.album.id.clone().unwrap_or("".to_string());
		let album_name = t.album.name.replace("\"", "'");
		let track_number = t.track_number.as_i64();
		format!(r#"("{}", "{}", "{}", "{}", "{}", {}, {})"#,
			t.id, track_name, album_name, album_id, t.external_urls.spotify, t.duration_ms, track_number)
	    })
	    .collect();
	self.insert(DbTable::Tracks, &values);
    }

    fn fill_ttop(&self, json_values: &[Ttop]) {
	let values: Vec<String> = json_values.iter()
	    .map(|(tid, tname, pid, pname, added_at)| {
		let track_name = tname.replace("\"", "'");
		format!(r#"("{}", "{}", "{}", "{}", "{}")"#,
			tid, track_name, pid, pname, added_at)
	    })
	    .collect();
	self.insert(DbTable::Ttop, &values);
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

    fn placeholders(&self) -> String {
	let ph: Vec<&str> = self.schema.iter()
	    .map(|_| "?")
	    .collect();
	format!("({})", ph.join(", "))
    }

    fn bind_iter<T, U>(&self, values: T) -> String
    where
	T: IntoIterator<Item = U>,
	U: Display
    {
	let strings: Vec<String> = values.into_iter()
	    .zip(self.schema.iter())
	    .map(|(v, col)| col.datatype.to_value(v))
	    .collect();
	format!("({})", strings.join(", "))
    }
}

#[derive(Debug)]
enum DT {
    Text,
    Integer,
    Numeric
}

impl Display for DT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let str = format!("{:?}", self);
	write!(f, "{}", str.to_uppercase())
    }
}

impl DT {
    fn to_value<T: Display>(&self, value: T) -> String {
	match self {
	    DT::Text => format!("\"{}\"", value),
	    DT::Integer => value.to_string(),
	    DT::Numeric => value.to_string(),
	}
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
    constraints: Vec<Cons>,
    // data: Vec<T>
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
    Tracks,
    Ttop
}

impl Display for DbTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let str = format!("{:?}", self);
	write!(f, "{}", str.to_lowercase())
    }
}
