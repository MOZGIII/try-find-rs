#![warn(missing_debug_implementations, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![feature(async_await)]
#![feature(try_trait)]

use std::ops::Try;

pub trait TryFindExt: Iterator {
    /// Applies function to the elements of iterator and returns
    /// the first non-none result or the first error.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use try_find::TryFindExt;
    ///
    /// let a = ["1", "2", "lol", "NaN", "5"];
    ///
    /// let is_my_num = |s: &str, search: i32| -> Result<bool, std::num::ParseIntError> {
    ///     Ok(s.parse::<i32>()?  == search)
    /// };
    ///
    /// let result = a.iter().try_find(|&&s| is_my_num(s, 2));
    /// assert_eq!(result, Ok(Some(&"2")));
    ///
    /// let result = a.iter().try_find(|&&s| is_my_num(s, 5));
    /// assert!(result.is_err());
    /// ```
    fn try_find<E, F, R>(&mut self, mut f: F) -> Result<Option<Self::Item>, E>
    where
        Self: Sized,
        R: Try<Ok = bool, Error = E>,
        F: FnMut(&Self::Item) -> R,
    {
        let done = self.try_for_each(move |x| match f(&x).into_result() {
            Ok(false) => Ok(()),
            Ok(true) => Err(Ok(x)),
            Err(x) => Err(Err(x)),
        });
        match done {
            Ok(..) => None,
            Err(x) => Some(x),
        }
        .transpose()
    }
}

impl<I: Iterator<Item = Item>, Item> TryFindExt for I {}

#[test]
fn test_try_find() {
    let xs: &[isize] = &[];
    assert_eq!(xs.iter().try_find(testfn), Ok(None));
    let xs: &[isize] = &[1, 2, 3, 4];
    assert_eq!(xs.iter().try_find(testfn), Ok(Some(&2)));
    let xs: &[isize] = &[1, 3, 4];
    assert_eq!(xs.iter().try_find(testfn), Err(()));

    let xs: &[isize] = &[1, 2, 3, 4, 5, 6, 7];
    let mut iter = xs.iter();
    assert_eq!(iter.try_find(testfn), Ok(Some(&2)));
    assert_eq!(iter.try_find(testfn), Err(()));
    assert_eq!(iter.next(), Some(&5));

    fn testfn(x: &&isize) -> Result<bool, ()> {
        if **x == 2 {
            return Ok(true);
        }
        if **x == 4 {
            return Err(());
        }
        Ok(false)
    }
}

