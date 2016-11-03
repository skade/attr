extern crate attr;

use attr::retrieve;
use attr::retrieve_mut;
use attr::IndexableAttr;
use attr::IndexableAttrMut;
use attr::Traverse;
use attr::TraverseMut;
use attr::Attributes;

#[derive(Debug)]
pub struct Foo {
    bar: String,
    batz: Bla,
    numbers: Vec<i32>
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
    pub struct Batz;
    #[derive(Default)]
    pub struct Numbers;

    #[derive(Default)]
    pub struct FooAttributes {
        pub bar: Bar,
        pub batz: Batz,
        pub numbers: Numbers
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

    impl<'a, 'b : 'a> IndexableAttr<'a, 'b, Foo, usize> for Numbers {
        type Output = i32;

        fn at(&self, i: &'b Foo, idx: usize) -> &'a i32 {
            &self.get(i)[idx]
        }
    }

    impl<'a, 'b : 'a> IndexableAttrMut<'a, 'b, Foo, usize> for Numbers {
        fn at_mut(&self, i: &'b mut Foo, idx: usize) -> &'a mut i32 {
            &mut self.get_mut(i)[idx]
        }
    }

    impl<'a, 'b: 'a> IterableAttr<'a, 'b, Foo> for Numbers {
        type Item = i32;

        fn iter(&self, i: &'b Foo) -> Box<Iterator<Item=&'a i32> + 'a> {
            Box::new(self.get(i).iter())
        }
    }

    impl<'a, 'b: 'a> IterableAttrMut<'a, 'b, Foo> for Numbers {
        fn iter_mut(&self, i: &'b mut Foo) -> Box<Iterator<Item=&'a mut i32> +'a> {
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
fn nested_access() {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    let path = retrieve(Bla::attrs().name).from(Foo::attrs().batz);
//    let mut path = new_path(bla::Name).prepend(foo::Bla); <-- this fails and should be made a compile-test \o/

    let val = path.traverse(&f);
    assert_eq!(val, "foo");
}

#[test]
fn nested_mutable() {
    let mut f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    let path = retrieve_mut(Bla::attrs().name).from(Foo::attrs().batz);

    {
        let x = path.traverse_mut(&mut f);
        *x = "bar".into();
    }
    let path = retrieve(Bla::attrs().name).from(Foo::attrs().batz);

    let y = path.traverse(&f);
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

fn size_of<T>(_t: &T) -> usize {
    std::mem::size_of::<T>()
}

#[test]
fn nested_filter() {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![1,2,3] };
    let f2 = Foo { bar: "foobar".into(), batz: Bla { name: "bar".into() }, numbers: vec![1,2,3] };

    let vec = vec![f, f2];
    let path = retrieve(Bla::attrs().name).from(Foo::attrs().batz);

    assert_eq!(size_of(&path),0);

    let filtered = vec.iter().filter(|foo| path.traverse(&foo) == "foo" ).collect::<Vec<_>>();

    assert_eq!(filtered.len(), 1);
}