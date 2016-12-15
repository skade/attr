extern crate attr;

use attr::retrieve;
use attr::Traverse;

trait Attributes<AttributeType> {
    fn attrs() -> AttributeType;
}

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
    use super::Attributes;

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

    impl<'a> Attr<&'a Foo> for Bar {
        type Output = &'a str;

        fn get(&self, i: &'a Foo) -> Self::Output {
            i.bar.as_ref()
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl<'a> Attr<&'a Foo> for Vector {
        type Output = &'a [Bla];

        fn get(&self, i: &'a Foo) -> &'a [Bla] {
            i.vector.as_ref()
        }

        fn name(&self) -> &'static str {
            "vector"
        }
    }

    impl<'a> IndexableAttr<&'a Foo, usize> for Vector {
        type Output = &'a Bla;

        fn at(&self, i: &'a Foo, idx: usize) -> &'a Bla {
            unsafe { self.get(i).get_unchecked(idx) }
        }
    }

    impl<'a> IterableAttr<'a, &'a Foo> for Vector {
        type Item = &'a Bla;

        fn iter(&self, i: &'a Foo) -> Box<Iterator<Item=&'a Bla> + 'a> {
            Box::new(self.get(i).iter())
        }
    }
}


pub mod bla {
    use attr::Attr;
    use super::Attributes;

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

    impl<'a> Attr<&'a Bla> for Name {
        type Output = &'a str;

        fn get(&self, i: &'a Bla) -> &'a str {
            i.name.as_ref()
        }

        fn name(&self) -> &'static str {
            "name"
        }
    }
}

pub mod top {
    use attr::Attr;
    use super::Attributes;

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

    impl<'a> Attr<&'a Top> for FooField {
        type Output = &'a Foo;

        fn get(&self, i: &'a Top) -> &'a Foo {
            &i.foo
        }

        fn name(&self) -> &'static str {
            "foo"
        }
    }
}

#[test]
fn test_access() {
    let b1 = Bla { name: "foo".into() };
    let b2 = Bla { name: "bla".into() };

    let f = Foo { bar: "bar".into(), vector: vec![b1,b2] };
    let top = Top { foo: f };

    let path = retrieve(foo::Vector).from(top::FooField);

    assert_eq!(path.traverse(&top).unwrap().len(), 2);
}

#[test]
fn test_mapped() {
    let b1 = Bla { name: "foo".into() };
    let b2 = Bla { name: "bla".into() };

    let f = Foo { bar: "bar".into(), vector: vec![b1,b2] };
    let path = retrieve(bla::Name).mapped(foo::Vector);

    let result = path.traverse(&f).unwrap().map(std::result::Result::unwrap).collect::<Vec<_>>();
    assert_eq!(result, vec!["foo", "bla"]);
}

#[test]
fn test_complex_mapped() {
    let b1 = Bla { name: "foo".into() };
    let b2 = Bla { name: "bla".into() };

    let f = Foo { bar: "bar".into(), vector: vec![b1,b2] };
    let top = Top { foo: f };

    let path = retrieve(bla::Name).mapped(foo::Vector).from(top::FooField);

    let result = path.traverse(&top).unwrap().map(std::result::Result::unwrap).collect::<Vec<_>>();
    assert_eq!(result, vec!["foo", "bla"]);
}
