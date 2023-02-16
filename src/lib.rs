use std::{
    fs::File,
    io::Write,
    error::Error
};
use serde::Serialize;
use csv::Writer;

pub fn save_to_csv<T: Serialize>(items: &[T], filename: &str) -> Result<(), Box<dyn Error>> {
    let project_root = dotenv::var("PROJECT_ROOT").expect("PROJECT_ROOT should be present in .env");
    let mut wtr = Writer::from_writer(vec![]);
    for item in items {
        wtr.serialize(item)?;
    }

    write!(File::create(format!("{project_root}/data/{filename}"))?,
           "{}", String::from_utf8(wtr.into_inner()?)?)?;
    Ok(())
}
