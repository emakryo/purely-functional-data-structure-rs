use std::sync::Arc;

use anyhow::{bail, Result};

use crate::{lazy, lazy::Lazy};

use super::impl_heap;

#[derive(Clone)]
pub struct LazyPairingHeap<T>(Arc<Heap<T>>);

#[derive(Clone)]
enum Heap<T> {
    Empty,
    Node(T, LazyPairingHeap<T>, Lazy<LazyPairingHeap<T>>),
}

impl<T> From<Heap<T>> for LazyPairingHeap<T> {
    fn from(x: Heap<T>) -> Self {
        Self(Arc::new(x))
    }
}

impl<T: Clone + PartialOrd + 'static> LazyPairingHeap<T> {
    pub fn empty() -> Self {
        Heap::Empty.into()
    }
    pub fn is_empty(&self) -> bool {
        matches!(self.0.as_ref(), Heap::Empty)
    }
    pub fn merge(&self, other: &Self) -> Self {
        match (self.0.as_ref(), other.0.as_ref()) {
            (Heap::Empty, _) => other.clone(),
            (_, Heap::Empty) => self.clone(),
            (Heap::Node(x, ..), Heap::Node(y, ..)) => {
                if x <= y {
                    link(self, other).into()
                } else {
                    link(other, self).into()
                }
            }
        }
    }
    pub fn insert(&self, x: T) -> Self {
        Self::from(Heap::Node(x, Self::empty(), lazy!(Self::empty()))).merge(self)
    }
    pub fn find_min(&self) -> Option<T> {
        match self.0.as_ref() {
            Heap::Empty => None,
            Heap::Node(x, ..) => Some(x.clone()),
        }
    }
    pub fn delete_min(&self) -> Result<Self> {
        match self.0.as_ref() {
            Heap::Empty => bail!("empty"),
            Heap::Node(_, a, b) => Ok(a.merge(&b.force())),
        }
    }
}

fn link<T: Clone + PartialOrd + 'static>(
    b: &LazyPairingHeap<T>,
    a: &LazyPairingHeap<T>,
) -> Heap<T> {
    let a = a.clone();
    match b.0.as_ref() {
        Heap::Node(x, b, m) => {
            if b.is_empty() {
                Heap::Node(x.clone(), a, m.clone())
            } else {
                let b = b.clone();
                let m = m.clone();
                Heap::Node(
                    x.clone(),
                    LazyPairingHeap::empty(),
                    lazy!(a.merge(&b).merge(&m.force())),
                )
            }
        }
        _ => unreachable!(),
    }
}

impl_heap!(LazyPairingHeap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h = [3, 1, 4, 0, 2]
            .into_iter()
            .fold(LazyPairingHeap::empty(), |h, x| h.insert(x));

        assert_eq!(h.find_min(), Some(0));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(1));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(2));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(3));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(4));
        let h = h.delete_min().unwrap();
        assert!(h.find_min().is_none());
    }
}
