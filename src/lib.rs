use std::{
    fs::File,
    io::Write,
    error::Error,
    path::Path,
};
use serde::Serialize;

pub fn save_to_csv<T: Serialize>(items: &[T], filename: &str) -> Result<(), Box<dyn Error>> {
    let project_root = dotenv::var("PROJECT_ROOT").expect("PROJECT_ROOT should be present in .env");
    let mut wtr = csv::Writer::from_writer(vec![]);
    for item in items {
        wtr.serialize(item)?;
    }

    write!(File::create(format!("{project_root}/data/{filename}"))?,
           "{}", String::from_utf8(wtr.into_inner()?)?)?;
    Ok(())
}

pub fn save_as_json<T: Serialize>(items: &[T], location: &Path) -> Result<(), Box<dyn Error>> {
    let items_as_json = serde_json::to_string_pretty(items).unwrap();
    write!(File::create(location)?, "{}", items_as_json)?;
    println!("[INFO]: Wrote {} items to {:?}", items.len(), location);
    Ok(())
}
