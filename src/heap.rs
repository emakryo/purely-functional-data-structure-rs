pub mod binomial_heap;
pub mod lazy_binomial_heap;
pub mod lazy_pairing_heap;
pub mod leftist_heap;
pub mod pairing_heap;
pub mod scheduled_binomial_heap;
pub mod splay_heap;

use anyhow::Result;

use crate::{stack::Cell, List};

pub trait Heap<T>
where
    Self: Sized + Clone,
    T: Clone,
{
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn insert(&self, val: T) -> Self;
    fn merge(&self, other: &Self) -> Self;
    fn find_min(&self) -> Option<T>;
    fn delete_min(&self) -> Result<Self>;

    fn from_list(xs: List<T>) -> Self {
        fn rec<T: Clone, H: Heap<T>>(xs: List<H>, ys: List<H>) -> H {
            match &*xs {
                Cell::Nil => rec(ys, List::empty()),
                Cell::Cons(x1, xs) => match &**xs {
                    Cell::Nil => {
                        if ys.is_empty() {
                            x1.clone()
                        } else {
                            rec(xs.clone(), List::cons(x1.clone(), ys))
                        }
                    }
                    Cell::Cons(x2, xs) => rec(xs.clone(), List::cons(x1.merge(x2), ys)),
                },
            }
        }

        rec(xs.map(|x| Self::empty().insert(x)), List::empty())
    }
}

macro_rules! impl_heap {
    ($h:ident) => {
        impl<T: Clone + PartialOrd + 'static> $crate::heap::Heap<T> for $h<T> {
            fn empty() -> Self {
                $h::empty()
            }
            fn is_empty(&self) -> bool {
                $h::is_empty(self)
            }
            fn insert(&self, x: T) -> Self {
                $h::insert(self, x)
            }
            fn merge(&self, other: &Self) -> Self {
                $h::merge(self, other)
            }
            fn find_min(&self) -> Option<T> {
                $h::find_min(self)
            }
            fn delete_min(&self) -> Result<Self> {
                $h::delete_min(self)
            }
        }
    };
}

pub(crate) use impl_heap;
