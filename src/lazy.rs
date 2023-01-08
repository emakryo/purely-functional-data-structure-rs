use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

enum LazyCell<T> {
    Future(Box<dyn FnOnce() -> T>),
    Done(T),
    /// default value only for std::mem::take
    None,
}

// implment manually because derive impl required T to be default
impl<T> Default for LazyCell<T> {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone)]
pub struct Lazy<T>(Arc<Mutex<LazyCell<T>>>);

impl<T: Clone> Lazy<T> {
    pub fn new<F: FnOnce() -> T + 'static>(f: F) -> Self {
        Self(Arc::new(Mutex::new(LazyCell::Future(Box::new(f)))))
    }
    pub fn force(&self) -> T {
        let mtx = self.0.as_ref();
        let mut guard = mtx.lock().unwrap();
        let cell: LazyCell<T> = std::mem::take(&mut guard);
        let x = match cell {
            LazyCell::Future(f) => f(),
            LazyCell::Done(x) => x,
            _ => unreachable!(),
        };

        *guard = LazyCell::Done(x.clone());
        x
    }
}

#[macro_export]
macro_rules! lazy {
    ($e:expr) => {
        $crate::lazy::Lazy::new(move || $e)
    };
}

#[derive(Clone)]
pub enum StreamCell<T> {
    Nil,
    Cons(T, Stream<T>),
}

impl<T> Deref for Stream<T> {
    type Target = Lazy<StreamCell<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct Stream<T>(Lazy<StreamCell<T>>);

impl<T: Clone + 'static> Stream<T> {
    pub fn empty() -> Self {
        Self(lazy!(StreamCell::Nil))
    }

    pub fn is_empty(&self) -> Lazy<bool> {
        let this = self.to_owned();
        lazy!(matches!(this.0.force(), StreamCell::Nil))
    }

    pub fn cons(xxs: Lazy<(T, Self)>) -> Self {
        Self(lazy!({
            let (x, xs) = xxs.force();
            StreamCell::Cons(x, xs)
        }))
    }

    pub fn head(&self) -> Lazy<Option<T>> {
        let this = self.to_owned();
        lazy!(match this.0.force() {
            StreamCell::Nil => None,
            StreamCell::Cons(x, _) => Some(x),
        })
    }

    pub fn tail(&self) -> Self {
        let this = self.to_owned();
        Self(lazy!(match this.0.force() {
            StreamCell::Nil => panic!(),
            StreamCell::Cons(_, xs) => xs.0.force(),
        }))
    }

    pub fn extend(&self, other: &Self) -> Self {
        let this = self.to_owned();
        let other = other.to_owned();
        Self(lazy!(match this.0.force() {
            StreamCell::Nil => other.0.force(),
            StreamCell::Cons(x, xs) => StreamCell::Cons(x, xs.extend(&other)),
        }))
    }

    pub fn take(&self, n: usize) -> Self {
        let this = self.to_owned();
        Self(lazy!(match (this.0.force(), n) {
            (_, 0) => StreamCell::Nil,
            (StreamCell::Nil, _) => StreamCell::Nil,
            (StreamCell::Cons(x, xs), n) => StreamCell::Cons(x, xs.take(n - 1)),
        }))
    }

    pub fn drop(&self, n: usize) -> Self {
        let this = self.clone();
        Self(lazy!(match (this.0.force(), n) {
            (s, 0) => s,
            (StreamCell::Nil, _) => StreamCell::Nil,
            (StreamCell::Cons(_, xs), n) => xs.drop(n - 1).0.force(),
        }))
    }

    pub fn reverse(&self) -> Self {
        let this = self.clone();
        Self(lazy!({
            fn rev<T: Clone + 'static>(xs: Stream<T>, ys: Stream<T>) -> Stream<T> {
                match xs.0.force() {
                    StreamCell::Nil => ys,
                    StreamCell::Cons(x, xs) => rev(xs, Stream::cons(lazy!((x, ys)))),
                }
            }
            rev(this, Stream::empty()).0.force()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend() {
        let xs = Stream::cons(lazy!((0, Stream::cons(lazy!((2, Stream::empty()))))));
        let ys = Stream::cons(lazy!((3, Stream::cons(lazy!((1, Stream::empty()))))));

        let zs = xs.extend(&ys);

        assert_eq!(zs.head().force(), Some(0));
        let zs = zs.tail();
        assert_eq!(zs.head().force(), Some(2));
        let zs = zs.tail();
        assert_eq!(zs.head().force(), Some(3));
        let zs = zs.tail();
        assert_eq!(zs.head().force(), Some(1));
        let zs = zs.tail();
        assert!(zs.is_empty().force());
    }

    #[test]
    fn test_take() {
        let xs = [0, 2, 3, 1];
        let xs = xs
            .into_iter()
            .rev()
            .fold(Stream::empty(), |xs, x| Stream::cons(lazy!((x, xs))));

        let zs = xs.take(2);
        assert_eq!(zs.head().force(), Some(0));
        let zs = zs.tail();
        assert_eq!(zs.head().force(), Some(2));
        let zs = zs.tail();
        assert!(zs.is_empty().force());
    }

    #[test]
    fn test_drop() {
        let xs = [0, 2, 3, 1];
        let xs = xs
            .into_iter()
            .rev()
            .fold(Stream::empty(), |xs, x| Stream::cons(lazy!((x, xs))));

        let zs = xs.drop(2);
        assert_eq!(zs.head().force(), Some(3));
        let zs = zs.tail();
        assert_eq!(zs.head().force(), Some(1));
        let zs = zs.tail();
        assert!(zs.is_empty().force());
    }
}
