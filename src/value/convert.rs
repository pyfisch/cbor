/*!
 * Convert Value from/to other data types and behaviors as Iterator and Map.
 * The module extends Value as a generic variant for API functions.
 */

use std::{
    borrow::Cow,
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    iter::{FromIterator, FusedIterator},
    mem,
    ops::{Index, IndexMut},
    slice::{Iter as SliceIter, IterMut as SliceIterMut},
    vec::IntoIter as VecIntoIter,
};

use crate::{
    error::{Error, Result},
    Value,
};
use Value::*;

impl Value {
    /// Index into a array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    pub fn get<K: Into<Value>>(&self, index: K) -> Option<&Value> {
        let index = index.into();
        match self {
            Array(x) => usize::try_from(index).ok().and_then(|n| x.get(n)),
            Map(x) => x.get(&index),
            _ => None,
        }
    }

    /// Mutably index into a JSON array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    pub fn get_mut<K: Into<Value>>(&mut self, index: K) -> Option<&mut Value> {
        let idx = index.into();
        match self {
            Array(ref mut vec) if idx.is_integer() => {
                usize::try_from(idx).ok().and_then(move |n| vec.get_mut(n))
            }
            Map(ref mut map) => map.get_mut(&idx),
            _ => None,
        }
    }

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    pub fn take(&mut self) -> Value {
        mem::replace(self, Null)
    }

    /// Null check
    pub fn is_null(&self) -> bool {
        *self == Null
    }

    /// Convert Value to Option, Null to None.
    /// These funtions give Value Option methods.
    pub fn into_option(self) -> Option<Value> {
        if self.is_null() {
            None
        } else {
            Some(self)
        }
    }

    /// Value as Option.
    pub fn as_option(&self) -> Option<&Value> {
        if self.is_null() {
            None
        } else {
            Some(self)
        }
    }

    /// Make reference iterator.
    /// These functions make Value iterable.
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    /// Make mutable iterator.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut::new(self)
    }
}

/// Conversation into an Iterator.
impl IntoIterator for Value {
    type Item = Value;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> IntoIter {
        IntoIter::new(self)
    }
}

/// Conversation to an mutable reference Iterator.
impl<'a> IntoIterator for &'a mut Value {
    type Item = &'a mut Value;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> IterMut<'a> {
        self.iter_mut()
    }
}

/// Conversation to an reference Iterator.
impl<'a> IntoIterator for &'a Value {
    type Item = &'a Value;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

/// Enumerate singleton and container Value as an Iter.
#[derive(Clone, Debug)]
pub enum Iter<'a> {
    Group(SliceIter<'a, Value>),
    Solo(Option<&'a Value>),
}

impl<'a> Iter<'a> {
    pub fn new(v: &'a Value) -> Self {
        match v {
            Array(x) => Iter::Group(x.iter()),
            _ => Iter::Solo(if v.is_null() { None } else { Some(v) }),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<&'a Value> {
        match self {
            Iter::Group(x) => x.next(),
            Iter::Solo(x) => x.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = match self {
            Iter::Group(x) => x.len(),
            Iter::Solo(x) if x.is_some() => 1,
            Iter::Solo(_) => 0,
        };
        (n, Some(n))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<&'a Value> {
        match self {
            Iter::Group(x) => x.next_back(),
            Iter::Solo(x) => x.take(),
        }
    }
}

impl ExactSizeIterator for Iter<'_> {}
impl FusedIterator for Iter<'_> {}

#[derive(Debug)]
pub enum IterMut<'a> {
    Group(SliceIterMut<'a, Value>),
    Solo(Option<&'a mut Value>),
}

impl<'a> IterMut<'a> {
    pub fn new(v: &'a mut Value) -> Self {
        match v {
            Array(x) => IterMut::Group(x.iter_mut()),
            _ => IterMut::Solo(if v.is_null() { None } else { Some(v) }),
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Value;

    fn next(&mut self) -> Option<&'a mut Value> {
        match self {
            IterMut::Group(x) => x.next(),
            IterMut::Solo(x) => x.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = match self {
            IterMut::Group(x) => x.len(),
            IterMut::Solo(x) if x.is_some() => 1,
            IterMut::Solo(_) => 0,
        };
        (n, Some(n))
    }
}

impl<'a> DoubleEndedIterator for IterMut<'a> {
    fn next_back(&mut self) -> Option<&'a mut Value> {
        match self {
            IterMut::Group(x) => x.next_back(),
            IterMut::Solo(x) => x.take(),
        }
    }
}

impl ExactSizeIterator for IterMut<'_> {}
impl FusedIterator for IterMut<'_> {}

#[derive(Clone, Debug)]
pub enum IntoIter {
    Group(VecIntoIter<Value>),
    Solo(Option<Value>),
}

impl IntoIter {
    pub fn new(v: Value) -> Self {
        match v {
            Array(x) => IntoIter::Group(x.into_iter()),
            _ => IntoIter::Solo(v.into_option()),
        }
    }
}

impl Iterator for IntoIter {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        match self {
            IntoIter::Group(x) => x.next(),
            IntoIter::Solo(x) => x.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = match self {
            IntoIter::Group(x) => x.len(),
            IntoIter::Solo(x) if x.is_some() => 1,
            IntoIter::Solo(_) => 0,
        };
        (n, Some(n))
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Value> {
        match self {
            IntoIter::Group(x) => x.next_back(),
            IntoIter::Solo(x) => x.take(),
        }
    }
}

impl ExactSizeIterator for IntoIter {}
impl FusedIterator for IntoIter {}

/// Convert iterable data type to Value. e.g. Vec, slice.
impl<T: Into<Value>> FromIterator<T> for Value {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Array(iter.into_iter().map(Into::into).collect())
    }
}

/// Convert key-value pairs data collections to Value. e.g. HashMap, k-v Array.
impl<K: Into<Value>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Map(iter
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect())
    }
}

/// Convert copy-on-write string to Value.
impl<'a> From<Cow<'a, str>> for Value {
    fn from(f: Cow<'a, str>) -> Self {
        Text(f.into_owned())
    }
}

/// Dedicated conversation for Vec.
impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(arr: Vec<T>) -> Self {
        Array(arr.into_iter().map(T::into).collect())
    }
}

/// Dedicated conversation for slice.
/// Notice: [T] shadow the u8 type.
impl<T: Clone + Into<Value>> From<&[T]> for Value {
    fn from(f: &[T]) -> Self {
        Array(f.iter().cloned().map(Into::into).collect())
    }
}

/// Void as Null.
impl From<()> for Value {
    fn from((): ()) -> Self {
        Null
    }
}

/// Flatten Option to Value.
impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(ov: Option<T>) -> Self {
        match ov {
            Some(v) => v.into(),
            None => Null,
        }
    }
}

macro_rules! impl_from_ref {
    ($($i:ident, $t:ty;)*) => {
        $(
            /// Conversation for ownerable data reference to Value.
            impl From<&$t> for Value {
                fn from(x: &$t) -> Self {
                    Value::$i(x.to_owned())
                }
            }
        )*
    };
}

impl_from_ref! {
    Bool, bool;
    Integer, i128;
    Float, f64;
    Text, String;
    Text, str;
    Bytes, Vec<u8>;
    Bytes, [u8];
    Map, BTreeMap<Value, Value>;
}

macro_rules! impl_integer_from {
    ($($t:ty)*) => {
        $(
            /// Conversation for special interges.
            impl From<$t> for Value {
                fn from(x: $t) -> Self {
                    Value::Integer(x as i128)
                }
            }
        )*
    };
}

impl_integer_from! {u128 usize isize}

macro_rules! impl_from {
    ($($i:ident, $t:ty;)*) => {
        $(
            /// Conversation for simple data type.
            impl From<$t> for Value {
                fn from(x: $t) -> Self {
                    Value::$i(x.into())
                }
            }
        )*
    };
}

impl_from! {
    Bool, bool;
    Integer, i8;
    Integer, i16;
    Integer, i32;
    Integer, i64;
    Integer, i128;
    Integer, u16;
    Integer, u32;
    Integer, u64;
    Float, f32;
    Float, f64;
    Text, String;
    Bytes, Vec<u8>;
    Map, BTreeMap<Value, Value>;
}

macro_rules! impl_try_from {
    ($($i:ident, $t:ty;)*) => {
        $(
            /// Try to extract given type data from Value.
            impl TryFrom<Value> for $t {
                type Error = Error;
                fn try_from(v: Value) -> Result<Self> {
                    if let Value::$i(x) = v {
                        Ok(x.try_into().map_err(Error::message)?)
                    } else {
                        Err(Error::message("mismatch"))
                    }
                }
            }
        )*
    };
}

impl_try_from! {
    Bool, bool;
    Integer, i8;
    Integer, i16;
    Integer, i32;
    Integer, i64;
    Integer, i128;
    Integer, isize;
    Integer, u8;
    Integer, u16;
    Integer, u32;
    Integer, u64;
    Integer, u128;
    Integer, usize;
    Float, f64;
    Text, String;
    Bytes, Vec<u8>;
    Array, Vec<Value>;
    Map, BTreeMap<Value, Value>;
}

macro_rules! impl_try_from_ref {
    ($($i:ident, $t:ty;)*) => {
        $(
            /// Try to extract given type data from Value reference.
            impl TryFrom<&Value> for $t {
                type Error = Error;
                fn try_from(v: &Value) -> Result<Self> {
                    if let Value::$i(x) = v {
                        Ok(x.clone().try_into().map_err(Error::message)?)
                    } else {
                        Err(Error::message("mismatch"))
                    }
                }
            }
        )*
    };
}

impl_try_from_ref! {
    Bool, bool;
    Integer, i8;
    Integer, i16;
    Integer, i32;
    Integer, i64;
    Integer, i128;
    Integer, isize;
    Integer, u8;
    Integer, u16;
    Integer, u32;
    Integer, u64;
    Integer, u128;
    Integer, usize;
    Float, f64;
    Text, String;
    Bytes, Vec<u8>;
    Array, Vec<Value>;
    Map, BTreeMap<Value, Value>;
}

macro_rules! is_as_mut {
    ($($i:ident - $t:ty : $f1:ident, $f2:ident $(, $f3:ident)?;)*) => {
        $(
            impl Value {
                /// Type check.
                pub fn $f1(&self) -> bool {
                    match self {
                        Value::$i(_) => true,
                        _ => false,
                    }
                }

                /// Try as type.
                pub fn $f2(&self) -> Result<&$t> {
                    match self {
                        Value::$i(ref x) => Ok(x),
                        _ => Err(Error::message("mismatch"))
                    }
                }
                $(
                    /// Try as mut type.
                    pub fn $f3(&mut self) -> Result<&mut $t> {
                        match self {
                            Value::$i(ref mut x) => Ok(x),
                            _ => Err(Error::message("mismatch"))
                        }
                    }
                )?
            }
        )*
    };
}

is_as_mut! {
    Bool - bool: is_bool, as_bool, as_mut_bool;
    Integer - i128: is_integer, as_integer, as_mut_integer;
    Float - f64: is_float, as_float, as_mut_float;
    Text - String: is_string, as_string, as_mut_string;
    Text - str: is_str, as_str, as_mut_str;
    Bytes - Vec<u8>: is_bytes, as_bytes, as_mut_bytes;
    Bytes - [u8]: is_binary, as_binary, as_mut_binary;
    Array - Vec<Value>: is_array, as_array, as_mut_array;
    Array - [Value]: is_slice, as_slice, as_mut_slice;
    Map - BTreeMap<Value, Value>: is_map, as_map, as_mut_map;
}

impl Index<&Value> for Value {
    type Output = Value;

    fn index(&self, idx: &Value) -> &Value {
        self.index(idx.clone())
    }
}

impl IndexMut<&Value> for Value {
    fn index_mut(&mut self, idx: &Value) -> &mut Value {
        self.index_mut(idx.clone())
    }
}

/// Convenient version for multiple type data as Index.
/// And for infallible design, if the indexed Value is not existed,
/// a static Null reference is retured without exception.
impl<K: Into<Value>> Index<K> for Value {
    type Output = Value;

    #[inline]
    fn index(&self, idx: K) -> &Value {
        static NULL: &Value = &Value::Null;
        self.get(idx).unwrap_or(&NULL)
    }
}

/// Null is treated as empty Map for insertion.
impl<K: Into<Value>> IndexMut<K> for Value {
    #[inline]
    fn index_mut(&mut self, idx: K) -> &mut Value {
        let idx = idx.into();
        if self.is_null() {
            *self = Value::Map(BTreeMap::new());
        }
        match self {
            Array(ref mut vec) if idx.is_integer() => {
                let n = usize::try_from(idx).unwrap();
                if n >= vec.len() {
                    panic!("out of bound");
                }
                vec.get_mut(n).unwrap()
            }
            Map(ref mut map) => map.entry(idx).or_insert(Null),
            _ => panic!("type is invalid for index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts() {
        let v = Null;
        assert!(v.is_null());

        let v = Value::from(());
        assert!(v.is_null());

        let v1 = Value::from(Some(Some(1)));
        let v2 = Value::from(Some(1));
        assert!(v1.is_integer() && v2.is_integer());
        assert_eq!(v1, v2);

        let v = Value::Integer(1);
        assert!(v.is_integer());
        let rv = Null.as_option().unwrap_or_else(|| &v);
        assert_eq!(*rv, v);
        assert_eq!(v.clone().into_option(), Some(v));

        let v: Value = "hello".into();
        assert!(v.is_string());
        assert_eq!(v.as_str().unwrap(), "hello");

        let d = vec!["hello", "world"];
        let v: Value = d.into();
        assert!(v.is_array());
        assert_eq!(v[1].as_str().unwrap(), "world");

        let mut v: Value = Null;
        assert!(v.is_null());
        v["hello"] = "world".into();
        assert!(v.is_map());
        assert_eq!(v.get("hello").unwrap().as_str().unwrap(), "world");
        assert_eq!(v["hello"].as_str().unwrap(), "world");

        let mut v: Value = Null;
        v["hello"]["world"] = "greetings".into();
        assert!(v["hello"].is_map());
        assert_eq!(v["hello"]["world"].as_str().unwrap(), "greetings");
        let x = &mut v["hello"];
        *x = "world".into();
        assert!(v["hello"].is_string());
        assert_eq!(v["hello"].as_str().unwrap(), "world");

        let mut v = Value::from(true);
        let x = v.take();
        assert_eq!(v, Null);
        assert!(x.as_bool().unwrap());
    }

    #[test]
    fn iterators() {
        let mut it = Null.iter();
        assert_eq!(it.next(), None);

        let v: Value = false.into();
        let mut it = v.iter();
        assert!(it.next().is_some());
        assert!(it.next().is_none());

        let v0 = vec![1];
        let v1: Value = v0.into();
        assert!(v1.is_array());
        let v2: Value = 1.into();
        assert_eq!(v1[0], v2);
        let mut it1 = v1.iter();
        let mut it2 = v2.iter();
        assert_eq!(it1.next().unwrap(), &v2);
        assert_eq!(it2.next().unwrap(), &v2);
        assert!(it1.next().is_none());
        assert!(it2.next().is_none());

        let v0 = vec![1, 2, 3];
        let v1: Value = v0.into_iter().map(|x| x + 1).collect();
        let v2: Vec<usize> = v1.iter().map(|x| usize::try_from(x).unwrap()).collect();
        assert_eq!(v2, vec![2, 3, 4]);

        let v0 = vec![("a", 1), ("b", 2), ("c", 3)];
        let v1: Value = v0.into_iter().collect();
        assert_eq!(v1["a"], Value::from(1));
    }
}

/*
```
*/
