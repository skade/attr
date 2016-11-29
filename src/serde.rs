use serde_json::value::Value;
use super::Attr;
use super::IndexableAttr;

pub struct SerdeAttribute<'a> {
    name: &'a str
}

impl<'a> SerdeAttribute<'a> {
    pub fn new(name: &'a str) -> SerdeAttribute<'a> {
        SerdeAttribute { name: name }
    }
}

impl<'a, 'b: 'a> Attr<&'a Value> for SerdeAttribute<'b> {
    type Output = &'a Value;

    fn name(&self) -> &str {
        self.name
    }

    fn get(&self, i: &'a Value) -> &'a Value {
        match i {
            &Value::Object(ref m) => { m.get(self.name).unwrap() },
            _ => panic!("get on a non-object")
        }
    }
}

impl<'a, 'b: 'a> Attr<&'a mut Value> for SerdeAttribute<'b> {
    type Output = &'a mut Value;

    fn name(&self) -> &str {
        self.name
    }

    fn get(&self, i: &'a mut Value) -> &'a mut Value {
        match i {
            &mut Value::Object(ref mut m) => { m.get_mut(self.name).unwrap() },
            _ => panic!("get on a non-object")
        }
    }
}

impl<'a, 'b: 'a> IndexableAttr<&'a Value, usize> for SerdeAttribute<'b> {
    type Output = &'a Value;

    fn at(&self, i: &'a Value, idx: usize) -> &'a Value {
        let v = self.get(i);
        match v {
            &Value::Array(ref vec) => { & vec[idx] },
            _ => panic!("at on a non-array")
        }
    }
}

impl<'a, 'b : 'a> IndexableAttr<&'a mut Value, usize> for SerdeAttribute<'a> {
    type Output = &'a mut Value;

    fn at(&self, i: &'a mut Value, idx: usize) -> &'a mut Value {
        let v = self.get(i);
        match v {
            &mut Value::Array(ref mut vec) => { &mut vec[idx] },
            _ => panic!("at on a non-array")
        }
    }
}
