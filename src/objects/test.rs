#[allow(dead_code)]

fn main() {
    macro_rules! concat {
        ($v:expr, $var:ident) => {
            $v.iter().map(|t| t.$var.to_string()).collect::<Vec<String>>().join("|")
        };
    }

    struct Test<'x> {
        a: &'x str,
        b: i8,
    }

    let v1: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let v2: Vec<Test> = vec![
        Test{a: "10", b: 0},
        Test{a: "20", b: 1},
        Test{a: "30", b: 2}
    ];
    let v3: Option<Vec<String>> = None;
    let v4: Option<Vec<String>> = Some(vec!["one".to_string(), "two".to_string(), "three".to_string()]);

    println!("{:?}", v1.join("|"));
    println!("{:?}", concat!(v2, a));
    println!("{:?}", v3.unwrap_or_default().join("|"));
    println!("{:?}", v4.unwrap_or_default().join("|"));
}
