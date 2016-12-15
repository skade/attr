extern crate attr;

use attr::*;

struct User {
    data: Data,
}

struct DataAttribute;

impl<'a> Attr<&'a User> for DataAttribute {
    type Output = &'a Data;

    fn name(&self) -> &'static str { "data" }
    fn get(&self, u: &'a User) -> &'a Data { &u.data }
}

struct Data {
    email: String
}

struct EmailAttribute;

impl<'a> Attr<&'a Data> for EmailAttribute {
    type Output = &'a str;

    fn name(&self) -> &'static str { "email" }
    fn get(&self, d: &'a Data) -> &'a str { &d.email }
}

struct PrefixValidator<P> {
    pattern: String,
    path: P
}

impl<P> PrefixValidator<P> {
    fn validate<'a, 'b: 'a, T: 'b>(&'a self, t: T) -> std::result::Result<(), String>
        where P: Traverse<'a, 'b, T, &'b str>
    {
        match self.path.traverse(t) {
            Ok(s) => {
                if s.starts_with(&self.pattern) {
                    Ok(())
                } else {
                    Err(format!("Does not start with {}", self.pattern))
                }
            }
            Err(reason) => Err(reason)
        }
    }
}

fn main() {
    let user = User { data: Data { email: "flo@andersground.net".into() }};
    assert!(validate(&user).is_ok());
}

fn validate(u: &User) -> std::result::Result<(), String> {
    let path = retrieve(EmailAttribute).from(DataAttribute);
    let validator = PrefixValidator { pattern: "flo".into(), path: path };

    validator.validate(u)
}
