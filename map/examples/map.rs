use serde_json::{Map, Value};

fn main() {
    let mut m = Map::new();
    m.insert("Lorem".to_string(), "ipsum".into());
    m.insert("count".to_string(), 4.into());
    let x: Value = m.into();
    println!("{}", x);
}
