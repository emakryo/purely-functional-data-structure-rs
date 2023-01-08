pub mod bankers_deque;
pub mod native_deque;
pub mod real_time_deque;

pub trait Deque<T>
where
    Self: Sized,
{
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(x: T, xs: &Self) -> Self;
    fn head(&self) -> Option<T>;
    fn tail(&self) -> Option<Self>;
    fn snoc(&self, x: T) -> Self;
    fn last(&self) -> Option<T>;
    fn init(&self) -> Option<Self>;
}

macro_rules! impl_deque {
    ($d:ident) => {
        impl<T: Clone + 'static> $crate::deque::Deque<T> for $d<T> {
            fn empty() -> Self {
                $d::empty()
            }
            fn is_empty(&self) -> bool {
                $d::is_empty(self)
            }
            fn cons(x: T, xs: &Self) -> Self {
                $d::cons(x, xs)
            }
            fn head(&self) -> Option<T> {
                $d::head(self)
            }
            fn tail(&self) -> Option<Self> {
                $d::tail(self)
            }
            fn snoc(&self, x: T) -> Self {
                $d::snoc(self, x)
            }
            fn last(&self) -> Option<T> {
                $d::last(self)
            }
            fn init(&self) -> Option<Self> {
                $d::init(self)
            }
        }
    };
}

pub(crate) use impl_deque;
