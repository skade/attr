extern crate serde_json;

pub mod serde_impl;

use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;

pub trait Attr<Type: ?Sized> {
    type Output: ?Sized;

    fn name(&self) -> &str;
    fn get<'a>(&self, i: &'a Type) -> &'a Self::Output;
}

pub trait AttrMut<Type: ?Sized> : Attr<Type> {
    fn get_mut<'a>(&self, i: &'a mut Type) -> &'a mut Self::Output;
}

pub trait IndexableAttr<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> : Attr<Type>{
    type Output: ?Sized;

    fn at(&self, i: &'b Type, idx: Idx) -> &'a <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait IndexableAttrMut<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> : IndexableAttr<'a, 'b, Type, Idx> {
    type Output: ?Sized;

    fn at_mut(&self, i: &'b mut Type, idx: Idx) -> &'a mut <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

impl<'a, 'b : 'a, Idx, O: Index<Idx> + 'a, Type, A: Attr<Type, Output=O>> IndexableAttr<'a, 'b, Type, Idx> for A {
    type Output = O::Output;

    fn at(&self, i: &'b Type, idx: Idx) -> &'a <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output {
        &self.get(i)[idx]
    }
}

impl<'a, 'b : 'a, Idx, O: IndexMut<Idx> + 'a, Type, A: AttrMut<Type, Output=O>> IndexableAttrMut<'a, 'b, Type, Idx> for A {
    type Output = O::Output;

    fn at_mut(&self, i: &'b mut Type, idx: Idx) -> &'a mut <Self as IndexableAttrMut<'a, 'b, Type, Idx>>::Output {
        &mut self.get_mut(i)[idx]
    }
}

pub trait Combine<'a, 'b: 'a, X, A: Attr<X>, B: Attr<A::Output>> {
    fn get(&self, i: &'b X) -> &'a B::Output;
}

pub trait MutCombine<'a, 'b: 'a, X, A: AttrMut<X>, B: AttrMut<A::Output>> {
    fn get_mut(&self, i: &'b mut X) -> &'a mut B::Output;
}

pub struct Combiner<'a, 'b: 'a, X: 'b, Y: 'a, Z: 'a, A: Attr<X, Output=Y>, B: Attr<Y, Output=Z>> {
    a: A,
    b: B,
    phantomx: PhantomData<&'b X>,
    phantomy: PhantomData<&'a Y>,
    phantomz: PhantomData<&'a Z>
}

impl<'a, 'b, X, Y, Z, A: Attr<X, Output=Y>, B: Attr<Y, Output=Z>> Combiner<'a, 'b, X, Y, Z, A, B> {
    pub fn combine(a: A, b: B) -> Self {
        Combiner {
            a: a,
            b: b,
            phantomx: PhantomData,
            phantomy: PhantomData,
            phantomz: PhantomData
        }
    }
}

impl<'a, 'b: 'a, X: 'b, Y, Z: 'a, A: Attr<X, Output=Y>, B: Attr<Y, Output=Z>> Combine<'a, 'b, X, A, B> for Combiner<'a, 'b, X, Y, Z, A, B> {
    fn get(&self, i: &'b X) -> &'a B::Output {
        self.b.get(self.a.get(i))
    }
}

impl<'a, 'b: 'a, X: 'b, Y, Z: 'a, A: AttrMut<X, Output=Y>, B: AttrMut<Y, Output=Z>> MutCombine<'a, 'b, X, A, B> for Combiner<'a, 'b, X, Y, Z, A, B> {
    fn get_mut(&self, i: &'b mut X) -> &'a mut B::Output {
        self.b.get_mut(self.a.get_mut(i))
    }
}
