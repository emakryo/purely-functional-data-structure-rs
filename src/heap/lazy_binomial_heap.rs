use std::cmp::Ordering;

use anyhow::{bail, Result};

use crate::{lazy, lazy::Lazy, stack::Cell, List};

use super::impl_heap;

#[derive(Clone)]
pub struct LazyBinomialHeap<T>(Lazy<List<Tree<T>>>);

#[derive(Clone)]
struct Tree<T> {
    rank: usize,
    val: T,
    children: List<Tree<T>>,
}

impl<T: Clone + PartialOrd + 'static> LazyBinomialHeap<T> {
    pub fn empty() -> Self {
        Self(lazy!(List::empty()))
    }
    pub fn is_empty(&self) -> bool {
        self.0.force().is_empty()
    }
    pub fn insert(&self, x: T) -> Self {
        let this = self.clone();
        Self(lazy!(ins_tree(
            Tree {
                rank: 0,
                val: x.clone(),
                children: List::empty()
            },
            this.0.force()
        )))
    }
    pub fn merge(&self, other: &Self) -> Self {
        let this = self.clone();
        let other = other.clone();
        Self(lazy!(merge(this.0.force(), other.0.force())))
    }
    pub fn find_min(&self) -> Option<T> {
        let (t, _) = remove_min_tree(self.0.force()).ok()?;
        Some(t.val)
    }
    pub fn delete_min(&self) -> Result<Self> {
        let this = self.clone();
        Ok(Self(lazy!({
            let (t, ts) = remove_min_tree(this.0.force()).unwrap();
            merge(t.children.reverse(), ts)
        })))
    }
}

fn link<T: Clone + PartialOrd>(t1: Tree<T>, t2: Tree<T>) -> Tree<T> {
    if t1.val <= t2.val {
        Tree {
            rank: t1.rank + 1,
            val: t1.val,
            children: List::cons(t2, t1.children),
        }
    } else {
        Tree {
            rank: t1.rank + 1,
            val: t2.val,
            children: List::cons(t1, t2.children),
        }
    }
}

fn ins_tree<T: Clone + PartialOrd>(t: Tree<T>, ts: List<Tree<T>>) -> List<Tree<T>> {
    match &*ts {
        Cell::Nil => List::cons(t, List::empty()),
        Cell::Cons(tp, tsp) => {
            if t.rank <= tp.rank {
                List::cons(t, ts)
            } else {
                ins_tree(link(t, tp.clone()), tsp.clone())
            }
        }
    }
}

fn merge<T: Clone + PartialOrd>(ts1: List<Tree<T>>, ts2: List<Tree<T>>) -> List<Tree<T>> {
    match (&*ts1, &*ts2) {
        (_, Cell::Nil) => ts1,
        (Cell::Nil, _) => ts2,
        (Cell::Cons(t1, ts1p), Cell::Cons(t2, ts2p)) => match t1.rank.cmp(&t2.rank) {
            Ordering::Less => List::cons(t1.clone(), merge(ts1p.clone(), ts2)),
            Ordering::Greater => List::cons(t2.clone(), merge(ts1, ts2p.clone())),
            Ordering::Equal => ins_tree(
                link(t1.clone(), t2.clone()),
                merge(ts1p.clone(), ts2p.clone()),
            ),
        },
    }
}

fn remove_min_tree<T: Clone + PartialOrd>(ts: List<Tree<T>>) -> Result<(Tree<T>, List<Tree<T>>)> {
    match &*ts {
        Cell::Nil => bail!("empty"),
        Cell::Cons(t, ts) => match &**ts {
            Cell::Nil => Ok((t.clone(), List::empty())),
            Cell::Cons(..) => {
                let (tp, tsp) = remove_min_tree(ts.clone())?;
                if t.val <= tp.val {
                    Ok((t.clone(), ts.clone()))
                } else {
                    Ok((tp, List::cons(t.clone(), tsp)))
                }
            }
        },
    }
}

impl_heap!(LazyBinomialHeap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let h = LazyBinomialHeap::empty();
        let h = h.insert(10);
        let h = h.insert(0);
        let h = h.insert(20);

        assert_eq!(h.find_min(), Some(0));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(10));
    }
}
