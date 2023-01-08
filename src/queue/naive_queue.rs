use crate::List;

#[derive(Clone)]
pub struct BatchedQueue<T>(List<T>, List<T>);

impl<T: Clone> BatchedQueue<T> {
    pub fn empty() -> Self {
        let e = List::empty();
        Self(e.clone(), e)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn checkf(self) -> Self {
        if self.0.is_empty() {
            Self(self.1.reverse(), List::empty())
        } else {
            self
        }
    }

    pub fn snoc(&self, x: T) -> Self {
        Self(self.0.clone(), List::cons(x, self.1.clone())).checkf()
    }

    pub fn head(&self) -> Option<T> {
        self.0.head()
    }

    pub fn tail(&self) -> Option<Self> {
        Some(Self(self.0.clone().tail()?, self.1.clone()).checkf())
    }
}

crate::queue::impl_queue!(BatchedQueue);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let xs = [1, 3, 2, 0, 4];
        let q = xs.iter().fold(BatchedQueue::empty(), |q, &x| q.snoc(x));

        assert_eq!(q.head(), Some(1));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(3));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(2));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(0));
        let q = q.tail().unwrap();
        assert_eq!(q.head(), Some(4));
        let q = q.tail().unwrap();
        assert!(q.is_empty());
    }
}
