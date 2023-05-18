use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::{json, Value};

#[test]
fn test_to_string() {
    println!("{}", serde_json::to_string(&1).unwrap());
    println!("{}", serde_json::to_string(&3.14).unwrap());
    println!("{}", serde_json::to_string(&true).unwrap());
    println!("{}", serde_json::to_string("abc").unwrap());
    println!("{}", serde_json::to_string(&vec!["abc"]).unwrap());

    let item = json!({
        "key1": "value",
        "key2": ["val", "val", "val"],
        "key3": { "keyX": 12 }
    });
    println!("{}", item);

    let mut m: HashMap<&str, Value> = HashMap::new();
    m.insert("str", Value::String("abc".into()));
    m.insert("int", Value::Number(1.into()));
    m.insert("float", json!(3.14));
    m.insert("bool", Value::from(true));
    m.insert("array", Value::Array(vec![Value::Null, Value::Number(1.into()), Value::Bool(false)]));
    println!("{}", serde_json::to_string(&m).unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_serde_from_str() {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
