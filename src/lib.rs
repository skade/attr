extern crate serde_json;

pub mod serde;

use std::marker::PhantomData;

pub trait Attr<Type: ?Sized> {
    type Output;

    fn name(&self) -> &str;
    fn get(&self, i: Type) -> Self::Output;
}

pub trait Attributes<AttributeType> {
    fn attrs() -> AttributeType;
}

pub trait IndexableAttr<Type: ?Sized, Idx: ?Sized> : Attr<Type> {
    type Output;

    fn at(&self, i: Type, idx: Idx) -> <Self as IndexableAttr<Type, Idx>>::Output;
}

pub trait IterableAttr<'a, Type: ?Sized> : Attr<Type> {
    type Item: 'a;

    fn iter(&self, i: Type) -> Box<Iterator<Item=Self::Item> + 'a>;
}

pub trait Traverse<'a, 'b: 'a, X: ?Sized + 'a, Y: ?Sized + 'a> {
    #[inline]
    fn traverse(&'b self, val: X) -> Y;
}

pub struct Identity;

pub struct Path<X, Z, A: Attr<X>, R> {
    attr: A,
    next: R,
    phantom_x: PhantomData<X>,
    phantom_z: PhantomData<Z>,
}

pub struct MapPath<A, R> {
    attr: A,
    next: R,
}

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

impl<'a, 'b: 'a, T: 'a> Traverse<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse(&'b self, val: T) -> T { val }
}

impl<'a, 'b: 'a, X: 'a, Z: 'a, A: Attr<X>, R: Traverse<'a, 'b, A::Output, Z>> Traverse<'a, 'b, X, Z> for Path<X, Z, A, R> where <A as Attr<X>>::Output: 'a {
    #[inline]
    fn traverse(&'b self, obj: X) -> Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

impl<'a, 'b: 'a, X: 'a, Z: 'a, A: IterableAttr<'a, X>, R: Traverse<'a, 'b, A::Item, Z>> Traverse<'a, 'b, X, Box<Iterator<Item=Z> + 'a>> for MapPath<A, R> {
    #[inline]
    fn traverse(&'b self, obj: X) -> Box<Iterator<Item=Z> + 'a> {
        let iter = self.attr.iter(obj);
        let next = &self.next;
        let map = iter.map(move |v| next.traverse(v) );
        Box::new(map)
    }
}

impl<'a, 'b: 'a, X: 'a, Z: 'a, A: Attr<X>, R: Traverse<'a, 'b, A::Output, Z>> Path<X, Z, A, R> where <A as Attr<X>>::Output: 'a {
    pub fn from<NX: 'a, NY: 'a, NZ: 'a, NA>(self, attr: NA) -> Path<NX, NZ, NA, Self>
        where A: Attr<NY, Output=Z>,
              NA: Attr<NX, Output=NY> {
        Path {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    pub fn mapped<NX: 'a, NY: 'a, NZ: 'a, NA>(self, attr: NA) -> MapPath<NA, Self>
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

impl<A, R> MapPath<A, R> {
    pub fn from<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, NX: 'b, NY: 'b, NA>(self, attr: NA) -> Path<NX, Box<std::iter::Iterator<Item=Z> + 'a>, NA, Self>
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
