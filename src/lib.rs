extern crate serde_json;

pub mod serde;

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

pub struct Tip<A> {
    attr: A,
}

pub struct Path<A, R> {
    attr: A,
    next: R,
}

pub struct MapPath<A, R> {
    attr: A,
    next: R,
}

pub fn retrieve<A>(attr: A) -> Tip<A>
{
    Tip {
        attr: attr,
    }
}

impl<'a, 'b: 'a, T: 'a, O: 'a, A: Attr<T, Output=O>> Traverse<'a, 'b, T, A::Output> for Tip<A> {
    #[inline]
    fn traverse(&'b self, val: T) -> A::Output { self.attr.get(val) }
}

impl<'a, 'b: 'a, X: 'a, Z: 'a, O: 'a, A: Attr<X, Output=O>, R: Traverse<'a, 'b, A::Output, Z>> Traverse<'a, 'b, X, Z> for Path<A, R> {
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

impl<A> Tip<A> {
    pub fn from<'a, NX: 'a, NA>(self, attr: NA) -> Path<NA, Self>
        where A: Attr<NA::Output>,
              NA: Attr<NX> {
        Path {
            attr: attr,
            next: self,
        }
    }

    pub fn mapped<'a, 'b: 'a, NX: 'a, NA>(self, attr: NA) -> MapPath<NA, Self>
        where A: Attr<NA::Item>,
              NA: IterableAttr<'a, NX> {
        MapPath {
            attr: attr,
            next: self,
        }
    }
}

impl<A, R> Path<A, R> {
    pub fn from<'a, 'b: 'a, NX: 'a, Z: 'a, NA>(self, attr: NA) -> Path<NA, Self>
        where A: Attr<NA::Output>,
              R: Traverse<'a, 'b, A::Output, Z>,
              NA: Attr<NX>,
              <A as Attr<<NA as Attr<NX>>::Output>>::Output: 'a,
              <NA as Attr<NX>>::Output: 'a,
              Self: Traverse<'a, 'b, NA::Output, Z> {
        Path {
            attr: attr,
            next: self,
        }
    }

    pub fn mapped<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, NX: 'a, NY: 'a, NZ: 'a, NA>(self, attr: NA) -> MapPath<NA, Self>
        where A: Attr<X, Output=Y>,
              R: Traverse<'a, 'b, Y, Z>,
              NA: IterableAttr<'a, NX, Item=NY>,
              Self: Traverse<'a, 'b, NY, NZ> {
        MapPath {
            attr: attr,
            next: self,
        }
    }
}

impl<A, R> MapPath<A, R> {
    pub fn from<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, NX: 'b, NY: 'b, NA>(self, attr: NA) -> Path<NA, Self>
        where A: IterableAttr<'a, X, Item=Y>,
              R: Traverse<'a, 'b, Y, Z>,
              NA: Attr<NX, Output=NY>,
              Self: Traverse<'a, 'b, NY, Box<std::iter::Iterator<Item=Z>>>
    {
        Path {
            attr: attr,
            next: self,
        }
    }
}
