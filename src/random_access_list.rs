pub mod binary_random_access_list;
pub mod skew_binary_random_access_list;

use anyhow::Result;

pub trait RandomAceessList<T>
where
    Self: Sized,
{
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(x: T, xs: &Self) -> Self;
    fn head(&self) -> Option<T>;
    fn tail(&self) -> Option<Self>;
    fn lookup(&self, idx: usize) -> Option<T>;
    fn update(&self, idx: usize, val: T) -> Result<Self>;
}

macro_rules! impl_ralist {
    ($l:ident) => {
        impl<T: Clone> $crate::random_access_list::RandomAceessList<T> for $l<T> {
            fn empty() -> Self {
                $l::empty()
            }
            fn is_empty(&self) -> bool {
                $l::is_empty(self)
            }
            fn cons(x: T, xs: &Self) -> Self {
                $l::cons(x, xs)
            }
            fn head(&self) -> Option<T> {
                $l::head(self)
            }
            fn tail(&self) -> Option<Self> {
                $l::tail(self)
            }
            fn lookup(&self, idx: usize) -> Option<T> {
                $l::lookup(self, idx)
            }
            fn update(&self, idx: usize, val: T) -> Result<Self> {
                $l::update(self, idx, val)
            }
        }
    };
}

pub(crate) use impl_ralist;
