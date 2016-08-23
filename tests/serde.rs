extern crate serde_json;
extern crate kv_access;

use serde_json as json;
use serde_json::value::Value;

use kv_access::Attr;
use kv_access::serde_impl::SerdeAttribute;

#[test]
fn test_attr() {
    let obj: Value = json::from_str(r#"{"x": 1}"#).unwrap();

    let attr = SerdeAttribute::new("x");

    println!("{:?}", attr.get(&obj));
    assert_eq!(attr.get(&obj), &Value::U64(1));
}