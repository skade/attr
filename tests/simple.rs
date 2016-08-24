extern crate kv_access;

use kv_access::Attr;
use kv_access::AttrMut;
use kv_access::Attributes;

pub struct Foo {
    bar: String,
    batz: i32
}

pub struct Bar {
    batz: String,
}

#[test]
fn simple_access() {
    #[derive(Default)]
    struct FooAttributeBar;
    #[derive(Default)]
    struct FooAttributeBatz;

    #[derive(Default)]
    struct FooAttributes {
        bar: FooAttributeBar,
        batz: FooAttributeBatz
    }

    impl Attributes<FooAttributes> for Foo {
        fn attrs() -> FooAttributes {
            FooAttributes::default()
        }
    }

    impl Attr<Foo> for FooAttributeBar {
        type Output = String;

        fn get<'a, >(&self, i: &'a Foo) -> &'a Self::Output {
            &i.bar
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl AttrMut<Foo> for FooAttributeBar {
        fn get_mut<'a, >(&self, i: &'a mut Foo) -> &'a mut Self::Output {
            &mut i.bar
        }
    }

    impl Attr<Foo> for FooAttributeBatz {
        type Output = i32;

        fn get<'a, >(&self, i: &'a Foo) -> &'a Self::Output {
            &i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl AttrMut<Foo> for FooAttributeBatz {
        fn get_mut<'a, >(&self, i: &'a mut Foo) -> &'a mut Self::Output {
            &mut i.batz
        }
    }

    #[derive(Default)]
    struct BarAttributeBatz;

    #[derive(Default)]
    struct BarAttributes {
        batz: BarAttributeBatz,
    }

    impl Attributes<BarAttributes> for Bar {
        fn attrs() -> BarAttributes {
            BarAttributes::default()
        }
    }

    impl Attr<Bar> for BarAttributeBatz {
        type Output = String;

        fn get<'a, >(&self, i: &'a Bar) -> &'a Self::Output {
            &i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    let f = Foo { bar: "foobar".into(), batz: 20 };
    Foo::attrs().bar.get(&f);
    Foo::attrs().batz.get(&f);

    let f = Bar { batz: "foobar".into() };
    Bar::attrs().batz.get(&f);

}
