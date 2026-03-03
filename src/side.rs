use std::{any::Any, fmt::Display};

trait NewTrait: Any + Display {}
impl NewTrait for String {}
impl NewTrait for i64 {}

fn main() {
    let v: Vec<Box<dyn NewTrait>> = vec![
	Box::new("one".to_string()),
	Box::new(2)
    ];
    println!("{:?}", v.iter().map(|v| v.to_string()).collect::<Vec<String>>());
}
