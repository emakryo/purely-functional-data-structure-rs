use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RedBlackTree<T>(Arc<Tree<T>>);

#[derive(Debug)]
enum Tree<T> {
    Empty,
    Node {
        color: Color,
        elem: T,
        left: RedBlackTree<T>,
        right: RedBlackTree<T>,
    },
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

impl<T> From<Tree<T>> for RedBlackTree<T> {
    fn from(x: Tree<T>) -> Self {
        Self(Arc::new(x))
    }
}

impl<T> From<Arc<Tree<T>>> for RedBlackTree<T> {
    fn from(x: Arc<Tree<T>>) -> Self {
        Self(x)
    }
}

impl<T: Clone + PartialOrd> RedBlackTree<T> {
    pub fn empty() -> Self {
        Tree::Empty.into()
    }
    pub fn member(&self, x: T) -> bool {
        match self.0.as_ref() {
            Tree::Empty => false,
            Tree::Node {
                elem: y,
                left,
                right,
                ..
            } => {
                if &x < y {
                    left.member(x)
                } else if y < &x {
                    right.member(x)
                } else {
                    true
                }
            }
        }
    }
    pub fn insert(&self, x: T) -> Self {
        if let Tree::Node {
            elem, left, right, ..
        } = self.0.clone().ins(x).as_ref()
        {
            Tree::node(Color::Black, elem.clone(), left.clone(), right.clone()).into()
        } else {
            unreachable!()
        }
    }
}

impl<T: Clone + PartialOrd> Tree<T> {
    fn node(color: Color, x: T, left: RedBlackTree<T>, right: RedBlackTree<T>) -> Arc<Self> {
        Arc::new(Self::Node {
            color,
            elem: x,
            left,
            right,
        })
    }
    fn ins(self: Arc<Self>, x: T) -> Arc<Self> {
        match self.as_ref() {
            Tree::Empty => Tree::node(Color::Red, x, Tree::Empty.into(), Tree::Empty.into()),
            Tree::Node {
                color,
                elem,
                left: a,
                right: b,
            } => {
                if &x < elem {
                    Tree::node(*color, elem.clone(), a.0.clone().ins(x).into(), b.clone()).balance()
                } else if elem < &x {
                    Tree::node(*color, elem.clone(), a.clone(), b.0.clone().ins(x).into()).balance()
                } else {
                    self
                }
            }
        }
    }

    fn balance(self: Arc<Self>) -> Arc<Self> {
        match self.as_ref() {
            Self::Node {
                color: Color::Black,
                elem: z,
                left: a,
                right: d,
            } => match (a.0.as_ref(), d.0.as_ref()) {
                (
                    Self::Node {
                        color: Color::Red,
                        elem: y,
                        left: a,
                        right: c,
                    },
                    _,
                ) => match (a.0.as_ref(), c.0.as_ref()) {
                    (
                        Self::Node {
                            color: Color::Red,
                            elem: x,
                            left: a,
                            right: b,
                        },
                        _,
                    ) => Self::node(
                        Color::Red,
                        y.clone(),
                        Self::node(Color::Black, x.clone(), a.clone(), b.clone()).into(),
                        Self::node(Color::Black, z.clone(), c.clone(), d.clone()).into(),
                    ),
                    (
                        _,
                        Self::Node {
                            color: Color::Red,
                            elem: x,
                            left: b,
                            right: c,
                        },
                    ) => Self::node(
                        Color::Red,
                        x.clone(),
                        Self::node(Color::Black, y.clone(), a.clone(), b.clone()).into(),
                        Self::node(Color::Black, z.clone(), c.clone(), d.clone()).into(),
                    ),
                    _ => self,
                },
                (
                    _,
                    Self::Node {
                        color: Color::Red,
                        elem: y,
                        left: b,
                        right: d,
                    },
                ) => match (b.0.as_ref(), d.0.as_ref()) {
                    (
                        Self::Node {
                            color: Color::Red,
                            elem: x,
                            left: b,
                            right: c,
                        },
                        _,
                    ) => Self::node(
                        Color::Red,
                        x.clone(),
                        Self::node(Color::Black, z.clone(), a.clone(), b.clone()).into(),
                        Self::node(Color::Black, y.clone(), c.clone(), d.clone()).into(),
                    ),
                    (
                        _,
                        Self::Node {
                            color: Color::Red,
                            elem: x,
                            left: c,
                            right: d,
                        },
                    ) => Self::node(
                        Color::Red,
                        y.clone(),
                        Self::node(Color::Black, z.clone(), a.clone(), b.clone()).into(),
                        Self::node(Color::Black, x.clone(), c.clone(), d.clone()).into(),
                    ),
                    _ => self,
                },
                _ => self,
            },
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let xs = [6, 2, 4, 8];
        let mut s = RedBlackTree::empty();
        for &x in &xs {
            s = s.insert(x);
        }

        for &x in &xs {
            assert!(s.member(x));
            assert!(!s.member(x + 1));
        }
    }
}
