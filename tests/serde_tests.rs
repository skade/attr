extern crate serde_json;
extern crate attr;

mod serde;

use serde::*;

use serde_json as json;
use serde_json::value::Value;

use attr::Attr;
use attr::InsecureAttr;
use attr::InsecureIndexableAttr;

use attr::retrieve_insecure;
use attr::Traverse;

#[test]
fn test_attr() {
    let obj: Value = json::from_str(r#"{"x": 1, "y": [1,2,3] }"#).unwrap();

    let attr_x = SerdeAttribute::new("x");
    let attr_y = SerdeAttribute::new("y");

    assert_eq!(attr_x.get(&obj).unwrap(), &Value::U64(1));
    assert_eq!(attr_y.at(&obj, 1).unwrap(), &Value::U64(2));
}

#[test]
fn test_multiple_attr() {
    let obj: Value = json::from_str(r#"{"x": 1, "y": { "z": 1 } }"#).unwrap();

    let attr_z = SerdeAttribute::new("z");
    let attr_y = SerdeAttribute::new("y");

    let path = retrieve_insecure(attr_z).try(attr_y);

    assert_eq!(path.traverse(&obj), Ok(&Value::U64(1)));
}

struct Foo {
    inner: Value
}

#[test]
fn test_combine() {
    use serde_json::value::Value;

    #[derive(Default)]
    struct Inner;

    impl<'a> Attr<&'a Foo> for Inner {
        type Output = &'a Value;

        fn get(&self, i: &'a Foo) -> &'a Value {
            &i.inner
        }

        fn name(&self) -> &'static str {
            "inner"
        }
    }

    let val: Value = json::from_str(r#"{"x": 1}"#).unwrap();
    let obj = Foo { inner: val };
    let attr = SerdeAttribute::new("x");

    let path = retrieve_insecure(attr).from(Inner);

    assert_eq!(path.traverse(&obj), Ok(&Value::U64(1)));
}
