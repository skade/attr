extern crate serde_json;

use std::marker::PhantomData;

pub trait Attr<'a, 'b: 'a, Type: 'a + ?Sized> {
    type Output: 'a + ?Sized;

    fn name(&self) -> &str;
    fn get(&self, i: Type) -> Self::Output;
}

pub trait Attributes<AttributeType> {
    fn attrs() -> AttributeType;
}

pub trait IndexableAttr<'a, 'b: 'a, Type: 'b + ?Sized, Idx: ?Sized> : Attr<'a, 'b, Type> {
    type Output: ?Sized;

    fn at(&self, i: Type, idx: Idx) -> <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait IterableAttr<'a, 'b: 'a, Type: 'b + ?Sized> : Attr<'a, 'b, Type> {
    type Item: ?Sized;

    fn iter(&self, i: Type) -> Box<Iterator<Item=Self::Item> + 'a>;
}

pub trait Traverse<'a, 'b: 'a, X: ?Sized + 'b, Y: ?Sized + 'a> {
    #[inline]
    fn traverse(&'b self, val: X) -> Y;
}

pub struct Identity;

pub struct Path<'a, 'b: 'a, X: 'b + ?Sized, Y: 'b + ?Sized, Z: 'a + ?Sized, A: Attr<'a, 'b, X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'b Y>,
    phantomz: PhantomData<&'a Z>
}

pub struct MapPath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'b + ?Sized, Z: 'a + ?Sized, A: IterableAttr<'a, 'b, X, Item=Y>, R: Traverse<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'b Y>,
    phantomz: PhantomData<&'a Z>
}

pub fn retrieve<'a, T, A: Attr<'a, 'a, T>>(attr: A) -> Path<'a, 'a, T, A::Output, A::Output, A, Identity>
        where Identity : Traverse<'a, 'a, A::Output, A::Output>
{
    Path {
        attr: attr,
        next: Identity,
        phantomx: PhantomData,
        phantomy: PhantomData,
        phantomz: PhantomData
    }
}

impl<'a, 'b: 'a, T: 'b> Traverse<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse(&'b self, val: T) -> T { val }
}

impl<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, A: Attr<'a, 'b, X, Output=Y>, R: Traverse<'a, 'b, Y, Z> + 'b> Traverse<'a, 'b, X, Z> for Path<'a, 'b, X, Y, Z, A, R> {
    #[inline]
    fn traverse(&'b self, obj: X) -> Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, A: IterableAttr<'a, 'b, X, Item=Y>, R: Traverse<'a, 'b, Y, Z>> Traverse<'a, 'b, X, Box<Iterator<Item=Z> + 'a>> for MapPath<'a, 'b, X, Y, Z, A, R> {
    #[inline]
    fn traverse(&'b self, obj: X) -> Box<Iterator<Item=Z> + 'a> {
        let iter = self.attr.iter(obj);
        let next = &self.next;
        let map = iter.map(move |v| next.traverse(v) );
        Box::new(map)
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: Attr<'a, 'b, X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> Path<'a, 'b, X, Y, Z, A, R> {
    pub fn from<NX, NY, NZ, NA>(self, attr: NA) -> Path<'a, 'b, NX, NY, NZ, NA, Self>
        where NA: Attr<'a, 'b, NX, Output=NY>,
              Self: Traverse<'a, 'b, NY, NZ> {
        Path {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }

    pub fn mapped<NX, NY, NZ, NA>(self, attr: NA) -> MapPath<'a, 'b, NX, NY, NZ, NA, Self>
        where NA: IterableAttr<'a, 'b, NX, Item=NY>,
              Self: Traverse<'a, 'b, NY, NZ> {
        MapPath {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: IterableAttr<'a, 'b, X, Item=Y>, R: Traverse<'a, 'b, Y, Z>> MapPath<'a, 'b, X, Y, Z, A, R> {
    pub fn from<NX, NY, NA>(self, attr: NA) -> Path<'a, 'b, NX, NY, Box<std::iter::Iterator<Item=Z> + 'a>, NA, Self>
        where NA: Attr<'a, 'b, NX, Output=NY>,
              Self: Traverse<'a, 'b, NY, Box<std::iter::Iterator<Item=Z> + 'a>>
    {
        Path {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}
