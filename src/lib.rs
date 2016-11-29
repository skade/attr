extern crate serde_json;

pub trait Attr<Type: ?Sized + 'b> {
    type Output;

    fn name(&self) -> &str;
    fn get(&self, i: Type) -> Self::Output;
}

pub trait Attributes<AttributeType> {
    fn attrs() -> AttributeType;
}

pub trait IndexableAttr<'a, 'b: 'a, Type: 'b + ?Sized, Idx: ?Sized> : Attr<'a, 'b, Type> {
    type Output: 'a;

    fn at(&self, i: Type, idx: Idx) -> <Self as IndexableAttr<'a, 'b, Type, Idx>>::Output;
}

pub trait IterableAttr<'a, 'b: 'a, Type: ?Sized + 'b> : Attr<'a, 'b, Type> {
    type Item: 'a;

    fn iter(&self, i: Type) -> Box<Iterator<Item=Self::Item> + 'a>;
}

pub trait Traverse<'a, 'b: 'a, X: ?Sized + 'b, Y: ?Sized + 'a> {
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

impl<'a, 'b: 'a, T: 'b, A: Attr<'a, 'b, T>> Traverse<'a, 'b, T, A::Output> for Tip<A> {
    #[inline]
    fn traverse(&'b self, val: T) -> A::Output { self.attr.get(val) }
}

impl<'a, 'b: 'a, X: 'b, Z: 'a, A: Attr<'b, 'b, X>, R: Traverse<'a, 'b, A::Output, Z>> Traverse<'a, 'b, X, Z> for Path<A, R> {
    #[inline]
    fn traverse(&'b self, obj: X) -> Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

impl<'a, 'b: 'a, X: 'b, Z: 'a, A: IterableAttr<'b, 'b, X>, R: Traverse<'a, 'b, A::Item, Z>> Traverse<'a, 'b, X, Box<Iterator<Item=Z> + 'a>> for MapPath<A, R> {
    #[inline]
    fn traverse(&'b self, obj: X) -> Box<Iterator<Item=Z> + 'a> {
        let iter = self.attr.iter(obj);
        let next = &self.next;
        let map = iter.map(move |v| next.traverse(v) );
        Box::new(map)
    }
}

impl<A> Tip<A> {
    pub fn from<'a, 'b: 'a, NX: 'b, NA>(self, attr: NA) -> Path<NA, Self>
        where A: Attr<'b, 'b, NA::Output>,
              NA: Attr<'b, 'b, NX> {
        Path {
            attr: attr,
            next: self,
        }
    }

    pub fn mapped<'a, 'b: 'a, NX: 'b, NA>(self, attr: NA) -> MapPath<NA, Self>
        where A: Attr<'b, 'b, NA::Item>,
              NA: IterableAttr<'b, 'b, NX>,
              Self: Traverse<'a, 'b, NA::Item, A::Output> {
        MapPath {
            attr: attr,
            next: self,
        }
    }
}

impl<A, R> Path<A, R> {
    pub fn from<'a, 'b: 'a, NX: 'b, Z: 'a, NA>(self, attr: NA) -> Path<NA, Self>
        where A: Attr<'b, 'b, NA::Output>,
              R: Traverse<'a, 'b, A::Output, Z>,
              NA: Attr<'b, 'b, NX>,
              Self: Traverse<'a, 'b, NA::Output, Z> {
        Path {
            attr: attr,
            next: self,
        }
    }

    pub fn mapped<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, NX: 'b, NY: 'b, NZ: 'a, NA>(self, attr: NA) -> MapPath<NA, Self>
        where A: Attr<'a, 'b, X, Output=Y>,
              R: Traverse<'a, 'b, Y, Z>,
              NA: IterableAttr<'a, 'b, NX, Item=NY>,
              Self: Traverse<'a, 'b, NY, NZ> {
        MapPath {
            attr: attr,
            next: self,
        }
    }
}

impl<A, R> MapPath<A, R> {
    pub fn from<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'a, NX: 'b, NY: 'b, NA>(self, attr: NA) -> Path<NA, Self>
        where A: IterableAttr<'a, 'b, X, Item=Y>,
              R: Traverse<'a, 'b, Y, Z>,
              NA: Attr<'a, 'b, NX, Output=NY>,
              Self: Traverse<'a, 'b, NY, Box<std::iter::Iterator<Item=Z>>>
    {
        Path {
            attr: attr,
            next: self,
        }
    }
}
