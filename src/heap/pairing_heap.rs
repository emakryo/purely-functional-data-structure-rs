use anyhow::{bail, Result};

use crate::{stack::Cell, List};
use std::sync::Arc;

use super::impl_heap;

#[derive(Clone)]
pub struct PairingHeap<T>(Arc<Tree<T>>);

#[derive(Clone)]
enum Tree<T> {
    Empty,
    Node(T, List<PairingHeap<T>>),
}

impl<T: Clone + PartialOrd> PairingHeap<T> {
    pub fn empty() -> Self {
        Self(Arc::new(Tree::Empty))
    }
    pub fn is_empty(&self) -> bool {
        matches!(self.0.as_ref(), Tree::Empty)
    }
    pub fn find_min(&self) -> Option<T> {
        match self.0.as_ref() {
            Tree::Empty => None,
            Tree::Node(x, _) => Some(x.clone()),
        }
    }
    fn node(x: T, hs: List<Self>) -> Self {
        Self(Arc::new(Tree::Node(x, hs)))
    }
    pub fn merge(&self, other: &Self) -> Self {
        match (self.0.as_ref(), other.0.as_ref()) {
            (Tree::Empty, _) => other.clone(),
            (_, Tree::Empty) => self.clone(),
            (Tree::Node(x, hs1), Tree::Node(y, hs2)) => {
                if x <= y {
                    Self::node(x.clone(), List::cons(other.clone(), hs1.clone()))
                } else {
                    Self::node(y.clone(), List::cons(self.clone(), hs2.clone()))
                }
            }
        }
    }
    pub fn insert(&self, x: T) -> Self {
        self.merge(&Self::node(x, List::empty()))
    }
    fn merge_pairs(hs: List<Self>) -> Self {
        match &*hs {
            Cell::Nil => Self::empty(),
            Cell::Cons(h1, hs) => match &**hs {
                Cell::Nil => h1.clone(),
                Cell::Cons(h2, hs) => h1.merge(h2).merge(&Self::merge_pairs(hs.clone())),
            },
        }
    }
    pub fn delete_min(&self) -> Result<Self> {
        match self.0.as_ref() {
            Tree::Empty => bail!("empty"),
            Tree::Node(_, hs) => Ok(Self::merge_pairs(hs.clone())),
        }
    }
}

impl_heap!(PairingHeap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h = [3, 1, 4, 0, 2]
            .into_iter()
            .fold(PairingHeap::empty(), |h, x| h.insert(x));

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
