# attr

`attr` is a library to provide external access to a datastructure through a
typed path object, using all type information known about the data
structure at hand.

This allows expressing queries on data structures not currently at hand without
retrieving a handle on the data structure itself.

`attr` decouples access from data, by giving you ways to describe paths through
data without exporting the exact structure of the data.

For that, it describes traversible paths that work on given types and return
specific values. `attr` makes creation of these paths painless and supported
by the given type information.

The library is "pay what you use", simple pointer dereferencing paths cost
as much as hand-written, direct code.

As an example, an interface to the serde_json library is provided.

## Motivating Example

Let's consider a generic validator: instead of being generic over the types it validates, it would be convenient to make it generic over the way it retrieves the data to validate.

This has multiple advantages:
* The validator implementation does not need to know the types it validates
* It avoids the orphan rule: the access operation is always custom and can be local
* If the structure of the type under validation changes, the validator implementation does not need to be changed

```rust
struct PrefixValidator<P> {
    pattern: String,
    path: P
}

impl<P> PrefixValidator<P> {
    fn validate<'a, 'b: 'a, T: 'b>(&'a self, t: T) -> bool
        where P: Traverse<'a, 'b, T, &'b str>
    {
        self.path.traverse(t).starts_with(&self.pattern)
    }
}

fn main() {
    let user = User { data: Data { email: "flo@andersground.net".into() }};
    assert!(validate(&user));
}

fn validate(u: &User) -> bool {
    let path = retrieve(EmailAttribute).from(DataAttribute);
    let validator = PrefixValidator { pattern: "flo".into(), path: path };

    validator.validate(u)
}
```

## Underlying definitions

See `examples/validation.rs` for the full example.

## Attributes

The library works by defining an accessor struct implementing the `Attr<Type>`
trait per access strategy. In this case, one per for every known field of the
data structure.

```rust
extern crate attr;

use attr::*;

struct Data {
    email: String
}

struct User {
    data: Data,
}

struct DataAttribute;

impl<'a> Attr<&'a User> for DataAttribute {
    type Output = &'a Data;

    fn name(&self) -> &'static str { "data" }
    fn get(&self, u: &'a User) -> &'a Data { &u.data }
}

struct EmailAttribute;

impl<'a> Attr<&'a Data> for EmailAttribute {
    type Output = &'a str;

    fn name(&self) -> &'static str { "email" }
    fn get(&self, d: &'a Data) -> &'a str { &d.email }
}
```

Attributes are ways to retrieve a piece of a structure. Note that while this is generally a reference, it doesn't have to be.

All attributes need a name for debugging and diagnostics purposes.

Every attribute is bound to a specific type, for example, this attribute only retrieves data from the `User` type.

Attributes externalise data access by moving it into a seperate type. This allows us to combine them. Attribute types are zero-sized unless needed, and have no runtime cost.

## Paths

Attributes can be combined into _access paths_, allowing to express complex access strategies in a safe manner. Paths are initially constructed through the `retrieve` function, and then chained with additional operations. Path construction happens _inside out_, which makes inference easy.

```rust
let path = retrieve(EmailAttribute).from(DataAttribute)
```

This constructs a path that, on use, will retrieve the `data` field from a `User` using `DataAttribute` and then the `email` field from the resulting `Data` using `EmailAttribute` and return the result.

Access happens by calling the `traverse` method of the path with the object to work on:

```
let email = path.traverse(&user);
```

Paths have the combined size of all attributes they hold. This means that replacing standard pointer access through access with a path does not incur a runtime cost.

# Additional access strategies

Currently, this library also provides `IndexableAttr`, for attributes that allow indexed access (such as a vector) and `IterableAttr`, for attributes that can be iterated through (such as vectors, again).

Also, it ships with additional attribute types called `Insecure*` for expressing attributes where retrieval may fail (e.g. for access of maps). They return Results instead of plain values.

# Further reading

To see a sketch implementation of attributes working on serde_json, refer to the test suite.

## Currently open things

* Unify the retrieval interface between attributes and paths, if possible
* Find a good interface for returning results from both secure and insecure paths

## Acknowledgements

* The non-uniform List used to construct paths is adapted from [Typed Linked Lists](http://src.codes/typed-linked-lists.html) and based on a pattern devised by Tomaka.

## Bitten tongues

Nearly called that library lazr-pointer, because it is intended to be used in the
laze.rs project.

## LICENSE

MIT
