use std::{any::{Any, type_name, type_name_of_val}, collections::HashMap, fmt::Display, path::{Path, PathBuf}};
use sqlite::{self, Connection, State, Value};
use crate::objects::{playlist::Playlist, track::Track, sj_number::SjNumber};

trait NewTrait: Any + Display {}
impl NewTrait for String {}
impl NewTrait for i64 {}
impl NewTrait for SjNumber {}

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
    // db.fill_playlists(&playlists);
    db.fill_playlists_strings(&playlists);

    let mut tracks_map: HashMap<String, Track> = HashMap::new();
    let mut ttop: Vec<Ttop> = Vec::new();
    for p in &playlists {
	let mut path = bkp_path.to_path_buf();
	path.push(&p.id);
	path.push("tracks.json");
	if let Ok(tracks) = spotiver::vec_from_json::<Track>(&path) {
	    for t in tracks {
		ttop.push(Ttop::new(&t, p));
		let _ = tracks_map.insert(t.id.clone(), t);
	    }
	}
    }
    // db.fill_tracks(&tracks_map);
    db.fill_tracks_strings(&tracks_map);
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

    fn insert_multiple_strings(&self, dbt: DbTable, rows: &[String]) {
	let table = self.tables.get(&dbt).unwrap();
	let chunk_size = usize::min(rows.len(), 10000);
	let num_rows = chunk_size / table.schema.len();
	for chunk in rows.chunks(chunk_size) {
	    let query = format!(
		"INSERT INTO {} {} VALUES {};",
		table.dbt.to_string(), table.columns(), chunk.join(", ")
	    );
	    let query_truncated = format!(
		"INSERT INTO {} {} VALUES ({} rows);",
		table.dbt.to_string(), table.columns(), chunk.len()
	    );
	    let res = self.con.execute(&query);
	    handle!(res, query_truncated, query);
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

    fn fill_playlists_strings(&self, json_values: &[Playlist]) {
	let table = self.tables.get(&DbTable::Playlists).unwrap();
	let values: Vec<String> = json_values.into_iter()
	    .map(|p| {
		let image = p.images.first().map_or("NULL".to_string(), |img| img.url.clone());
		let mut col_iter = table.schema.iter();
		let v: Vec<String> = vec![
		    col_iter.next().expect("column mismatch").print(&p.id),
		    col_iter.next().expect("column mismatch").print(&p.name),
		    col_iter.next().expect("column mismatch").print(&p.external_urls.spotify),
		    col_iter.next().expect("column mismatch").print(&image),
		    col_iter.next().expect("column mismatch").print(&p.tracks.total)
		];
		format!("({})", v.join(", "))
	    })
	    .collect();
	self.insert_multiple_strings(DbTable::Playlists, &values);
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

    fn fill_tracks_strings(&self, json_values: &HashMap<String, Track>) {
	let table = self.tables.get(&DbTable::Tracks).unwrap();
	let values: Vec<String> = json_values.into_iter()
	    .map(|(_, t)| {
		let album_id = t.album.id.clone().unwrap_or("".to_string());
		let duration = t.duration_ms.as_i64();
		let track_number = t.track_number.as_i64();
		let mut col_iter = table.schema.iter();
		let v: Vec<String> = vec![
		    col_iter.next().expect("column mismatch").print(&t.id),
		    col_iter.next().expect("column mismatch").print(&t.name),
		    col_iter.next().expect("column mismatch").print(&t.album.name),
		    col_iter.next().expect("column mismatch").print(&album_id),
		    col_iter.next().expect("column mismatch").print(&t.external_urls.spotify),
		    col_iter.next().expect("column mismatch").print(&duration),
		    col_iter.next().expect("column mismatch").print(&track_number),
		];
		format!("({})", v.join(", "))
	    })
	    .collect();
	self.insert_multiple_strings(DbTable::Tracks, &values);
    }

    fn fill_ttop(&self, json_values: &[Ttop]) {
	let table = self.tables.get(&DbTable::Ttop).unwrap();
	let values: Vec<String> = json_values.iter()
	    .map(|t| {
		let mut col_iter = table.schema.iter();
		let v: Vec<String> = vec![
		    col_iter.next().expect("column mismatch").print(&t.track_id),
		    col_iter.next().expect("column mismatch").print(&t.track_name),
		    col_iter.next().expect("column mismatch").print(&t.playlist_id),
		    col_iter.next().expect("column mismatch").print(&t.playlist_name),
		    col_iter.next().expect("column mismatch").print(&t.added_at),
		];
		format!("({})", v.join(", "))
	    })
	    .collect();
	self.insert_multiple_strings(DbTable::Ttop, &values);
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

    fn print<T: Display>(&self, value: &T) -> String {
	match self.datatype {
	    DT::Text => {
		let v = value.to_string()
		    .replace("\"", "'");
		format!("\"{}\"", v)
	    },
	    DT::Integer => value.to_string(),
	    DT::Numeric => value.to_string(),
	}
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

/* Track-to-Playlist */
#[derive(Debug)]
struct Ttop {
    track_id: String,
    track_name: String,
    playlist_id: String,
    playlist_name: String,
    added_at: String // TODO: Date Type ???
}

impl Ttop {
    fn new(t: &Track, p: &Playlist) -> Self {
	Ttop {
	    track_id: t.id.clone(),
	    track_name: t.name.clone(),
	    playlist_id: p.id.clone(),
	    playlist_name: p.name.clone(),
	    added_at: t.added_at.clone()
	}
    }
}
