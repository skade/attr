extern crate serde_json;

pub mod serde_impl;

use std::marker::PhantomData;

pub enum PathWalkError {
    PathCut(String)
}

pub type PathWalkResult<T> = Result<T, PathWalkError>;

pub trait Attr<Type: ?Sized> {
    type Output: ?Sized;

    fn name(&self) -> &str;
    fn get<'a>(&self, i: &'a Type) -> &'a Self::Output;
}

pub trait Attributes<AttributeType> {
    fn attrs() -> AttributeType;
}

pub trait AttrMut<Type: ?Sized> : Attr<Type> {
    fn get_mut<'a>(&self, i: &'a mut Type) -> &'a mut Self::Output;
}

pub trait IndexableAttr<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> : Attr<Type> {
    type Output: ?Sized;

    fn at(&self, i: &'b Type, idx: Idx) -> &'a <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait IndexableAttrMut<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> : IndexableAttr<'a, 'b, Type, Idx> + AttrMut<Type> {
    fn at_mut(&self, i: &'b mut Type, idx: Idx) -> &'a mut <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait IterableAttr<Type: ?Sized> : Attr<Type> {
    type Item: ?Sized;

    fn iter(&self, i: &Type) -> Box<Iterator<Item=&Self::Item>>;
}

pub trait IterableAttrMut<Type: ?Sized> : IterableAttr<Type> + AttrMut<Type> {
    fn iter_mut(&self, i: &mut Type) -> Box<Iterator<Item=&mut Self::Item>>;
}

pub struct ImmutablePath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

pub struct ImmutableMapPath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: IterableAttr<X, Item=Y>, R: Traverse<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

pub fn retrieve<'a, 'b, T, A: Attr<T>>(attr: A) -> ImmutablePath<'a, 'b, T, <A as Attr<T>>::Output, <A as Attr<T>>::Output, A, Identity>
        where Identity : Traverse<'a, 'b, <A as Attr<T>>::Output, <A as Attr<T>>::Output>

{
    ImmutablePath {
        attr: attr,
        next: Identity,
        phantomx: PhantomData,
        phantomy: PhantomData,
        phantomz: PhantomData
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> ImmutablePath<'a, 'b, X, Y, Z, A, R> {
    //pub fn new<T>(attr: A) -> ImmutablePathComponent<'a, 'b, T, <A as Attr<T>>::Output, <A as Attr<T>>::Output, A, Identity>
    //    where Identity : Traverse<'a, 'b, <A as Attr<T>>::Output, <A as Attr<T>>::Output>,
    //          A: Attr<T> {
    //    ImmutablePath {
    //        attr: attr,
    //        next: Identity,
    //        phantomx: PhantomData,
    //        phantomy: PhantomData,
    //        phantomz: PhantomData
    //    }
    //}

    pub fn from<NX, NY, NZ, NA>(self, attr: NA) -> ImmutablePath<'a, 'b, NX, NY, NZ, NA, ImmutablePath<'a, 'b, X, Y, Z, A, R>>
        where NA: Attr<NX, Output=NY>,
              ImmutablePath<'a, 'b, X, Y, Z, A, R>: Traverse<'a, 'b, NY, NZ> {
        ImmutablePath {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }

    pub fn map<NX, NY, NZ, NA>(self, attr: NA) -> ImmutableMapPath<'a, 'b, NX, NY, NZ, NA, ImmutablePath<'a, 'b, X, Y, Z, A, R>>
        where NA: IterableAttr<NX, Item=NY>,
              NZ: std::iter::Iterator<Item=NY>,
              ImmutablePath<'a, 'b, X, Y, Z, A, R>: Traverse<'a, 'b, NY, NZ> {
        ImmutableMapPath {
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

impl<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z> + 'b> Traverse<'a, 'b, X, Z> for ImmutablePath<'a, 'b, X, Y, Z, A, R> {
    fn traverse(&'b self, obj: &'b X) -> Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, A: IterableAttr<X, Item=Y>, R: Traverse<'a, 'b, Y, Z> + Copy + Clone> Traverse<'a, 'b, X, Box<Iterator<Item=Z> + 'a>> for ImmutableMapPath<'a, 'b, X, Y, Z, A, R> {
    fn traverse(&'b self, obj: &'b X) -> Box<Iterator<Item=Z> + 'a> {
        let iter = self.attr.iter(obj);
        let next = &self.next;
        let map = iter.map(move |v| next.traverse(v) );
        Box::new(map) as Box<std::iter::Iterator<Item=Z> + 'a>
    }
}

pub struct MutablePath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: AttrMut<X> + Attr<X, Output=Y>, R: TraverseMut<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b mut X>,
    phantomy: PhantomData<&'a mut Y>,
    phantomz: PhantomData<&'a mut Z>
}

pub fn retrieve_mut<'a, 'b, T, A: AttrMut<T>>(attr: A) -> MutablePath<'a, 'b, T, <A as Attr<T>>::Output, <A as Attr<T>>::Output, A, Identity>
        where Identity : TraverseMut<'a, 'b, <A as Attr<T>>::Output, <A as Attr<T>>::Output>
{
    MutablePath {
        attr: attr,
        next: Identity,
        phantomx: PhantomData,
        phantomy: PhantomData,
        phantomz: PhantomData
    }
}

impl<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: AttrMut<X, Output=Y>, R: TraverseMut<'a, 'b, Y, Z>> MutablePath<'a, 'b, X, Y, Z, A, R> {
    //pub fn new<T>(attr: A) -> ImmutablePathComponent<'a, 'b, T, <A as AttrMut<T>>::Output, <A as AttrMut<T>>::Output, A, Identity>
    //    where Identity : Traverse<'a, 'b, <A as AttrMut<T>>::Output, <A as AttrMut<T>>::Output>,
    //          A: AttrMut<T> {
    //    ImmutablePath {
    //        attr: attr,
    //        next: Identity,
    //        phantomx: PhantomData,
    //        phantomy: PhantomData,
    //        phantomz: PhantomData
    //    }
    //}

    pub fn from<NX, NY, NZ, NA>(self, attr: NA) -> MutablePath<'a, 'b, NX, NY, NZ, NA, MutablePath<'a, 'b, X, Y, Z, A, R>>
        where NA: AttrMut<NX, Output=NY>,
              MutablePath<'a, 'b, X, Y, Z, A, R>: TraverseMut<'a, 'b, NY, NZ> {
        MutablePath {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}

pub trait TraverseMut<'a, 'b: 'a, X: ?Sized, Y: ?Sized> {
    fn traverse_mut(&self, val: &'b mut X) -> &'a mut Y;
}

impl<'a, 'b: 'a, T> TraverseMut<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse_mut(&self, val: &'b mut T) -> &'a mut T { val }
}

impl<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, A: AttrMut<X, Output=Y>, R: TraverseMut<'a, 'b, Y, Z>> TraverseMut<'a, 'b, X, Z> for MutablePath<'a, 'b, X, Y, Z, A, R> {
    fn traverse_mut(&self, obj: &'b mut X) -> &'a mut Z {
        let val = self.attr.get_mut(obj);
        self.next.traverse_mut(val)
    }
}

