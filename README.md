# attr

`attr` is a Library to provide external access to a datastructure through a
typed path object, using all type information known about the data
structure at hand.

This allows expressing queries on data structures not currently at hand without
retrieving a handle on the data structure itself.

It is currently in experimentation phase, but runs on stable and nightly.

The current goal is to make the API convenient and uniform and the definition
of all user-facing types easy to do by hand. Definitions are still rather
verbose, but convenience macros and compiler plugins are currently not in scope
(but maybe some soul more interested in those will provide one? ;) )

As an example, an interface to the serde_json library is provided.

## Example

```rust
fn edit_name() {
    let mut f = Foo { bar: "foobar".into(), batz: Bla { name: "foo".into() }, numbers: vec![] };

    let path = retrieve_mut(bla::Name).from(foo::Batz);

    {
        let x = path.traverse_mut(&mut f);
        *x = "bar".into();
    }
    let path = retrieve(bla::Name).from(foo::Batz);

    let y = path.traverse(&f);
    assert_eq!(y, "bar");
}
```

The construction of the following path would fail with a type error:

```rust
retrieve_mut(foo::Batz).from(bla::Name);
```

## Underlying definitions

The library works by defining an accessor struct implementing the `Attr<Type>`
trait per access strategy. In this case, one per for every known field of the
data structure. Defining those is currently busywork. In the case of Foo and Bar,
the definition looks like follows. Field accessors get grouped into a seperate struct for convenience.

```rust
pub mod foo {
    use attr::Attr;
    use attr::AttrMut;
    use attr::IndexableAttr;
    use attr::IndexableAttrMut;
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
      type Output = i32;

      fn at_mut(&self, i: &'b mut Foo, idx: usize) -> &'a mut i32 {
          &mut self.get_mut(i)[idx]
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
```

## Currently open things

* Unify mutable and immutable retrieval, if possible
* Unify the retrieval interface between fields and paths, if possible
* Make all parts return Results, so that walking may fail
* More fancy path combinators!

## Acknowledgements

* The non-uniform List used to construct paths is adapted from [Link](src.codes) and based on a pattern devised by Tomaka.

## Bitten tongues

Nearly called that library lazr-pointer, because it is inteded to be used in the
laze.rs project.

## LICENSE

Currently none, coming.
