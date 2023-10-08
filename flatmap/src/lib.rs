#![forbid(unsafe_code)]

use std::mem::swap;
use std::{borrow::Borrow, iter::FromIterator, ops::Index};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.0.binary_search_by(|pair| pair.0.cmp(&key)) {
            Ok(idx) => {
                let prev = self.0.get_mut(idx).unwrap();
                let mut new_pair = (key, value);
                swap(prev, &mut new_pair);
                Some(new_pair.1)
            }
            Err(idx) => {
                self.0.insert(idx, (key, value));
                None
            }
        }
    }

    pub fn get<Q: ?Sized + Ord>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        match self
            .0
            .binary_search_by(|pair| <K as Borrow<Q>>::borrow(&pair.0).cmp(key))
        {
            Ok(idx) => Some(&self.0.get(idx).unwrap().1),
            Err(_) => None,
        }
    }

    pub fn remove<Q: ?Sized + Ord>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        match self.remove_entry(key) {
            None => None,
            Some(p) => Some(p.1),
        }
    }

    pub fn remove_entry<Q: ?Sized + Ord>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
    {
        match self
            .0
            .binary_search_by(|pair| <K as Borrow<Q>>::borrow(&pair.0).cmp(key))
        {
            Ok(idx) => Some(self.0.remove(idx)),
            Err(_) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K: Ord, Q: ?Sized + Ord, V> Index<&Q> for FlatMap<K, V>
where
    K: Borrow<Q>,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("no entry found for key")
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for i in iter {
            self.insert(i.0, i.1);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(mut value: Vec<(K, V)>) -> Self {
        value.sort_by(|a, b| a.0.cmp(&b.0));
        value.reverse();
        value.dedup_by(|a, b| a.0 == b.0);
        value.reverse();
        Self(value)
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(value: FlatMap<K, V>) -> Self {
        value.0
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<K: Ord, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = <Vec<(K, V)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
