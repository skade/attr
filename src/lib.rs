use std::marker::PhantomData;

pub type Result<X> = std::result::Result<X, String>;

pub trait Attr<Type: ?Sized> {
    type Output;

    fn name(&self) -> &str;
    fn get(&self, i: Type) -> Self::Output;
}

pub trait InsecureAttr<Type: ?Sized> {
    type Output;

    fn name(&self) -> &str;
    fn get(&self, i: Type) -> Result<Self::Output>;
}

pub trait IndexableAttr<Type: ?Sized, Idx: ?Sized> : Attr<Type>{
    type Output;

    fn at(&self, i: Type, idx: Idx) -> <Self as IndexableAttr<Type, Idx>>::Output;
}

pub trait InsecureIndexableAttr<Type: ?Sized, Idx: ?Sized> : InsecureAttr<Type> {
    type Output;

    fn at(&self, i: Type, idx: Idx) -> Result<<Self as InsecureIndexableAttr<Type, Idx>>::Output>;
}

pub trait IterableAttr<'a, Type: ?Sized> {
    type Item: 'a;

    fn iter(&self, i: Type) -> Box<Iterator<Item=Self::Item> + 'a>;
}

pub trait Traverse<'a, 'b: 'a, X: ?Sized + 'b, Y: ?Sized + 'b> {
    #[inline]
    fn traverse(&'a self, val: X) -> Y;
}

pub struct Identity;

pub struct Path<X, Z, A: Attr<X>, R> {
    attr: A,
    next: R,
    phantom_x: PhantomData<X>,
    phantom_z: PhantomData<Z>,
}

pub struct InsecurePath<X, Z, A: InsecureAttr<X>, R> {
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

pub fn retrieve_insecure<X, Z, A>(attr: A) -> InsecurePath<X, Z, A, Identity>
    where A: InsecureAttr<X>
{
    InsecurePath {
        attr: attr,
        next: Identity,
        phantom_x: PhantomData,
        phantom_z: PhantomData,
    }
}

impl<'a, 'b: 'a, T: 'b> Traverse<'a, 'b, T, T> for Identity {
    #[inline]
    fn traverse(&'a self, val: T) -> T { val }
}

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: Attr<X>, R: Traverse<'a, 'b, A::Output, Z>> Traverse<'a, 'b, X, Z> for Path<X, Z, A, R> where <A as Attr<X>>::Output: 'b {
    #[inline]
    fn traverse(&'a self, obj: X) -> Z {
        let val = self.attr.get(obj);
        self.next.traverse(val)
    }
}

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: InsecureAttr<X>, R: Traverse<'a, 'b, A::Output, Z>> Traverse<'a, 'b, X, Result<Z>> for InsecurePath<X, Z, A, R> where <A as InsecureAttr<X>>::Output: 'b {
    #[inline]
    fn traverse(&'a self, obj: X) -> Result<Z> {
        let val = self.attr.get(obj);
        match val {
            Ok(v) => Ok(self.next.traverse(v)),
            Err(_) => Err("Something went wrong".into())
        }
    }
}

impl<'a, X: 'a, Z: 'a, A: IterableAttr<'a, X>, R: Traverse<'a, 'a, A::Item, Z>> Traverse<'a, 'a, X, Box<Iterator<Item=Z> + 'a>> for MapPath<A, R> {
    #[inline]
    fn traverse(&'a self, obj: X) -> Box<Iterator<Item=Z> + 'a> {
        let iter = self.attr.iter(obj);
        let next = &self.next;
        let map = iter.map(move |v| next.traverse(v) );
        Box::new(map)
    }
}

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: Attr<X>, R: Traverse<'a, 'b, A::Output, Z>> Path<X, Z, A, R> where <A as Attr<X>>::Output: 'b {
    pub fn from<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> Path<NX, NZ, NA, Self>
        where A: Attr<NY, Output=Z>,
              NA: Attr<NX, Output=NY> {
        Path {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    pub fn try<NX: 'a, NY: 'a, NZ: 'a, NA>(self, attr: NA) -> InsecurePath<NX, NZ, NA, Self>
        where A: Attr<NY, Output=Z>,
              NA: InsecureAttr<NX, Output=NY> {
        InsecurePath {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    pub fn mapped<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> MapPath<NA, Self>
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

impl<'a, 'b: 'a, X: 'b, Z: 'b, A: InsecureAttr<X>, R: Traverse<'a, 'b, A::Output, Z>> InsecurePath<X, Z, A, R> where <A as InsecureAttr<X>>::Output: 'b {
    pub fn from<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> Path<NX, NZ, NA, Self>
        where A: InsecureAttr<NY, Output=Z>,
              NA: Attr<NX, Output=NY> {
        Path {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    pub fn try<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> InsecurePath<NX, NZ, NA, Self>
        where A: InsecureAttr<NY, Output=Z>,
              NA: InsecureAttr<NX, Output=NY> {
        InsecurePath {
            attr: attr,
            next: self,
            phantom_x: PhantomData,
            phantom_z: PhantomData,
        }
    }

    pub fn mapped<NX: 'b, NY: 'b, NZ: 'b, NA>(self, attr: NA) -> MapPath<NA, Self>
        where A: InsecureAttr<X>,
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
    pub fn from<'a, 'b: 'a, X: 'b, Y: 'b, Z: 'b, NX: 'b, NY: 'b, NA>(self, attr: NA) -> Path<NX, Box<std::iter::Iterator<Item=Z> + 'a>, NA, Self>
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
