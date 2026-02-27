use std::{
    fs,
    io::Write,
    error::Error,
    path::Path,
};
use serde::{
    Serialize,
    de::DeserializeOwned,
};

pub fn save_to_csv<T: Serialize>(items: &[T], filename: &str) -> Result<(), Box<dyn Error>> {
    let project_root = dotenv::var("PROJECT_ROOT").expect("PROJECT_ROOT should be present in .env");
    let mut wtr = csv::Writer::from_writer(vec![]);
    for item in items {
        wtr.serialize(item)?;
    }

    write!(fs::File::create(format!("{project_root}/data/{filename}"))?,
           "{}", String::from_utf8(wtr.into_inner()?)?)?;
    Ok(())
}

pub fn save_as_json<T: Serialize>(items: &[T], location: &Path) -> Result<(), Box<dyn Error>> {
    let items_as_json = serde_json::to_string_pretty(items).unwrap();
    write!(fs::File::create(location)?, "{}", items_as_json)?;
    println!("Info: Wrote {} items to {:?}", items.len(), location);
    Ok(())
}

pub fn vec_from_json<T: DeserializeOwned>(location: &Path) -> Result<Vec<T>, Box<dyn Error>> {
    let file_content = fs::read_to_string(&location)
        .map_err(|e| {
            println!("Error: Couldn't read file [{:?}]: {}", location, e);
            e
        })?;

    let list: Vec<T> = serde_json::from_str(&file_content)
        .map_err(|e| {
            println!("Error: Couldn't deserialise object in file [{:?}]: {}", location, e);
            e
        }).unwrap();

    Ok(list)
}
