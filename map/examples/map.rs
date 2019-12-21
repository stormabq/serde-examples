
fn main() {

use serde_json::{Map, Value};

let mut m = Map::new();
m.insert("Lorem".to_string(), "ipsum".into());
let x: Value = m.into();
println!("{}",x);


}
