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

pub struct ImmutablePathComponent<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

impl<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> ImmutablePathComponent<'a, 'b, X, Y, Z, A, R> {
    pub fn new<NX, NY, NA>(attr: NA) -> ImmutablePathComponent<'a, 'b, NX, NY, NY, NA, Identity>
        where Identity : Traverse<'a, 'b, NY, NY>,
              NA: Attr<NX, Output=NY> {
        ImmutablePathComponent {
            attr: attr,
            next: Identity,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }

    pub fn prepend<NX, NY, NZ, NA>(self, attr: NA) -> ImmutablePathComponent<'a, 'b, NX, NY, NZ, NA, ImmutablePathComponent<'a, 'b, X, Y, Z, A, R>>
        where NA: Attr<NX, Output=NY>,
              ImmutablePathComponent<'a, 'b, X, Y, Z, A, R>: Traverse<'a, 'b, NY, NZ> {
        ImmutablePathComponent {
            attr: attr,
            next: self,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}

pub struct Identity;

pub trait Traverse<'a, 'b: 'a, X, Y> {
    fn traverse(&self, val: &'b X) -> &'a Y;
}

impl<'a, 'b: 'a, T> Traverse<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse(&self, val: &'b T) -> &'a T { val }
}

impl<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, A: Attr<X, Output=Y>, R: Traverse<'a, 'b, Y, Z>> Traverse<'a, 'b, X, Z> for ImmutablePathComponent<'a, 'b, X, Y, Z, A, R> {
    fn traverse(&self, obj: &'b X) -> &'a Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

pub struct Path<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: Attr<X, Output=Y>, B: Attr<Y, Output=Z>> {
    a: A,
    b: B,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

impl<'a, 'b, X, Y, Z, A: Attr<X, Output=Y>, B: Attr<Y, Output=Z>> Path<'a, 'b, X, Y, Z, A, B> {
    pub fn combine(a: A, b: B) -> Self {
        Path {
            a: a,
            b: b,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}

impl<'a, 'b: 'a, X: 'b, Y, Z: 'a, A: Attr<X, Output=Y>, B: Attr<Y, Output=Z>> Combine<'a, 'b, X, A, B> for Path<'a, 'b, X, Y, Z, A, B> {
    fn get(&self, i: &'b X) -> &'a B::Output {
        self.b.get(self.a.get(i))
    }
}

impl<'a, 'b: 'a, X: 'b, Y, Z: 'a, A: AttrMut<X, Output=Y>, B: AttrMut<Y, Output=Z>> MutCombine<'a, 'b, X, A, B> for Path<'a, 'b, X, Y, Z, A, B> {
    fn get_mut(&self, i: &'b mut X) -> &'a mut B::Output {
        self.b.get_mut(self.a.get_mut(i))
    }
}
