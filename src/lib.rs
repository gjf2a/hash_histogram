//! # Overview
//! This library provides `struct HashHistogram`. It wraps `HashMap` to provide a straightforward histogram facility.
//!
//! ```
//! use hash_histogram::HashHistogram;
//!
//! // Record and inspect histogram counts.
//!
//! let mut h = HashHistogram::new();
//! for s in ["a", "b", "a", "b", "c", "b", "a", "b"].iter() {
//!     h.bump(s);
//! }
//!
//! for (s, c) in [("a", 3), ("b", 4), ("c", 1), ("d", 0)].iter() {
//!     assert_eq!(h.count(s), *c);
//! }
//!
//! assert_eq!(h.total_count(), 8);
//!
//! // Iteration
//! let mut iterated: Vec<(&str,usize)> = h.iter().map(|(s,c)| (*s, *c)).collect();
//! iterated.sort();
//! assert_eq!(iterated, vec![("a", 3), ("b", 4), ("c", 1)]);
//!
//! // Ranked ordering
//! assert_eq!(h.ranking(), vec!["b", "a", "c"]);
//!
//! // Mode
//! assert_eq!(h.mode(), Some(("b", 4)));
//! ```
//!
//! `HashHistogram` implements the `FromIterator` and `Extend` traits:
//! ```
//! use hash_histogram::HashHistogram;
//! // Initialization from an iterator:
//! let mut h: HashHistogram<&str> = ["a", "b", "a", "b", "c", "b", "a", "b"].iter().collect();
//! for (s, c) in [("a", 3), ("b", 4), ("c", 1), ("d", 0)].iter() {
//!     assert_eq!(h.count(s), *c);
//! }
//!
//! h.extend(["b", "d", "b", "e", "b"].iter());
//!
//! for (s, c) in [("a", 3), ("b", 7), ("c", 1), ("d", 1), ("e", 1), ("f", 0)].iter() {
//!     assert_eq!(h.count(s), *c);
//! }
//! ```
//!
//! Calculating the mode is sufficiently useful on its own that the `mode()` function is provided.
//! It uses a `HashHistogram` to calculate a mode from an object of any type that has the
//! `IntoIterator` trait:
//!
//! ```
//! use hash_histogram::mode;
//! let nums = vec!["a", "b", "c", "d", "a", "b", "a"];
//!
//! // Directly passing the container.
//! assert_eq!(mode(&nums).unwrap(), ("a", 3));
//!
//! // Passing an iterator from the container.
//! assert_eq!(mode(nums.iter()).unwrap(), ("a", 3));
//! ```

//    Copyright 2021, Gabriel J. Ferrer
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

use core::fmt;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::collections::hash_map::Iter;

// From https://stackoverflow.com/questions/26070559/is-there-any-way-to-create-a-type-alias-for-multiple-traits
pub trait KeyType: Hash + Clone + Eq {}
impl <T: Hash + Clone + Eq> KeyType for T {}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct HashHistogram<T:KeyType> {
    histogram: HashMap<T,usize>
}

impl <T:KeyType> HashHistogram<T> {
    pub fn new() -> Self { HashHistogram { histogram: HashMap::new()}}

    pub fn bump(&mut self, item: &T) {
        match self.histogram.get_mut(item) {
            None => {self.histogram.insert(item.clone(), 1);}
            Some(count) => {*count += 1}
        };
    }

    pub fn len(&self) -> usize {
        self.histogram.len()
    }

    pub fn count(&self, item: &T) -> usize {
        *self.histogram.get(item).unwrap_or(&0)
    }

    pub fn iter(&self) -> Iter<T,usize> {
        self.histogram.iter()
    }

    pub fn all_labels(&self) -> HashSet<T> {
        self.iter()
            .map(|(k, _)| k.clone())
            .collect()
    }

    pub fn ranking(&self) -> Vec<T> {
        let mut ranking: Vec<(usize,T)> = self.iter().map(|(t, n)| (*n, t.clone())).collect();
        ranking.sort_by_key(|(n,_)| -(*n as isize));
        ranking.iter().map(|(_,t)| t.clone()).collect()
    }

    pub fn mode(&self) -> Option<(T,usize)> {
        self.iter()
            .max_by_key(|(_,count)| **count)
            .map(|(key, count)| (key.clone(), *count))
    }

    pub fn total_count(&self) -> usize {
        self.iter().map(|(_,value)| value).sum()
    }
}

impl<T: KeyType + std::cmp::Ord + fmt::Display> fmt::Display for HashHistogram<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut in_order: Vec<T> = self.iter().map(|(k,_)| k).cloned().collect();
        in_order.sort();
        for label in in_order {
            write!(f, "{}:{}; ", label, self.count(&label))?;
        }
        Ok(())
    }
}

impl <'a, T: 'a + KeyType> FromIterator<&'a T> for HashHistogram<T> {
    fn from_iter<V: IntoIterator<Item=&'a T>>(iter: V) -> Self {
        let mut result = HashHistogram::new();
        for value in iter {
            result.bump(value);
        }
        result
    }
}

impl <'a, T: 'a + KeyType> Extend<&'a T> for HashHistogram<T> {
    fn extend<V: IntoIterator<Item=&'a T>>(&mut self, iter: V) {
        for value in iter {
            self.bump(value);
        }
    }
}

// Future idea:
//
// https://stackoverflow.com/questions/30540766/how-can-i-add-new-methods-to-iterator
//
pub fn mode<'a, T: 'a + KeyType, C: IntoIterator<Item=&'a T>>(container: C) -> Option<(T, usize)> {
    let mut counts: HashHistogram<T> = HashHistogram::new();
    for item in container.into_iter() {
        counts.bump(item);
    }
    counts.mode()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hist() {
        let mut hist = HashHistogram::new();
        let zeros = 10;
        let ones = 15;
        let twos = 20;

        for _ in 0..zeros {
            hist.bump(&0);
        }

        for _ in 0..ones {
            hist.bump(&1);
        }

        for _ in 0..twos {
            hist.bump(&2);
        }

        assert_eq!(3, hist.all_labels().len());
        assert_eq!(zeros, hist.count(&0));
        assert_eq!(ones, hist.count(&1));
        assert_eq!(twos, hist.count(&2));
        assert_eq!((2, 20), hist.mode().unwrap());
        assert_eq!(zeros + ones + twos, hist.total_count());
    }
}

