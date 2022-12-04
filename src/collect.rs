use std::fmt::Debug;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub trait MoreItertools : Itertools {
    // Consumes the only element in the iterator, returning an error if iterator does not contain
    // exactly one element. See also Itertools::exactly_one() which this wraps.
    fn drain_only(self) -> Result<Self::Item>
        where Self: Sized, <Self as Iterator>::Item: Debug,
    {
        self.exactly_one().map_err(|e| anyhow!("Unexpected contents: {:?}", e.collect::<Vec<_>>()))
    }
}
impl<T: ?Sized> MoreItertools for T where T: Iterator { }

pub trait MoreIntoIterator : IntoIterator {
    // Consumes a collection and returns its only element. See also Itertools::exactly_one().
    fn take_only(self) -> Result<Self::Item>
        where Self: Sized, <Self as IntoIterator>::Item: Debug
    {
        self.into_iter().drain_only()
    }
}
impl<T: ?Sized> MoreIntoIterator for T where T: IntoIterator { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drain_only_test() {
        assert_eq!((1..1).drain_only().unwrap_err().to_string(), "Unexpected contents: []");
        assert_eq!((1..=1).drain_only().unwrap(), 1);
        assert_eq!((1..=3).drain_only().unwrap_err().to_string(), "Unexpected contents: [1, 2, 3]");
    }

    #[test]
    fn take_only_test() {
        let empty: &[i32] = &[];
        assert_eq!(empty.take_only().unwrap_err().to_string(), "Unexpected contents: []");
        assert_eq!(&[1].take_only().unwrap(), &1);
        assert_eq!(&[1, 2, 3].take_only().unwrap_err().to_string(), "Unexpected contents: [1, 2, 3]");
    }
}