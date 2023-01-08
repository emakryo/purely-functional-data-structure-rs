use std::sync::Arc;

use anyhow::{bail, Result};

enum TreeImpl<T> {
    Empty,
    Node(T, Tree<T>, Tree<T>),
}

#[derive(Clone)]
pub struct Tree<T>(Arc<TreeImpl<T>>);

pub trait UnbalancedSet<T> {
    fn empty() -> Self;
    fn insert(&self, val: T) -> Self;
    fn member(&self, val: &T) -> bool;
}

impl<T: Clone + PartialOrd> UnbalancedSet<T> for Tree<T> {
    fn empty() -> Self {
        Self(TreeImpl::empty())
    }

    fn insert(&self, x: T) -> Self {
        // self.insert1(x)
        // match self.clone().insert_nodup1(x) {
        //     Ok(ret) => ret,
        //     _=> self
        // }

        Self(self.0.clone().insert_nodup2(x))
    }

    fn member(&self, x: &T) -> bool {
        self.0.member2(x)
    }
}

impl<T> From<TreeImpl<T>> for Tree<T> {
    fn from(x: TreeImpl<T>) -> Self {
        Self(Arc::new(x))
    }
}

impl<T> From<Arc<TreeImpl<T>>> for Tree<T> {
    fn from(x: Arc<TreeImpl<T>>) -> Self {
        Self(x)
    }
}

impl<T: Clone + PartialOrd> TreeImpl<T> {
    fn empty() -> Arc<Self> {
        Arc::new(Self::Empty)
    }
    /// list 2.9
    #[allow(dead_code)]
    fn insert1(self: Arc<Self>, x: T) -> Arc<Self> {
        match self.as_ref() {
            Self::Empty => Arc::new(Self::Node(x, self.clone().into(), self.into())),
            Self::Node(y, left, right) => {
                if &x < y {
                    Arc::new(Self::Node(
                        y.to_owned(),
                        left.clone().insert(x),
                        right.clone(),
                    ))
                } else if y < &x {
                    Arc::new(Self::Node(
                        y.to_owned(),
                        right.clone(),
                        left.clone().insert(x),
                    ))
                } else {
                    self
                }
            }
        }
    }

    /// exercise 2.3
    #[allow(dead_code)]
    fn insert_nodup1(self: Arc<Self>, x: T) -> Result<Arc<Self>> {
        match self.as_ref() {
            Self::Empty => Ok(Arc::new(Self::Node(x, self.clone().into(), self.into()))),
            Self::Node(y, left, right) => {
                if &x < y {
                    Ok(Arc::new(Self::Node(
                        y.to_owned(),
                        left.clone().insert(x),
                        right.clone(),
                    )))
                } else if y < &x {
                    Ok(Arc::new(Self::Node(
                        y.to_owned(),
                        left.clone(),
                        right.clone().insert(x),
                    )))
                } else {
                    bail!("Duplicated entry")
                }
            }
        }
    }

    /// exercise 2.4
    #[allow(dead_code)]
    fn insert_nodup2(self: Arc<Self>, x: T) -> Arc<Self> {
        fn rec<T: Clone + PartialOrd>(
            t: Arc<TreeImpl<T>>,
            x: T,
            z: Option<T>,
        ) -> Result<Arc<TreeImpl<T>>> {
            match t.as_ref() {
                TreeImpl::Empty if Some(x.clone()) == z => bail!("Duplicated entry"),
                TreeImpl::Empty => Ok(Arc::new(TreeImpl::Node(x, t.clone().into(), t.into()))),
                TreeImpl::Node(y, left, right) => {
                    if &x <= y {
                        Ok(Arc::new(TreeImpl::Node(
                            y.clone(),
                            rec(left.clone().0, x, Some(y.clone()))?.into(),
                            right.clone(),
                        )))
                    } else {
                        Ok(Arc::new(TreeImpl::Node(
                            y.clone(),
                            left.clone(),
                            rec(right.clone().0, x, z)?.into(),
                        )))
                    }
                }
            }
        }

        match rec(self.clone(), x, None) {
            Ok(ret) => ret,
            _ => self,
        }
    }

    /// list 2.9
    #[allow(dead_code)]
    fn member1(&self, x: &T) -> bool {
        match self {
            Self::Empty => false,
            Self::Node(y, left, right) => {
                if x < y {
                    left.member(x)
                } else if y < x {
                    right.member(x)
                } else {
                    true
                }
            }
        }
    }

    /// excercise 2.2
    #[allow(dead_code)]
    fn member2(&self, x: &T) -> bool {
        fn rec<T: Clone + PartialOrd>(t: &TreeImpl<T>, x: &T, z: Option<&T>) -> bool {
            match t {
                TreeImpl::Empty => Some(x) == z,
                TreeImpl::Node(y, left, right) => {
                    if x <= y {
                        rec(&left.0, x, Some(y))
                    } else {
                        rec(&right.0, x, z)
                    }
                }
            }
        }

        rec(self, x, None)
    }
}

impl<T: Clone + PartialOrd> Tree<T> {
    /// exercise 2.5 (a)
    pub fn complete(x: T, depth: usize) -> Self {
        if depth == 0 {
            Self::empty()
        } else {
            let c = Self::complete(x.clone(), depth - 1);
            TreeImpl::Node(x, c.clone(), c).into()
        }
    }

    /// exercise 2.5 (b)
    pub fn create(x: T, size: usize) -> Self {
        if size == 0 {
            Self::empty()
        } else {
            let (a, b) = Self::create2(x.clone(), size / 2);
            if size % 2 == 0 {
                TreeImpl::Node(x, a.clone(), a).into()
            } else {
                TreeImpl::Node(x, a, b).into()
            }
        }
    }

    fn create2(x: T, size: usize) -> (Self, Self) {
        match size {
            0 => {
                let e = Self::empty();
                (e.clone(), TreeImpl::Node(x, e.clone(), e).into())
            }
            _ if size % 2 == 0 => {
                let (a, b) = Self::create2(x.clone(), size / 2);
                (
                    TreeImpl::Node(x.clone(), a.clone(), a.clone()).into(),
                    TreeImpl::Node(x, a, b).into(),
                )
            }
            _ => {
                let (a, b) = Self::create2(x.clone(), size / 2);
                (
                    TreeImpl::Node(x.clone(), a, b.clone()).into(),
                    TreeImpl::Node(x, b.clone(), b).into(),
                )
            }
        }
    }
}

/// Exercise 2.6
pub trait FiniteMap<K, V> {
    fn empty() -> Self;
    fn bind(&self, key: K, value: V) -> Self;
    fn lookup(&self, key: K) -> Option<V>;
}

impl<K: Clone + PartialOrd, V: Clone> FiniteMap<K, V> for Tree<(K, V)> {
    fn empty() -> Self {
        Self(Arc::new(TreeImpl::Empty))
    }
    fn bind(&self, key: K, value: V) -> Self {
        match self.0.as_ref() {
            TreeImpl::Empty => TreeImpl::Node((key, value), self.clone(), self.clone()).into(),
            TreeImpl::Node((k2, _), left, right) if k2 == &key => {
                TreeImpl::Node((key, value), left.clone(), right.clone()).into()
            }
            TreeImpl::Node(e @ (k2, _), left, right) if k2 < &key => {
                TreeImpl::Node(e.clone(), left.clone(), right.clone().bind(key, value)).into()
            }
            TreeImpl::Node(e, left, right) => {
                TreeImpl::Node(e.clone(), left.clone().bind(key, value), right.clone()).into()
            }
        }
    }
    fn lookup(&self, key: K) -> Option<V> {
        match self.0.as_ref() {
            TreeImpl::Empty => None,
            TreeImpl::Node((k, v), _, _) if k == &key => Some(v.to_owned()),
            TreeImpl::Node((k, _), _, right) if k < &key => right.lookup(key),
            TreeImpl::Node(_, left, _) => left.lookup(key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbalanced_set() {
        let x = 10;
        let y = 20;
        let z = 15;
        let t = <Tree<_> as UnbalancedSet<_>>::empty();
        let t = t.insert(x);
        let t = t.insert(y);

        assert!(t.member(&x));
        assert!(t.member(&y));
        assert!(!t.member(&z));
    }

    #[test]
    fn test_finite_map() {
        let one = (1, "one");
        let two = (2, "two");
        let thr = (3, "three");

        let t = <Tree<_> as FiniteMap<_, _>>::empty();
        let t = t.bind(one.0, one.1);
        let t = t.bind(thr.0, thr.1);

        assert_eq!(t.lookup(one.0), Some(one.1));
        assert_eq!(t.lookup(two.0), None);
        assert_eq!(t.lookup(thr.0), Some(thr.1));
    }
}
