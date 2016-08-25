use serde_json::value::Value;
use super::Attr;
use super::AttrMut;
use super::IndexableAttr;
use super::IndexableAttrMut;

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

impl<'a, 'b : 'a> IndexableAttr<'a, 'b, Value, usize> for SerdeAttribute<'a> {
    type Output = Value;

    fn at(&self, i: &'b Value, idx: usize) -> &'a Value {
        let v = self.get(i);
        match v {
            &Value::Array(ref vec) => { & vec[idx] },
            _ => panic!("at on a non-array")
        }
    }
}

impl<'a, 'b : 'a> IndexableAttrMut<'a, 'b, Value, usize> for SerdeAttribute<'a> {
    type Output = Value;

    fn at_mut(&self, i: &'b mut Value, idx: usize) -> &'a mut Value {
        let v = self.get_mut(i);
        match v {
            &mut Value::Array(ref mut vec) => { &mut vec[idx] },
            _ => panic!("at on a non-array")
        }
    }
}
