use serde_json::value::Value;
use super::InsecureAttr;
use super::InsecureIndexableAttr;
use super::Result;

pub struct SerdeAttribute<'a> {
    name: &'a str
}

impl<'a> SerdeAttribute<'a> {
    pub fn new(name: &'a str) -> SerdeAttribute<'a> {
        SerdeAttribute { name: name }
    }
}

impl<'a, 'b: 'a> InsecureAttr<&'a Value> for SerdeAttribute<'b> {
    type Output = &'a Value;

    fn name(&self) -> &str {
        self.name
    }

    fn get(&self, i: &'a Value) -> Result<&'a Value> {
        match *i {
            Value::Object(ref m) => { m.get(self.name).ok_or_else(|| {format!("{} it empty or not present", self.name)}) },
            _ => Err(format!("{} is not an non-object", self.name))
        }
    }
}

impl<'a, 'b: 'a> InsecureAttr<&'a mut Value> for SerdeAttribute<'b> {
    type Output = &'a mut Value;

    fn name(&self) -> &str {
        self.name
    }

    fn get(&self, i: &'a mut Value) -> Result<&'a mut Value> {
        match *i {
            Value::Object(ref mut m) => { m.get_mut(self.name).ok_or_else(|| { format!("{} it empty or not present", self.name)}) },
            _ => Err(format!("{} is not an non-object", self.name))
        }
    }
}

impl<'a, 'b: 'a> InsecureIndexableAttr<&'a Value, usize> for SerdeAttribute<'b> {
    type Output = &'a Value;

    fn at(&self, i: &'a Value, idx: usize) -> Result<&'a Value> {
        let v = self.get(i);
        match v {
            Ok(&Value::Array(ref vec)) => { Ok(& vec[idx]) },
            _ => Err("Not an object or array".into())
        }
    }
}

impl<'a, 'b : 'a> InsecureIndexableAttr<&'a mut Value, usize> for SerdeAttribute<'a> {
    type Output = &'a mut Value;

    fn at(&self, i: &'a mut Value, idx: usize) -> Result<&'a mut Value> {
        let v = self.get(i);
        match v {
            Ok(&mut Value::Array(ref mut vec)) => { Ok(&mut vec[idx]) },
            _ => Err("Not an object or array".into())
        }
    }
}
