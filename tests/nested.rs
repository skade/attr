extern crate kv_access;

use kv_access::Combiner;
use kv_access::Combine;
use kv_access::MutCombine;
use kv_access::IndexableAttr;
use kv_access::IndexableAttrMut;

pub struct Foo {
    bar: String,
    batz: Bla,
    numbers: Vec<i32>
}

pub struct Bla {
    name: String
}

pub mod foo {
    use kv_access::Attr;
    use kv_access::AttrMut;

    use super::Foo;
    use super::Bla;

    pub struct Bar;
    pub struct Batz;
    pub struct Numbers;

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

    impl Attr<Foo> for Batz {
        type Output = Bla;

        fn get<'a, >(&self, i: &'a Foo) -> &'a Bla {
            &i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl AttrMut<Foo> for Batz {
        fn get_mut<'a, >(&self, i: &'a mut Foo) -> &'a mut Bla {
            &mut i.batz
        }
    }

    impl Attr<Foo> for Numbers {
        type Output = Vec<i32>;

        fn get<'a, >(&self, i: &'a Foo) -> &'a Vec<i32> {
            &i.numbers
        }

        fn name(&self) -> &'static str {
            "numbers"
        }
    }

    impl AttrMut<Foo> for Numbers {
        fn get_mut<'a, >(&self, i: &'a mut Foo) -> &'a mut Vec<i32> {
            &mut i.numbers
        }
    }
}

pub mod bla {
    use kv_access::Attr;
    use kv_access::AttrMut;

    use super::Bla;
    pub struct Name;

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
fn nested_access() {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    let c = Combiner::combine(foo::Batz, bla::Name);
    c.get(&f);
}

#[test]
fn nested_mutable() {
    let mut f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    let c = Combiner::combine(foo::Batz, bla::Name);
    {
        let x = c.get_mut(&mut f);
        *x = "bar".into();
    }

    let y = c.get(&f);
    assert_eq!(y, "bar");
}

#[test]
fn nested_vec() {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![1,2,3] };
    let x = foo::Numbers.at(&f, 1);

    assert_eq!(*x, 2)
}

#[test]
fn nested_vec_mutable() {
    let mut f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![1,2,3] };
    {
        let x = foo::Numbers.at_mut(&mut f, 1);
        *x = 4;
    }
    let y = foo::Numbers.at(&f, 1);
    assert_eq!(*y, 4)
}