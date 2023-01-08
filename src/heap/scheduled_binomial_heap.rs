use anyhow::{bail, Result};

use crate::{
    lazy,
    lazy::{Stream, StreamCell},
    stack::Cell,
    List,
};

use super::impl_heap;

#[derive(Clone)]
pub struct ScheduledBinomialHeap<T>(Stream<Digit<T>>, Schedule<T>);

#[derive(Clone)]
struct Schedule<T>(List<Stream<Digit<T>>>);

#[derive(Clone)]
enum Digit<T> {
    Zero,
    One(Tree<T>),
}

#[derive(Clone)]
struct Tree<T>(T, List<Tree<T>>);

impl<T: Clone + PartialOrd + 'static> ScheduledBinomialHeap<T> {
    pub fn empty() -> Self {
        Self(Stream::empty(), Schedule(List::empty()))
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty().force()
    }
    pub fn insert(&self, x: T) -> Self {
        let dsp = ins_tree(Tree(x, List::empty()), self.0.clone());
        Self(
            dsp.clone(),
            Schedule(List::cons(dsp, self.1 .0.clone())).exec().exec(),
        )
    }
    pub fn merge(&self, other: &Self) -> Self {
        let ds = normalize(merge(self.0.clone(), other.0.clone()));
        Self(ds, Schedule(List::empty()))
    }
    pub fn find_min(&self) -> Option<T> {
        remove_min_tree(self.0.clone()).ok().map(|(t, _)| t.0)
    }
    pub fn delete_min(&self) -> Result<Self> {
        let (t, ds) = remove_min_tree(self.0.clone())?;
        let dsp = merge(t.1.reverse().map(Digit::One).to_stream(), ds);
        Ok(Self(normalize(dsp), Schedule(List::empty())))
    }
}

fn link<T: Clone + PartialOrd>(t1: Tree<T>, t2: Tree<T>) -> Tree<T> {
    if t1.0 <= t2.0 {
        Tree(t1.0.clone(), List::cons(t2, t1.1))
    } else {
        Tree(t2.0.clone(), List::cons(t1, t2.1))
    }
}

fn ins_tree<T: Clone + PartialOrd + 'static>(t: Tree<T>, ds: Stream<Digit<T>>) -> Stream<Digit<T>> {
    match ds.force() {
        StreamCell::Nil => Stream::cons(lazy!((Digit::One(t), Stream::empty()))),
        StreamCell::Cons(d, ds) => match d {
            Digit::Zero => Stream::cons(lazy!((Digit::One(t), ds))),
            Digit::One(tp) => Stream::cons(lazy!((Digit::Zero, ins_tree(link(t, tp), ds)))),
        },
    }
}

fn merge<T: Clone + PartialOrd + 'static>(
    ds1: Stream<Digit<T>>,
    ds2: Stream<Digit<T>>,
) -> Stream<Digit<T>> {
    match (ds1.force(), ds2.force()) {
        (StreamCell::Nil, _) => ds1,
        (_, StreamCell::Nil) => ds2,
        (StreamCell::Cons(Digit::Zero, ds1), StreamCell::Cons(d, ds2)) => {
            Stream::cons(lazy!((d, merge(ds1, ds2))))
        }
        (StreamCell::Cons(d, ds1), StreamCell::Cons(Digit::Zero, ds2)) => {
            Stream::cons(lazy!((d, merge(ds1, ds2))))
        }
        (StreamCell::Cons(Digit::One(t1), ds1), StreamCell::Cons(Digit::One(t2), ds2)) => {
            Stream::cons(lazy!((
                Digit::Zero,
                ins_tree(link(t1, t2), merge(ds1, ds2))
            )))
        }
    }
}

fn normalize<T: Clone + 'static>(ds: Stream<T>) -> Stream<T> {
    if let StreamCell::Cons(_, dsp) = ds.force() {
        normalize(dsp);
    }
    ds
}

impl<T: Clone> Schedule<T> {
    fn exec(self) -> Self {
        Self(match &*self.0 {
            Cell::Nil => List::empty(),
            Cell::Cons(ds, sched) => match ds.force() {
                StreamCell::Cons(Digit::Zero, job) => List::cons(job, sched.clone()),
                _ => sched.clone(),
            },
        })
    }
}

fn remove_min_tree<T: Clone + PartialOrd + 'static>(
    ds: Stream<Digit<T>>,
) -> Result<(Tree<T>, Stream<Digit<T>>)> {
    match ds.force() {
        StreamCell::Nil => bail!("empty"),
        StreamCell::Cons(Digit::One(t), ds) if ds.is_empty().force() => Ok((t, Stream::empty())),
        StreamCell::Cons(Digit::Zero, ds) => {
            let (tp, dsp) = remove_min_tree(ds)?;
            Ok((tp, Stream::cons(lazy!((Digit::Zero, dsp)))))
        }
        StreamCell::Cons(Digit::One(t), ds) => {
            let (tp, dsp) = remove_min_tree(ds.clone())?;
            if t.0 <= tp.0 {
                Ok((t, Stream::cons(lazy!((Digit::Zero, ds)))))
            } else {
                Ok((tp, Stream::cons(lazy!((Digit::One(t), dsp)))))
            }
        }
    }
}

impl_heap!(ScheduledBinomialHeap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h = [3, 1, 4, 0, 2]
            .into_iter()
            .fold(ScheduledBinomialHeap::empty(), |h, x| h.insert(x));

        assert_eq!(h.find_min(), Some(0));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(1));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(2));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(3));
        let h = h.delete_min().unwrap();
        assert_eq!(h.find_min(), Some(4));
        let h = h.delete_min().unwrap();
        assert!(h.find_min().is_none());
    }
}
