use anyhow::{bail, Result};
use std::{cmp::Ordering, sync::Arc};

use crate::List;

use super::impl_heap;

struct Tree<T> {
    rank: usize,
    elem: T,
    children: List<Arc<Tree<T>>>,
}

impl<T: Clone + PartialOrd> Tree<T> {
    fn node(rank: usize, elem: T, children: List<Arc<Tree<T>>>) -> Arc<Tree<T>> {
        Arc::new(Tree {
            rank,
            elem,
            children,
        })
    }
    fn root(&self) -> T {
        self.elem.clone()
    }
    fn link(self: Arc<Self>, other: Arc<Self>) -> Arc<Tree<T>> {
        if self.elem <= other.elem {
            Self::node(
                self.rank + 1,
                self.elem.clone(),
                List::cons(other, self.children.clone()),
            )
        } else {
            Self::node(
                self.rank + 1,
                other.elem.clone(),
                List::cons(self, other.children.clone()),
            )
        }
    }
}

#[derive(Clone)]
pub struct BinomialHeap<T>(List<Arc<Tree<T>>>);

impl<T: Clone + PartialOrd> BinomialHeap<T> {
    pub fn empty() -> Self {
        Self(List::empty())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn ins_tree(&self, t: Arc<Tree<T>>) -> Self {
        if self.is_empty() {
            Self(List::cons(t, List::empty()))
        } else {
            let tp = self.0.head().unwrap();
            let tsp = self.0.tail().unwrap();
            if t.rank < tp.rank {
                Self(List::cons(t, self.0.clone()))
            } else {
                Self(tsp).ins_tree(t.link(tp))
            }
        }
    }
    pub fn insert(&self, x: T) -> Self {
        self.ins_tree(Tree::node(0, x, List::empty()))
    }
    pub fn merge(&self, other: &Self) -> Self {
        if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            let t1 = self.0.head().unwrap();
            let ts1 = self.0.tail().unwrap();
            let t2 = other.0.head().unwrap();
            let ts2 = other.0.tail().unwrap();
            match t1.rank.cmp(&t2.rank) {
                Ordering::Less => Self(List::cons(t1, other.merge(&Self(ts1)).0)),
                Ordering::Greater => Self(List::cons(t2, self.merge(&Self(ts2)).0)),
                Ordering::Equal => Self(ts1).merge(&Self(ts2)).ins_tree(t1.link(t2)),
            }
        }
    }
    fn remove_min_tree(&self) -> Result<(Arc<Tree<T>>, Self)> {
        if self.is_empty() {
            bail!("Empty");
        } else if self.0.tail().unwrap().is_empty() {
            Ok((self.0.head().unwrap(), Self::empty()))
        } else {
            let t = self.0.head().unwrap();
            let ts = self.0.tail().unwrap();
            let (tp, tsp) = Self(ts.clone()).remove_min_tree().unwrap();

            if t.root() <= tp.root() {
                Ok((t, Self(ts)))
            } else {
                Ok((tp, Self(List::cons(t, tsp.0))))
            }
        }
    }
    pub fn find_min(&self) -> Option<T> {
        self.remove_min_tree().ok().map(|(t, _)| t.root())
    }
    pub fn delete_min(&self) -> Result<Self> {
        let (t, ts2) = self.remove_min_tree()?;
        let ts1 = t.children.clone();
        Ok(Self(ts1.reverse()).merge(&ts2))
    }
}

impl_heap!(BinomialHeap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let h = BinomialHeap::empty();
        let h = h.insert(10);
        let h = h.insert(0);
        let h = h.insert(20);

        assert_eq!(h.find_min(), Some(0));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(10));
    }
}
