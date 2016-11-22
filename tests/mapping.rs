extern crate attr;

use attr::retrieve;
use attr::Traverse;

#[derive(Debug)]
pub struct Foo {
    bar: String,
    vector: Vec<Bla>
}

#[derive(Debug)]
pub struct Bla {
    name: String
}

#[derive(Debug)]
pub struct Top {
    foo: Foo
}


pub mod foo {
    use attr::Attr;
    use attr::IndexableAttr;
    use attr::IterableAttr;
    use attr::Attributes;

    use super::Foo;
    use super::Bla;

    #[derive(Default)]
    pub struct Bar;
    #[derive(Default)]
    pub struct Vector;

    #[derive(Default)]
    pub struct FooAttributes {
        pub bar: Bar,
        pub numbers: Vector
    }

    impl Attributes<FooAttributes> for Foo {
        fn attrs() -> FooAttributes {
            FooAttributes::default()
        }
    }

    impl Attr<Foo> for Bar {
        type Output = String;

        fn get<'a, >(&self, i: &'a Foo) -> &'a String {
            &i.bar
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl Attr<Foo> for Vector {
        type Output = Vec<Bla>;

        fn get<'a, >(&self, i: &'a Foo) -> &'a Vec<Bla> {
            &i.vector
        }

        fn name(&self) -> &'static str {
            "vector"
        }
    }

    impl<'a, 'b : 'a> IndexableAttr<'a, 'b, Foo, usize> for Vector {
        type Output = Bla;

        fn at(&self, i: &'b Foo, idx: usize) -> &'a Bla {
            &self.get(i)[idx]
        }
    }

    impl<'a, 'b: 'a> IterableAttr<'a, 'b, Foo> for Vector {
        type Item = Bla;

        fn iter(&self, i: &'b Foo) -> Box<Iterator<Item=&'a Bla> + 'a> {
            Box::new(self.get(i).iter())
        }
    }
}


pub mod bla {
    use attr::Attr;
    use attr::Attributes;

    use super::Bla;

    #[derive(Default)]
    pub struct Name;

    #[derive(Default)]
    pub struct BlaAttributes {
        pub name: Name,
    }

    impl Attributes<BlaAttributes> for Bla {
        fn attrs() -> BlaAttributes {
            BlaAttributes::default()
        }
    }

    impl Attr<Bla> for Name {
        type Output = String;

        fn get<'a, >(&self, i: &'a Bla) -> &'a String {
            &i.name
        }

        fn name(&self) -> &'static str {
            "name"
        }
    }
}

pub mod top {
    use attr::Attr;
    use attr::Attributes;

    use super::Top;
    use super::Foo;

    #[derive(Default)]
    pub struct FooField;

    #[derive(Default)]
    pub struct TopAttributes {
        pub foo: FooField,
    }

    impl Attributes<TopAttributes> for Top {
        fn attrs() -> TopAttributes {
            TopAttributes::default()
        }
    }

    impl Attr<Top> for FooField {
        type Output = Foo;

        fn get<'a, >(&self, i: &'a Top) -> &'a Foo {
            &i.foo
        }

        fn name(&self) -> &'static str {
            "foo"
        }
    }
}

#[test]
fn test_mapped() {
    let b1 = Bla { name: "foo".into() };
    let b2 = Bla { name: "bla".into() };

    let foo = Foo { bar: "bar".into(), vector: vec![b1,b2] };
    let path = retrieve(bla::Name).mapped(foo::Vector);

    let result = path.traverse(&foo).collect::<Vec<_>>();
    assert_eq!(result, vec!["foo", "bla"]);
}


#[test]
fn test_complex_mapped() {
    let b1 = Bla { name: "foo".into() };
    let b2 = Bla { name: "bla".into() };

    let foo = Foo { bar: "bar".into(), vector: vec![b1,b2] };
    let top = Top { foo: foo };

    let path = retrieve(bla::Name).mapped(foo::Vector).from(top::FooField);

    let result = path.traverse(&top).collect::<Vec<_>>();
    assert_eq!(result, vec!["foo", "bla"]);
}
