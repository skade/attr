extern crate kv_access;

use kv_access::Attr;
use kv_access::AttrMut;
use kv_access::Attributes;

pub struct Foo {
    bar: String,
    batz: i32
}

#[test]
fn simple_access() {
    #[derive(Default)]
    struct Bar;
    #[derive(Default)]
    struct Batz;

    #[derive(Default)]
    struct FooAttributes {
        bar: Bar,
        batz: Batz
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
        type Output = i32;

        fn get<'a, >(&self, i: &'a Foo) -> &'a i32 {
            &i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl AttrMut<Foo> for Batz {
        fn get_mut<'a, >(&self, i: &'a mut Foo) -> &'a mut i32 {
            &mut i.batz
        }
    }

    let f = Foo { bar: "foobar".into(), batz: 20 };
    Foo::attrs().bar.get(&f);
    Foo::attrs().batz.get(&f);

}