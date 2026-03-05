use std::{any::{Any, type_name, type_name_of_val}, collections::HashMap, fmt::Display, path::{Path, PathBuf}};
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

type Ttop = [String; 5];

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

    let playlists: Vec<Playlist> = spotiver::vec_from_json(&bkp_path.join("playlists.json"))
	.unwrap();
    db.fill_playlists(&playlists);

    let mut tracks_map: HashMap<String, Track> = HashMap::new();
    let mut ttop: Vec<Ttop> = Vec::new();
    for p in &playlists {
	let mut path = bkp_path.to_path_buf();
	path.push(&p.id);
	path.push("tracks.json");
	if let Ok(tracks) = spotiver::vec_from_json::<Track>(&path) {
	    for t in tracks {
		ttop.push([t.id.clone(), t.name.clone(), p.id.clone(), p.name.clone(), t.added_at.clone()]);
		let _ = tracks_map.insert(t.id.clone(), t);
	    }
	}
    }
    db.fill_tracks(&tracks_map);
    db.fill_ttop(&ttop);
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
		table.dbt.to_string(), table.dbt.to_string(), table.stringify_schema()
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

    fn insert_single_prepared(&self, dbt: DbTable, values: &[String]) {
	if let Some(table) = self.tables.get(&dbt) {
	    let num_rows = values.len() / table.schema.len();
	    let query_truncated = format!(
		"INSERT INTO {} {} VALUES ({} rows);",
		table.dbt.to_string(), table.columns(), num_rows
	    );
	    let placeholders: String = (0..num_rows)
		.map(|_| table.placeholders())
		.collect::<Vec<String>>()
		.join(", ");
	    let query = format!(
		"INSERT INTO {} {} VALUES {};",
		table.dbt.to_string(), table.columns(), placeholders
	    );
	    println!("{}", values.len());
	    let mut stmt = self.con.prepare(&query).unwrap();
	    let values_indexed: Vec<(usize, &str)> = values.iter()
		.enumerate()
		.map(|(i, v)| (i+1, v.as_str()))
		.collect();
	    stmt.bind_iter(values_indexed); // equivalent to stmt.bind(&values_indexed[..])
	    handle!(stmt.next(), query_truncated, query);
	    stmt.reset();
	}
    }

    fn insert_multiple_prepared(&self, dbt: DbTable, rows: &[Vec<String>]) {
	let table = self.tables.get(&dbt).unwrap();
	let chunk_size = usize::min(rows.len(), 10000);
	let num_rows = chunk_size / table.schema.len();
	let placeholders: String = (0..num_rows)
	    .map(|_| table.placeholders())
	    .collect::<Vec<String>>()
	    .join(", ");
	let query = format!(
	    "INSERT INTO {} {} VALUES {};",
	    table.dbt.to_string(), table.columns(), placeholders
	);
	let mut stmt = self.con.prepare(&query).unwrap();
	for chunk in rows.chunks(chunk_size) {
	    let query_truncated = format!(
		"INSERT INTO {} {} VALUES ({} rows);",
		table.dbt.to_string(), table.columns(), chunk.len()
	    );
	    let values: Vec<&str> = chunk.iter()
		.flatten()
		.map(|s| s.as_str())
		.collect();
	    stmt.bind(&values[..]);
	    handle!(stmt.next(), query_truncated);
	    stmt.reset();
	}
    }

    fn fill_playlists(&self, json_values: &[Playlist]) {
	assert!(self.tables.get(&DbTable::Playlists).is_some());
	let values: Vec<Vec<String>> = json_values.into_iter()
	    .map(|p| {
		let image = p.images.first().map_or("NULL".to_string(), |img| img.url.clone());
		vec![p.id.clone(), p.name.clone(), p.external_urls.spotify.clone(), image, p.tracks.total.to_string()]
	    })
	    .collect();
	self.insert_multiple_prepared(DbTable::Playlists, &values);
    }

    fn fill_tracks(&self, json_values: &HashMap<String, Track>) {
	assert!(self.tables.get(&DbTable::Tracks).is_some());
	let values: Vec<Vec<String>> = json_values.iter()
	    .map(|(_, t)| {
		let album_id = t.album.id.clone().unwrap_or("".to_string());
		let duration = t.duration_ms.as_i64().to_string();
		let track_number = t.track_number.as_i64().to_string();
		vec![t.id.clone(), t.name.clone(), t.album.name.clone(), album_id, t.external_urls.spotify.clone(), duration, track_number]
	    })
	    .collect();
	self.insert_multiple_prepared(DbTable::Tracks, &values);
    }

    fn fill_ttop(&self, json_values: &[Ttop]) {
	assert!(self.tables.get(&DbTable::Ttop).is_some());
	let values: Vec<Vec<String>> = json_values.iter()
	    .map(|s| s.into())
	    .collect();
	self.insert_multiple_prepared(DbTable::Ttop, &values);
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

    fn stringify_schema(&self) -> String {
	let cols: Vec<String> = self.schema.iter()
	    .map(|c| c.to_string())
	    .collect();
	format!("({})", cols.join(", "))
    }

    fn columns(&self) -> String {
	let cols: Vec<&str> = self.schema.iter()
	    .map(|c| c.name.as_str())
	    .collect();
	format!("({})", cols.join(", "))
    }

    fn placeholders(&self) -> String {
	format!(
	    "({})",
	    self.schema.iter()
		.map(|_| "?")
		.collect::<Vec<&str>>()
		.join(", ")
	)
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
