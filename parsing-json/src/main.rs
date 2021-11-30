extern crate serde_json;

use serde_json::Value as JsonValue;

fn main() {
    let mut json_str = r#"
    {
    "name":"CJ",
    "age":30,
    "is_male":"true"
    }
    "#;
    let res = serde_json::from_str(json_str);
    if res.is_ok() {
        let p: JsonValue = res.unwrap();
        println!("The name is {}", p["name"].as_str().unwrap());
    } else { println!("Sorry could not parse JSON :(") }
}
