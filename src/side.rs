use std::fmt::Display;

fn main() {
    let dt = DT::Text;
    println!("{}", dt);
}

#[derive(Debug)]
enum DT {
    Text,
    Integer
}

impl Display for DT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
