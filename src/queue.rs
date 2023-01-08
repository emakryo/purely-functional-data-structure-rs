pub mod bankers_queue;
pub mod hood_melville_queue;
pub mod naive_queue;
pub mod physicists_queue;
pub mod real_time_queue;

pub trait Queue<T>
where
    Self: Sized,
{
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn snoc(&self, x: T) -> Self;
    fn head(&self) -> Option<T>;
    fn tail(&self) -> Option<Self>;
}

macro_rules! impl_queue {
    ($q:ident) => {
        impl<T: Clone + 'static> $crate::queue::Queue<T> for $q<T> {
            fn empty() -> Self {
                $q::empty()
            }
            fn is_empty(&self) -> bool {
                $q::is_empty(self)
            }
            fn snoc(&self, x: T) -> Self {
                $q::snoc(self, x)
            }
            fn head(&self) -> Option<T> {
                $q::head(self)
            }
            fn tail(&self) -> Option<Self> {
                $q::tail(self)
            }
        }
    };
}

pub(crate) use impl_queue;
