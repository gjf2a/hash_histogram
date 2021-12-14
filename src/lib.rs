//! # Overview
//! This library provides `struct HashHistogram`. It wraps `HashMap` to provide a
//! straightforward histogram facility.
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
//! assert_eq!(h.mode(), Some("b"));
//!
//! // Incrementing larger counts
//! for (s, count) in [("a", 2), ("b", 3), ("c", 10)].iter() {
//!     h.bump_by(s, *count);
//! }
//!
//! for (s, count) in [("a", 5), ("b", 7), ("c", 11)].iter() {
//!     assert_eq!(h.count(s), *count);
//! }
//! ```
//!
//! Calculating the mode is sufficiently useful on its own that the `mode()` and `mode_values()`
//! functions are provided. Use `mode()` with iterators containing references to values in
//! containers, and `mode_values()` for iterators that own the values they return.
//!
//! They each use a `HashHistogram` to calculate a mode from an object of any type that has the
//! `IntoIterator` trait:
//!
//! ```
//! use hash_histogram::{mode, mode_values};
//! let chars = vec!["a", "b", "c", "d", "a", "b", "a"];
//!
//! // Directly passing the container.
//! assert_eq!(mode(&chars).unwrap(), "a");
//!
//! // Passing an iterator from the container.
//! assert_eq!(mode(chars.iter()).unwrap(), "a");
//!
//! // Use mode_values() when using an iterator generating values in place.
//! let nums = vec![100, 200, 100, 200, 300, 200, 100, 200];
//! assert_eq!(mode_values(nums.iter().map(|n| n + 1)).unwrap(), 201);
//! ```
//!
//! `HashHistogram` supports common Rust data structure operations. It implements the
//! `FromIterator` and `Extend` traits, and derives `serde`:
//! ```
//! use hash_histogram::HashHistogram;
//!
//! // Initialization from an iterator:
//! let mut h: HashHistogram<isize> = [100, 200, 100, 200, 300, 200, 100, 200].iter().collect();
//!
//! // Extension from an iterator
//! h.extend([200, 400, 200, 500, 200].iter());
//!
//! // Serialization
//! let serialized = serde_json::to_string(&h).unwrap();
//!
//! // Deserialization
//! let deserialized: HashHistogram<isize> = serde_json::from_str(&serialized).unwrap();
//! assert_eq!(deserialized, h);
//! ```
//!

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
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

// From https://stackoverflow.com/questions/26070559/is-there-any-way-to-create-a-type-alias-for-multiple-traits
pub trait KeyType: Debug + Hash + Clone + Eq {}
impl <T: Debug + Hash + Clone + Eq> KeyType for T {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct HashHistogram<T:KeyType> {
    histogram: HashMap<T,usize>
}

impl <T:KeyType> HashHistogram<T> {
    pub fn new() -> Self { HashHistogram { histogram: HashMap::new()}}

    pub fn bump(&mut self, item: &T) {
        self.bump_by(item, 1);
    }

    pub fn bump_by(&mut self, item: &T, increment: usize) {
        match self.histogram.get_mut(item) {
            None => {self.histogram.insert(item.clone(), 1);}
            Some(count) => {*count += increment;}
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

    pub fn mode(&self) -> Option<T> {
        self.iter()
            .max_by_key(|(_,count)| **count)
            .map(|(key, _)| key.clone())
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

impl <T: KeyType> FromIterator<T> for HashHistogram<T> {
    fn from_iter<V: IntoIterator<Item=T>>(iter: V) -> Self {
        let mut result = HashHistogram::new();
        for value in iter {
            result.bump(&value);
        }
        result
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
pub fn mode<'a, T: 'a + KeyType, C: IntoIterator<Item=&'a T>>(container: C) -> Option<T> {
    container.into_iter().collect::<HashHistogram<T>>().mode()
}

pub fn mode_values<T: KeyType, C: IntoIterator<Item=T>>(container: C) -> Option<T> {
    container.into_iter().collect::<HashHistogram<T>>().mode()
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
        assert_eq!(2, hist.mode().unwrap());
        assert_eq!(zeros + ones + twos, hist.total_count());
    }
}

