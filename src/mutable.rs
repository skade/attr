use super::*;
use std::marker::PhantomData;

pub trait AttrMut<Type: ?Sized> : Attr<Type> {
    fn get_mut<'a>(&self, i: &'a mut Type) -> &'a mut Self::Output;
}

pub trait IndexableAttrMut<'a, 'b: 'a, Type: ?Sized, Idx: ?Sized> : IndexableAttr<'a, 'b, Type, Idx> + AttrMut<Type> {
    fn at_mut(&self, i: &'b mut Type, idx: Idx) -> &'a mut <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait IterableAttrMut<'a, 'b: 'a, Type: ?Sized> : IterableAttr<'a, 'b, Type> + AttrMut<Type> {
    fn iter_mut(&self, i: &'b mut Type) -> Box<Iterator<Item=&'a mut Self::Item> + 'a>;
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

pub struct MutablePath<'a, 'b: 'a, X: 'b + ?Sized, Y: 'a + ?Sized, Z: 'a + ?Sized, A: AttrMut<X> + Attr<X, Output=Y>, R: TraverseMut<'a, 'b, Y, Z>> {
    attr: A,
    next: R,
    phantomx: PhantomData<&'b mut X>,
    phantomy: PhantomData<&'a mut Y>,
    phantomz: PhantomData<&'a mut Z>
}

impl<'a, 'b: 'a, X, Y, Z, A: AttrMut<X, Output=Y>, R: TraverseMut<'a, 'b, Y, Z>> MutablePath<'a, 'b, X, Y, Z, A, R> {
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
    fn traverse_mut(&'b self, val: &'b mut X) -> &'a mut Y;
}

impl<'a, 'b: 'a, T> TraverseMut<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse_mut(&'b self, val: &'b mut T) -> &'a mut T { val }
}

impl<'a, 'b: 'a, X, Y, Z, A: AttrMut<X, Output=Y>, R: TraverseMut<'a, 'b, Y, Z>> TraverseMut<'a, 'b, X, Z> for MutablePath<'a, 'b, X, Y, Z, A, R> {
    fn traverse_mut(&'b self, obj: &'b mut X) -> &'a mut Z {
        let val = self.attr.get_mut(obj);
        self.next.traverse_mut(val)
    }
}
