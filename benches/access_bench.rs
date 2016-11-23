#![feature(test)]

extern crate attr;
extern crate test;

use test::{Bencher, black_box};

use attr::retrieve;
use attr::Traverse;

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

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Foo> for Bar {
        type Output = &'a str;

        fn get(&self, i: &'b Foo) -> &'a str {
            i.bar.as_ref()
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'a mut Foo> for Bar {
        type Output = &'a mut String;

        fn get(&self, i: &'a mut Foo) -> &'a mut String {
            &mut i.bar
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Foo> for Batz {
        type Output = &'a Bla;

        fn get(&self, i: &'b Foo) -> &'a Bla {
            &i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b mut Foo> for Batz {
        type Output = &'a mut Bla;

        fn get(&self, i: &'b mut Foo) -> &'a mut Bla {
            &mut i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Foo> for Numbers {
        type Output = &'a[i32];

        fn get(&self, i: &'b Foo) -> &'a[i32] {
            i.numbers.as_ref()
        }

        fn name(&self) -> &'static str {
            "numbers"
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b mut Foo> for Numbers {
        type Output = &'a mut Vec<i32>;

        fn get(&self, i: &'b mut Foo) -> &'a mut Vec<i32> {
            &mut i.numbers
        }

        fn name(&self) -> &'static str {
            "numbers"
        }
    }

    impl<'a, 'b : 'a> IndexableAttr<'a, 'b, &'b Foo, usize> for Numbers {
        type Output = i32;

        fn at(&self, i: &'b Foo, idx: usize) -> i32 {
            self.get(i)[idx]
        }
    }

    impl<'a, 'b : 'a> IndexableAttr<'a, 'b, &'b mut Foo, usize> for Numbers {
        type Output = &'a mut i32;

        fn at(&self, i: &'b mut Foo, idx: usize) -> &'a mut i32 {
            unsafe { self.get(i).get_unchecked_mut(idx) }
        }
    }

    impl<'a, 'b: 'a> IterableAttr<'a, 'b, &'b Foo> for Numbers {
        type Item = &'a i32;

        fn iter(&self, i: &'b Foo) -> Box<Iterator<Item=&'a i32> + 'a> {
            Box::new(self.get(i).iter())
        }
    }

    impl<'a, 'b: 'a> IterableAttr<'a, 'b, &'b mut Foo> for Numbers {
        type Item = &'a mut i32;

        fn iter(&self, i: &'b mut Foo) -> Box<Iterator<Item=&'a mut i32> +'a> {
            Box::new(self.get(i).iter_mut())
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

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Bla> for Name {
        type Output = &'a str;

        fn get(&self, i: &'a Bla) -> &'a str {
            i.name.as_ref()
        }

        fn name(&self) -> &'static str {
            "name"
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b mut Bla> for Name {
        type Output = &'a mut String;

        fn get(&self, i: &'a mut Bla) -> &'a mut String {
            &mut i.name
        }

        fn name(&self) -> &'static str {
            "name"
        }
    }
}

#[bench]
fn direct_access_bench(b: &mut Bencher) {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![1,2,3] };
    b.iter(|| black_box(direct_access(&f)) );
}

#[inline]
fn direct_access(f: &Foo) -> &str {
    &f.batz.name
}

#[bench]
fn through_path(b: &mut Bencher) {
    let f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![1,2,3] };
    let p = retrieve(bla::Name).from(foo::Batz);
    b.iter(|| black_box(path_access(&p, &f)) );
}

#[inline]
fn path_access<'a, 'b: 'a, P: Traverse<'a, 'b, &'b Foo, &'a str>>(p: &'b P, f: &'b Foo) -> &'a str {
    p.traverse(f)
}