use serde_json::value::Value;
use super::Attr;
use super::AttrMut;

pub struct SerdeAttribute<'a> {
    name: &'a str
}

impl<'a> SerdeAttribute<'a> {
    pub fn new(name: &'a str) -> SerdeAttribute<'a> {
        SerdeAttribute { name: name }
    }
}

impl<'a> Attr<Value> for SerdeAttribute<'a> {
    type Output = Value;

    fn name(&self) -> &str {
        self.name
    }

    fn get<'b>(&self, i: &'b Value) -> &'b Value {
        match i {
            &Value::Object(ref m) => { m.get(self.name).unwrap() },
            _ => panic!("get on a non-object")
        }
    }
}

impl<'a> AttrMut<Value> for SerdeAttribute<'a> {
    fn get_mut<'b>(&self, i: &'b mut Value) -> &'b mut Value {
        match i {
            &mut Value::Object(ref mut m) => { m.get_mut(self.name).unwrap() },
            _ => panic!("get on a non-object")
        }
    }
}

//impl IndexableAttr<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> for SerdeAttribute<'a> {
//    type Output: ?Sized;
//
//    fn at(&self, i: &'b Type, idx: Idx) -> &'a <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
//}
