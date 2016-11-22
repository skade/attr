extern crate serde_json;

//pub mod serde_impl;
pub mod mutable;

use std::marker::PhantomData;

pub trait Attr<Type: ?Sized> {
    type Output: ?Sized;

    fn name(&self) -> &str;
    fn get<'a>(&self, i: &'a Type) -> &'a Self::Output;
}

pub trait Attributes<AttributeType> {
    fn attrs() -> AttributeType;
}

pub trait IndexableAttr<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> : Attr<Type> {
    type Output: ?Sized;

    fn at(&self, i: &'b Type, idx: Idx) -> &'a <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait IterableAttr<'a, 'b: 'a, Type: ?Sized> : Attr<Type> {
    type Item: ?Sized;

    fn iter(&self, i: &'b Type) -> Box<Iterator<Item=&'a Self::Item> + 'a>;
}

pub struct ImmutablePath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, &'a Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

pub struct ImmutableMapPath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: IterableAttr<'a, 'b, X, Item=Y>, R: Traverse<'a, 'b, Y, &'a Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

pub fn retrieve<'a, 'b, T, A: Attr<T>>(attr: A) -> ImmutablePath<'a, 'b, T, <A as Attr<T>>::Output, <A as Attr<T>>::Output, A, Identity>
        where Identity : Traverse<'a, 'b, <A as Attr<T>>::Output, &'a <A as Attr<T>>::Output>
{
    ImmutablePath {
        attr: attr,
        next: Identity,
        phantomx: PhantomData,
        phantomy: PhantomData,
        phantomz: PhantomData
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, &'a Z>> ImmutablePath<'a, 'b, X, Y, Z, A, R> {
    pub fn from<NX, NY, NZ, NA>(self, attr: NA) -> ImmutablePath<'a, 'b, NX, NY, NZ, NA, Self>
        where NA: Attr<NX, Output=NY>,
              Self: Traverse<'a, 'b, NY, &'a NZ> {
        ImmutablePath {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }

    pub fn mapped<NX, NY, NZ, NA>(self, attr: NA) -> ImmutableMapPath<'a, 'b, NX, NY, NZ, NA, Self>
        where NA: IterableAttr<'a, 'b, NX, Item=NY>,
              Self: Traverse<'a, 'b, NY, &'a NZ> {
        ImmutableMapPath {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: IterableAttr<'a, 'b, X, Item=Y>, R: Traverse<'a, 'b, Y, &'a Z>> ImmutableMapPath<'a, 'b, X, Y, Z, A, R> {
    pub fn from<NX, NY, NA>(self, attr: NA) -> ImmutablePath<'a, 'b, NX, NY, Box<std::iter::Iterator<Item=&'a Z> + 'a>, NA, Self>
        where NA: Attr<NX, Output=NY>,
              Self: Traverse<'a, 'b, NY, &'a Box<std::iter::Iterator<Item=&'a Z> + 'a>>
    {
        ImmutablePath {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}

pub struct Identity;

pub trait Traverse<'a, 'b: 'a, X: ?Sized, Y: ?Sized> {
    fn traverse(&'b self, val: &'b X) -> Y;
}

impl<'a, 'b: 'a, T> Traverse<'a, 'b, T, &'a T> for Identity {
    #[inline]
    fn traverse(&'b self, val: &'b T) -> &'a T { val }
}

impl<'a, 'b: 'a, X, Y, Z, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, &'a Z> + 'b> Traverse<'a, 'b, X, &'a Z> for ImmutablePath<'a, 'b, X, Y, Z, A, R> {
    fn traverse(&'b self, obj: &'b X) -> &'a Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

impl<'a, 'b: 'a, X, Y, Z, A: IterableAttr<'a, 'b, X, Item=Y>, R: Traverse<'a, 'b, Y, &'a Z>> Traverse<'a, 'b, X, Box<Iterator<Item=&'a Z> + 'a>> for ImmutableMapPath<'a, 'b, X, Y, Z, A, R> {
    fn traverse(&'b self, obj: &'b X) -> Box<Iterator<Item=&'a Z> + 'a> {
        let iter = self.attr.iter(obj);
        let next = &self.next;
        let map = iter.map(move |v| next.traverse(v) );
        Box::new(map)
    }
}
