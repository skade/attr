extern crate attr;

use attr::retrieve;
use attr::IndexableAttr;
use attr::Traverse;

trait Attributes<AttributeType> {
    fn attrs() -> AttributeType;
}

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
    use attr::IndexableAttr;
    use attr::IterableAttr;
    use super::Attributes;

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

    impl<'a> Attr<&'a Foo> for Bar {
        type Output = &'a str;

        fn get(&self, i: &'a Foo) -> &'a str {
            i.bar.as_ref()
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl<'a> Attr<&'a mut Foo> for Bar {
        type Output = &'a mut String;

        fn get(&self, i: &'a mut Foo) -> &'a mut String {
            &mut i.bar
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl<'a> Attr<&'a Foo> for Batz {
        type Output = &'a Bla;

        fn get(&self, i: &'a Foo) -> &'a Bla {
            &i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl<'a> Attr<&'a mut Foo> for Batz {
        type Output = &'a mut Bla;

        fn get(&self, i: &'a mut Foo) -> &'a mut Bla {
            &mut i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl<'a> Attr<&'a Foo> for Numbers {
        type Output = &'a[i32];

        fn get(&self, i: &'a Foo) -> &'a[i32] {
            i.numbers.as_ref()
        }

        fn name(&self) -> &'static str {
            "numbers"
        }
    }

    impl<'a> Attr<&'a mut Foo> for Numbers {
        type Output = &'a mut Vec<i32>;

        fn get(&self, i: &'a mut Foo) -> &'a mut Vec<i32> {
            &mut i.numbers
        }

        fn name(&self) -> &'static str {
            "numbers"
        }
    }

    impl<'a> IndexableAttr<&'a Foo, usize> for Numbers {
        type Output = i32;

        fn at(&self, i: &'a Foo, idx: usize) -> i32 {
            self.get(i)[idx]
        }
    }

    impl<'a> IndexableAttr<&'a mut Foo, usize> for Numbers {
        type Output = &'a mut i32;

        fn at(&self, i: &'a mut Foo, idx: usize) -> &'a mut i32 {
            unsafe { self.get(i).get_unchecked_mut(idx) }
        }
    }

    impl<'a> IterableAttr<'a, &'a Foo> for Numbers {
        type Item = &'a i32;

        fn iter(&self, i: &'a Foo) -> Box<Iterator<Item=&'a i32> + 'a> {
            Box::new(self.get(i).iter())
        }
    }

    impl<'a> IterableAttr<'a, &'a mut Foo> for Numbers {
        type Item = &'a mut i32;

        fn iter(&self, i: &'a mut Foo) -> Box<Iterator<Item=&'a mut i32> +'a> {
            Box::new(self.get(i).iter_mut())
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
    pub struct NameMut;

    #[derive(Default)]
    pub struct BlaAttributes {
        pub name: Name,
        pub name_mut: NameMut,
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

    impl<'a> Attr<&'a mut Bla> for Name {
        type Output = &'a mut String;

        fn get(&self, i: &'a mut Bla) -> &'a mut String {
            &mut i.name
        }

        fn name(&self) -> &'static str {
            "name"
        }
    }
}

#[test]
fn nested_access() {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    let path = retrieve(Bla::attrs().name).from(Foo::attrs().batz);

    let val = path.traverse(&f).unwrap();
    assert_eq!(val, "foo");
}

#[test]
fn simple_mutable() {
    let mut f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    let path = retrieve(Foo::attrs().batz);
    assert_eq!(path.traverse(&mut f).unwrap().name, "foo");
}

#[test]
fn nested_mutable() {
    let mut f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    {
        let path = retrieve(Bla::attrs().name).from(Foo::attrs().batz);
        let x = path.traverse(&mut f).unwrap();
        *x = "bar".into();
    }
    {
        let path = retrieve(Bla::attrs().name).from(Foo::attrs().batz);

        let y = path.traverse(&f).unwrap();
        assert_eq!(y, "bar");
    }
}

#[test]
fn nested_vec() {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![1,2,3] };
    let x = foo::Numbers.at(&f, 1);

    assert_eq!(x, 2)
}

#[test]
fn nested_vec_mutable() {
    let mut f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![1,2,3] };
    {
        let mut x: &mut i32 = foo::Numbers.at(&mut f, 1);
        *x = 4;
    }
    let y = foo::Numbers.at(&f, 1);
    assert_eq!(y, 4)
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

    let filtered = vec.iter().filter(|f| path.traverse(*f).unwrap() == "foo" ).collect::<Vec<_>>();

    assert_eq!(filtered.len(), 1);
}
