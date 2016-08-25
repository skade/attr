extern crate serde_json;
extern crate kv_access;

use serde_json as json;
use serde_json::value::Value;
use kv_access::IndexableAttr;

use kv_access::Attr;
use kv_access::Attributes;
use kv_access::Combine;
use kv_access::serde_impl::SerdeAttribute;

use kv_access::new_immutable_path;
use kv_access::Traverse;

#[test]
fn test_attr() {
    let obj: Value = json::from_str(r#"{"x": 1, "y": [1,2,3] }"#).unwrap();

    let attr_x = SerdeAttribute::new("x");
    let attr_y = SerdeAttribute::new("y");

    assert_eq!(attr_x.get(&obj), &Value::U64(1));
    assert_eq!(attr_y.at(&obj, 1), &Value::U64(2));
}

struct Foo {
    inner: Value
}

#[test]
fn test_combine() {
    use serde_json::value::Value;

    #[derive(Default)]
    struct Inner;

    #[derive(Default)]
    struct FooAttributes {
        inner: Inner,
    }

    impl Attributes<FooAttributes> for Foo {
        fn attrs() -> FooAttributes {
            FooAttributes::default()
        }
    }

    impl Attr<Foo> for Inner {
        type Output = Value;

        fn get<'a, >(&self, i: &'a Foo) -> &'a Value {
            &i.inner
        }

        fn name(&self) -> &'static str {
            "inner"
        }
    }

    let val: Value = json::from_str(r#"{"x": 1}"#).unwrap();
    let obj = Foo { inner: val };
    let attr = SerdeAttribute::new("x");

    let path = new_immutable_path(attr).prepend(Foo::attrs().inner);

    assert_eq!(path.traverse(&obj), &Value::U64(1));
}