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

pub mod foo {
    use attr::Attr;
    use attr::AttrMut;
    use attr::IndexableAttr;
    use attr::IndexableAttrMut;
    use attr::IterableAttr;
    use attr::IterableAttrMut;
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

    impl AttrMut<Foo> for Bar {
        fn get_mut<'a, >(&self, i: &'a mut Foo) -> &'a mut String {
            &mut i.bar
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

    impl AttrMut<Foo> for Vector {
        fn get_mut<'a, >(&self, i: &'a mut Foo) -> &'a mut Vec<Bla> {
            &mut i.vector
        }
    }

    impl<'a, 'b : 'a> IndexableAttr<'a, 'b, Foo, usize> for Vector {
        type Output = Bla;

        fn at(&self, i: &'b Foo, idx: usize) -> &'a Bla {
            &self.get(i)[idx]
        }
    }

    impl<'a, 'b : 'a> IndexableAttrMut<'a, 'b, Foo, usize> for Vector {
        fn at_mut(&self, i: &'b mut Foo, idx: usize) -> &'a mut Bla {
            &mut self.get_mut(i)[idx]
        }
    }

    impl<'a, 'b: 'a> IterableAttr<'a, 'b, Foo> for Vector {
        type Item = Bla;

        fn iter(&self, i: &'b Foo) -> Box<Iterator<Item=&'a Bla> + 'a> {
            Box::new(self.get(i).iter())
        }
    }

    impl<'a, 'b: 'a> IterableAttrMut<'a, 'b, Foo> for Vector {
        fn iter_mut(&self, i: &'b mut Foo) -> Box<Iterator<Item=&'a mut Bla> +'a> {
            Box::new(self.get_mut(i).iter_mut())
        }
    }
}


pub mod bla {
    use attr::Attr;
    use attr::AttrMut;
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

    impl AttrMut<Bla> for Name {
        fn get_mut<'a, >(&self, i: &'a mut Bla) -> &'a mut String {
            &mut i.name
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