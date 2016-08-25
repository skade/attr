extern crate serde_json;

pub mod serde_impl;

use std::marker::PhantomData;
use std::ops::Add;

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

pub trait IndexableAttrMut<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> : IndexableAttr<'a, 'b, Type, Idx> {
    type Output: ?Sized;

    fn at_mut(&self, i: &'b mut Type, idx: Idx) -> &'a mut <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait Combine<'a, 'b: 'a, X, A: Attr<X>, B: Attr<A::Output>> {
    fn get(&self, i: &'b X) -> &'a B::Output;
}

pub trait MutCombine<'a, 'b: 'a, X, A: AttrMut<X>, B: AttrMut<A::Output>> {
    fn get_mut(&self, i: &'b mut X) -> &'a mut B::Output;
}

pub struct ImmutablePath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

pub fn new_immutable_path<'a, 'b, T, A: Attr<T>>(attr: A) -> ImmutablePath<'a, 'b, T, <A as Attr<T>>::Output, <A as Attr<T>>::Output, A, Identity>
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

    pub fn prepend<NX, NY, NZ, NA>(self, attr: NA) -> ImmutablePath<'a, 'b, NX, NY, NZ, NA, ImmutablePath<'a, 'b, X, Y, Z, A, R>>
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
}

pub struct Identity;

pub trait Traverse<'a, 'b: 'a, X: ?Sized, Y: ?Sized> {
    fn traverse(&self, val: &'b X) -> &'a Y;
}

impl<'a, 'b: 'a, T> Traverse<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse(&self, val: &'b T) -> &'a T { val }
}

impl<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> Traverse<'a, 'b, X, Z> for ImmutablePath<'a, 'b, X, Y, Z, A, R> {
    fn traverse(&self, obj: &'b X) -> &'a Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

pub struct MutablePath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: AttrMut<X> + Attr<X, Output=Y>, R: TraverseMut<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b mut X>,
    phantomy: PhantomData<&'a mut Y>,
    phantomz: PhantomData<&'a mut Z>
}

pub fn new_mutable_path<'a, 'b, T, A: AttrMut<T>>(attr: A) -> MutablePath<'a, 'b, T, <A as Attr<T>>::Output, <A as Attr<T>>::Output, A, Identity>
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

    pub fn prepend<NX, NY, NZ, NA>(self, attr: NA) -> MutablePath<'a, 'b, NX, NY, NZ, NA, MutablePath<'a, 'b, X, Y, Z, A, R>>
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

