#![deny(missing_docs)]

//! # attr - static paths for Rust

use std::marker::PhantomData;

/// In case of failed traversals, this Result type is
/// returned.
pub type Result<X> = std::result::Result<X, String>;

/// Direct access to an attribute of a type.
///
/// All attributes need to be named for debugging purposes.
pub trait Attr<Type: ?Sized> {
    /// The resulting value when accessing the attribute
    type Output;

    /// The attributes name
    fn name(&self) -> &str;
    /// Implementation of the retrieval
    fn get(&self, i: Type) -> Self::Output;
}

/// Direct, possibly failing access to an attribute of a type.
pub trait InsecureAttr<Type: ?Sized> {
    /// The resulting value when accessing the attribute
    type Output;

    /// The attributes name
    fn name(&self) -> &str;
    /// Implementation of the retrieval
    fn get(&self, i: Type) -> Result<Self::Output>;
}

/// Access to a part of the attribute by index
///
/// This can be used to provide access to parts of a vector
pub trait IndexableAttr<Type: ?Sized, Idx: ?Sized> : Attr<Type>{
    /// The resulting value when accessing the attribute
    type Output;

    /// Implementation of the indexing operation
    fn at(&self, i: Type, idx: Idx) -> <Self as IndexableAttr<Type, Idx>>::Output;
}

/// Access to a part of the attribute by index, where access can fail
///
/// This can be used to provide access to parts of a vector where the exact structure
/// of vector parts is unknown.
pub trait InsecureIndexableAttr<Type: ?Sized, Idx: ?Sized> : InsecureAttr<Type> {
    /// The resulting value when accessing the attribute
    type Output;

    /// Implementation of the indexing operation
    fn at(&self, i: Type, idx: Idx) -> Result<<Self as InsecureIndexableAttr<Type, Idx>>::Output>;
}

/// Iteration over an attribute
///
/// This allows to express path that branch out, for example at a vector.
pub trait IterableAttr<'a, Type: ?Sized> : Attr<Type> {
    /// The output item of the iteration
    type Item: 'a;

    /// Retrieval of an Iterator
    fn iter(&self, i: Type) -> Box<Iterator<Item=Self::Item> + 'a>;
}

/// Insecure variant of iteration over an attribute
///
/// This allows to express path that branch out, for example at a vector.
/// This operation may fail.
pub trait InsecureIterableAttr<'a, Type: ?Sized> : Attr<Type> {
    /// The output item of the iteration
    type Item: 'a;

    /// Retrieval of an Iterator
    fn iter(&self, i: Type) -> Result<Box<Iterator<Item=Self::Item> + 'a>>;
}

/// Recursive path traversal
///
/// This trait should rarely need to be implemented yourself,
/// but is needed to express bounds when accepting paths.
pub trait Traverse<'a, 'b: 'a, X: ?Sized + 'b, Y: ?Sized + 'b> {
    /// implementation of the traversal for a specific path
    #[inline]
    fn traverse(&'a self, val: X) -> Y;
}

/// The Identity is the end of a path and provides the point where
/// input equals output and we start returning.
/// It's necessary for recursive path traversal, but generally not
/// to be used in user code.
pub struct Identity;

/// A plain path describing how to retrieve a value at a point,
/// and then recursive down the rest of the path.
///
/// Paths are usually inferred and should not be directly used
/// in user code.
pub struct Path<Input, Output, A: Attr<Input>, Rest> {
    attr: A,
    next: Rest,
    phantom_x: PhantomData<Input>,
    phantom_z: PhantomData<Output>,
}

/// A path path describing how to retrieve a value at a point,
/// and then recursive down the rest of the path.
///
/// For InsecurePath, the retrieval operation could fail!
///
/// Paths are usually inferred and should not be directly used
/// in user code.
pub struct InsecurePath<Input, Output, A: InsecureAttr<Input>, Rest> {
    attr: A,
    next: Rest,
    phantom_x: PhantomData<Input>,
    phantom_z: PhantomData<Output>,
}

/// A path that describes a mapping operation, which later application
/// of a subpath.
///
/// Paths are usually inferred and should not be directly used
/// in user code.
pub struct MapPath<A, R> {
    attr: A,
    next: R,
}

/// `retrieve` is the starting point of a path that always
/// returns a value.
///
/// Note that path creation starts inside out, this
/// needs to be called with the innermost attribute.
pub fn retrieve<X, Z, A>(attr: A) -> Path<X, Z, A, Identity>
    where A: Attr<X>
{
    Path {
        attr: attr,
        next: Identity,
        phantom_x: PhantomData,
        phantom_z: PhantomData,
    }
}

/// `retrieve_insecure` is the starting point of a path that is
/// not always present. For that reason, it returns a Result
/// indicating the success of the path operation.
pub fn retrieve_insecure<X, Z, A>(attr: A) -> InsecurePath<X, Z, A, Identity>
    where A: InsecureAttr<X>
{
    InsecurePath {
        attr: attr,
        next: Identity,
        phantom_x: PhantomData,
        phantom_z: PhantomData,
    }
}

impl<'a, 'b: 'a, T: 'b> Traverse<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse(&'a self, val: T) -> T { val }
}

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: Attr<X>, R: Traverse<'a, 'b, A::Output, Z>> Traverse<'a, 'b, X, Z> for Path<X, Z, A, R> where <A as Attr<X>>::Output: 'b {
    #[inline]
    fn traverse(&'a self, obj: X) -> Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: InsecureAttr<X>, R: Traverse<'a, 'b, A::Output, Z>> Traverse<'a, 'b, X, Result<Z>> for InsecurePath<X, Z, A, R> where <A as InsecureAttr<X>>::Output: 'b {
    #[inline]
    fn traverse(&'a self, obj: X) -> Result<Z> {
        let val = self.attr.get(obj);
        match val {
            Ok(v) => Ok(self.next.traverse(v)),
            Err(_) => Err("Something went wrong".into())
        }
    }
}

impl<'a, X: 'a, Z: 'a, A: IterableAttr<'a, X>, R: Traverse<'a, 'a, A::Item, Z>> Traverse<'a, 'a, X, Box<Iterator<Item=Z> + 'a>> for MapPath<A, R> {
    #[inline]
    fn traverse(&'a self, obj: X) -> Box<Iterator<Item=Z> + 'a> {
        let iter = self.attr.iter(obj);
        let next = &self.next;
        let map = iter.map(move |v| next.traverse(v) );
        Box::new(map)
    }
}

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: Attr<X>, R: Traverse<'a, 'b, A::Output, Z>> Path<X, Z, A, R> where <A as Attr<X>>::Output: 'b {
    /// Extends a path by another segment.
    ///
    /// This needs a retrieval that always succeds
    pub fn from<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> Path<NX, NZ, NA, Self>
        where A: Attr<NY, Output=Z>,
              NA: Attr<NX, Output=NY> {
        Path {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    /// Extends a path by another segment.
    ///
    /// This assumes that the retrieval cannot always succeed.
    pub fn try<NX: 'a, NY: 'a, NZ: 'a, NA>(self, attr: NA) -> InsecurePath<NX, NZ, NA, Self>
        where A: Attr<NY, Output=Z>,
              NA: InsecureAttr<NX, Output=NY> {
        InsecurePath {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    /// Extends a path by an iteration operation.
    ///
    /// This assumes that the iteration is always possible
    pub fn mapped<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> MapPath<NA, Self>
        where A: Attr<X>,
              R: Traverse<'a, 'b, A::Output, Z>,
              NA: IterableAttr<'a, NX, Item=NY>,
              Self: Traverse<'a, 'b, NY, NZ> {
        MapPath {
            attr: attr,
            next: self,
        }
    }
}

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: InsecureAttr<X>, R: Traverse<'a, 'b, A::Output, Z>> InsecurePath<X, Z, A, R> where <A as InsecureAttr<X>>::Output: 'b {
    /// Extends a path that may fail by another segment that always succeeds.
    pub fn from<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> Path<NX, NZ, NA, Self>
        where A: InsecureAttr<NY, Output=Z>,
              NA: Attr<NX, Output=NY> {
        Path {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    /// Extends a path that may fail by another segment that may fail.
    pub fn try<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> InsecurePath<NX, NZ, NA, Self>
        where A: InsecureAttr<NY, Output=Z>,
              NA: InsecureAttr<NX, Output=NY> {
        InsecurePath {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    /// Extends a path that may fail by another segment that may fail.
    pub fn mapped<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> MapPath<NA, Self>
        where A: InsecureAttr<X>,
              R: Traverse<'a, 'b, A::Output, Z>,
              NA: IterableAttr<'a, NX, Item=NY>,
              Self: Traverse<'a, 'b, NY, NZ> {
        MapPath {
            attr: attr,
            next: self,
        }
    }
}

impl<A, R> MapPath<A, R> {
    /// Extends a mapped path by another segment that always succeeds
    pub fn from<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'b, NX: 'b, NY: 'b, NA>(self, attr: NA) -> Path<NX, Box<std::iter::Iterator<Item=Z> + 'a>, NA, Self>
        where A: IterableAttr<'a, X, Item=Y>,
              R: Traverse<'a, 'b, Y, Z>,
              NA: Attr<NX, Output=NY>,
              Self: Traverse<'a, 'b, NY, Box<std::iter::Iterator<Item=Z>>>
    {
        Path {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }
}
