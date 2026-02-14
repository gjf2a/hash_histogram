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
//! // Iterating over counts only
//! let mut counts: Vec<usize> = h.counts().collect();
//! counts.sort();
//! assert_eq!(counts, vec![1, 3, 4]);
//!
//! // Ranked ordering
//! assert_eq!(h.ranking(), vec!["b", "a", "c"]);
//!
//! // Ranked ordering with counts
//! assert_eq!(h.ranking_with_counts(), vec![("b", 4), ("a", 3), ("c", 1)]);
//!
//! // Mode
//! assert_eq!(h.mode(), Some("b"));
//!
//! // Incrementing larger counts
//! for (s, count) in [("a", 2), ("b", 3), ("c", 10), ("d", 5)].iter() {
//!     h.bump_by(s, *count);
//! }
//!
//! for (s, count) in [("a", 5), ("b", 7), ("c", 11), ("d", 5)].iter() {
//!     assert_eq!(h.count(s), *count);
//! }
//! ```
//!
//! Counts can be of any numerical type, including floating-point values.
//! ```
//! use hash_histogram::HashHistogram;
//!
//! let mut h = HashHistogram::new();
//! for (s, weight) in [("a", 0.25), ("b", 0.5), ("a", 0.3), ("c", 0.4), ("b", 0.1)].iter() {
//!     h.bump_by(s, *weight);
//! }
//!
//! for (s, total) in [("a", 0.55), ("b", 0.6), ("c", 0.4)].iter() {
//!     assert_eq!(h.count(s), *total);
//! }
//!
//! assert_eq!(h.ranking_with_counts(), vec![("b", 0.6), ("a", 0.55), ("c", 0.4)]);
//! ```
//! 
//! Histograms can be normalized so that all their counts add up to a target value. 
//! In the doc-tests below, we use the `Decimal` type from the 
//! [rust_decimal](https://crates.io/crates/rust_decimal) crate to enable reliable assertions
//! of equality. The example would work similarly with `f64` counts.
//! ```
//! use hash_histogram::HashHistogram;
//! use rust_decimal_macros::dec;
//!
//! let mut h = HashHistogram::new();
//! for (s, weight) in [("a", dec!(0.6)), ("b", dec!(2.6)), ("c", dec!(1.0)), ("a", dec!(0.6)), ("c", dec!(0.8)), ("b", dec!(0.4))].iter() {
//!     h.bump_by(s, *weight);
//! }
//!
//! assert_eq!(h.ranking_with_counts(), vec![("b", dec!(3.0)), ("c", dec!(1.8)), ("a", dec!(1.2))]);
//! h.normalize(dec!(1.0));
//! assert_eq!(h.ranking_with_counts(), vec![("b", dec!(0.5)), ("c", dec!(0.3)), ("a", dec!(0.2))]);
//! ```
//! 
//! By selecting suitable target values, normalization can be useful with integer counts as well.
//! ```
//! use hash_histogram::HashHistogram;
//! 
//! let mut h = HashHistogram::new();
//! for s in ["a", "b", "a", "b", "c", "b", "a", "b", "c", "d"].iter() {
//!     h.bump(s);
//! }
//! 
//! h.normalize(100);
//! assert_eq!(h.ranking_with_counts(), vec![("b", 40), ("a", 30), ("c", 20), ("d", 10)]);
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
//! The counts from a `HashHistogram` can be interpreted as weights for a probability
//! distribution. The `pick_random_key()` method will select a key with a probability 
//! derived from the counts.
//! 
//! ```
//! use hash_histogram::HashHistogram;
//! 
//! let distro: HashHistogram<&str,usize> = ["a", "a", "a", "a", "b", "b"].iter().collect();
//! for _ in 0..100 {
//!     let mut counter: HashHistogram<&str, usize> = HashHistogram::default();
//!     for _ in 0..10000 {
//!         counter.bump(&distro.pick_random_key());
//!     }
//!     assert!(counter.count(&"a")> 6000 && counter.count(&"a") < 7000);
//!     assert!(counter.count(&"b")> 3000 && counter.count(&"b") < 4000);
//! }
//! ```
//! 
//! `HashHistogram` supports common Rust data structure operations. It implements the
//! `FromIterator` and `Extend` traits, and derives `serde`:
//! ```
//! use hash_histogram::HashHistogram;
//!
//! // Initialization from an iterator:
//! let mut h1: HashHistogram<isize> = [100, 200, 100, 200, 300, 200, 100, 200].iter().collect();
//! let mut h2: HashHistogram<isize, usize> = [(100, 2), (200, 3), (300, 1), (100, 1), (200, 1)].iter().copied().collect();
//! let expected_ranking = vec![(200, 4), (100, 3), (300, 1)];
//! assert_eq!(h1.ranking_with_counts(), expected_ranking);
//! assert_eq!(h2.ranking_with_counts(), expected_ranking);
//!
//! // Extension from an iterator
//! h1.extend([200, 300, 200, 400, 200].iter());
//! h2.extend([(200, 1), (300, 1), (400, 1), (200, 2)].iter());
//! let expected_ranking_extended = vec![(200, 7), (100, 3), (300, 2), (400, 1)];
//! assert_eq!(h1.ranking_with_counts(), expected_ranking_extended);
//! assert_eq!(h2.ranking_with_counts(), expected_ranking_extended);
//!
//! // Serialization
//! let serialized = serde_json::to_string(&h1).unwrap();
//!
//! // Deserialization
//! let deserialized: HashHistogram<isize> = serde_json::from_str(&serialized).unwrap();
//! assert_eq!(deserialized, h1);
//! ```
//!
//! We can combine histograms of the same type using the `+=` operator:
//! ```
//! use hash_histogram::HashHistogram;
//! let mut h1: HashHistogram<&str> = ["a", "b", "c", "d", "b", "d", "c", "b", "d"].iter().collect();
//! let h2: HashHistogram<&str> = ["e", "b", "c", "d", "d", "e"].iter().collect();
//!
//! h1 += &h2;
//! assert_eq!(h1.ranking_with_counts(), vec![("d", 5), ("b", 4), ("c", 3), ("e", 2), ("a", 1)]);
//! ```
//!

//    Copyright 2021-2026, Gabriel J. Ferrer
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
use num::{One, Zero};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Sum;
use std::ops::{AddAssign, Div};
use trait_set::trait_set;

trait_set! {
    pub trait KeyType = Debug + Hash + Clone + Eq;
    pub trait CounterType = Copy + Clone + One + Zero + AddAssign + PartialOrd + Sum + Default;
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct HashHistogram<T: KeyType, C: CounterType = usize> {
    histogram: HashMap<T, C>,
}

impl<T: KeyType, C: CounterType> Default for HashHistogram<T, C> {
    fn default() -> Self {
        Self {
            histogram: Default::default(),
        }
    }
}

impl<T: KeyType, C: CounterType> HashHistogram<T, C> {
    pub fn new() -> Self {
        HashHistogram::default()
    }

    pub fn bump(&mut self, item: &T) {
        self.bump_by(item, num::one());
    }

    pub fn bump_by(&mut self, item: &T, increment: C) {
        match self.histogram.get_mut(item) {
            None => {
                self.histogram.insert(item.clone(), increment);
            }
            Some(count) => {
                *count += increment;
            }
        };
    }

    pub fn count(&self, item: &T) -> C {
        *self.histogram.get(item).unwrap_or(&num::zero())
    }

    pub fn len(&self) -> usize {
        self.histogram.len()
    }

    pub fn iter(&self) -> Iter<'_, T, C> {
        self.histogram.iter()
    }

    pub fn counts(&self) -> impl Iterator<Item = C> + '_ {
        self.iter().map(|(_, c)| c).copied()
    }

    pub fn all_labels(&self) -> HashSet<T> {
        self.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn ranking(&self) -> Vec<T> {
        self.ranking_with_counts()
            .iter()
            .map(|(k, _)| k.clone())
            .collect()
    }

    pub fn ranking_with_counts(&self) -> Vec<(T, C)> {
        let mut ranking: Vec<(T, C)> = self.iter().map(|(t, n)| (t.clone(), *n)).collect();
        ranking.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).unwrap_or(Ordering::Equal));
        ranking
    }

    pub fn mode(&self) -> Option<T> {
        self.iter()
            .max_by(|(_, c1), (_, c2)| c1.partial_cmp(c2).unwrap_or(Ordering::Equal))
            .map(|(key, _)| key.clone())
    }

    pub fn total_count(&self) -> C {
        self.iter().map(|(_, value)| value).copied().sum::<C>()
    }
}

impl<T: KeyType, C: CounterType> AddAssign<&HashHistogram<T, C>> for HashHistogram<T, C> {
    fn add_assign(&mut self, rhs: &HashHistogram<T, C>) {
        for (key, count) in rhs.histogram.iter() {
            self.bump_by(key, *count);
        }
    }
}

impl<T: KeyType, C: CounterType + Div<Output = C>> HashHistogram<T,C> {
    /// Normalizes counts so that they add up to `target_total`.
    /// Because of rounding and truncation, they might not add up exactly
    /// to that total. 
    pub fn normalize(&mut self, target_total: C) {
        let total = self.total_count();
        for value in self.histogram.values_mut() {
            *value = *value * target_total / total;
        }
    }
}

impl<T: KeyType + std::cmp::Ord + fmt::Display, C: CounterType + fmt::Display> fmt::Display
    for HashHistogram<T, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut in_order: Vec<T> = self.iter().map(|(k, _)| k).cloned().collect();
        in_order.sort();
        for label in in_order {
            write!(f, "{}:{}; ", label, self.count(&label))?;
        }
        Ok(())
    }
}

impl<T: KeyType, C: CounterType> FromIterator<T> for HashHistogram<T, C> {
    fn from_iter<V: IntoIterator<Item = T>>(iter: V) -> Self {
        let mut result = HashHistogram::new();
        for value in iter {
            result.bump(&value);
        }
        result
    }
}

impl<T: KeyType, C: CounterType> FromIterator<(T, C)> for HashHistogram<T, C> {
    fn from_iter<V: IntoIterator<Item = (T, C)>>(iter: V) -> Self {
        let mut result = HashHistogram::new();
        for (key, value) in iter {
            result.bump_by(&key, value);
        }
        result
    }
}

impl<'a, T: 'a + KeyType, C: 'a + CounterType> FromIterator<&'a T> for HashHistogram<T, C> {
    fn from_iter<V: IntoIterator<Item = &'a T>>(iter: V) -> Self {
        let mut result = HashHistogram::new();
        for value in iter {
            result.bump(value);
        }
        result
    }
}

impl<'a, T: 'a + KeyType, C: 'a + CounterType> Extend<&'a T> for HashHistogram<T, C> {
    fn extend<V: IntoIterator<Item = &'a T>>(&mut self, iter: V) {
        for value in iter {
            self.bump(value);
        }
    }
}

impl<'a, T: 'a + KeyType, C: 'a + CounterType> Extend<&'a (T, C)> for HashHistogram<T, C> {
    fn extend<V: IntoIterator<Item = &'a (T, C)>>(&mut self, iter: V) {
        for (item, value) in iter {
            self.bump_by(item, *value);
        }
    }
}

pub trait RandomRanger : Copy + Clone {
    fn choice_from_range(total: Self) -> Self;
}

macro_rules! derive_random_ranger_int {
    ($datatype:tt) => {
        impl RandomRanger for $datatype {
            fn choice_from_range(total: Self) -> Self {
                rand::rng().random_range(0..total)
            }
        }
    };
}

macro_rules! derive_random_ranger_float {
    ($datatype:tt) => {
        impl RandomRanger for $datatype {
            fn choice_from_range(total: Self) -> Self {
                rand::random::<$datatype>() * total
            }
        }
    }
}

derive_random_ranger_int!(u8);
derive_random_ranger_int!(u16);
derive_random_ranger_int!(u32);
derive_random_ranger_int!(u64);
derive_random_ranger_int!(u128);
derive_random_ranger_int!(usize);
derive_random_ranger_int!(i8);
derive_random_ranger_int!(i16);
derive_random_ranger_int!(i32);
derive_random_ranger_int!(i64);
derive_random_ranger_int!(i128);

derive_random_ranger_float!(f32);
derive_random_ranger_float!(f64);

impl<T: KeyType, C: CounterType + RandomRanger> HashHistogram<T, C> {
    pub fn pick_random_key(&self) -> T {
        assert!(self.len() > 0);
        let choice = C::choice_from_range(self.total_count());
        let mut total = C::zero();
        for (k, w) in self.iter() {
            total += *w;
            if total > choice {
                return k.clone();
            }
        }
        panic!("This should never happen.");
    }
}

// Future idea:
//
// https://stackoverflow.com/questions/30540766/how-can-i-add-new-methods-to-iterator
//
pub fn mode<'a, T: 'a + KeyType, A: IntoIterator<Item = &'a T>>(container: A) -> Option<T> {
    container
        .into_iter()
        .collect::<HashHistogram<T, usize>>()
        .mode()
}

pub fn mode_values<T: KeyType, A: IntoIterator<Item = T>>(container: A) -> Option<T> {
    container
        .into_iter()
        .collect::<HashHistogram<T, usize>>()
        .mode()
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f64_near;

    use super::*;

    #[test]
    fn test_hist() {
        let mut hist = HashHistogram::<_, usize>::new();
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

    #[test]
    fn test_str_key() {
        let mut h: HashHistogram<&str, usize> = HashHistogram::new();
        for s in ["a", "b", "a", "b", "c", "b", "a", "b"].iter() {
            h.bump(s);
        }

        for (s, c) in [("a", 3), ("b", 4), ("c", 1), ("d", 0)].iter() {
            assert_eq!(h.count(s), *c);
        }

        assert_eq!(h.total_count(), 8);
    }

    #[test]
    fn test_bump_by() {
        let mut h = HashHistogram::new();
        for (s, c) in [("a", 1), ("b", 3), ("a", 2), ("c", 1), ("b", 1)].iter() {
            h.bump_by(s, *c);
        }

        for (s, c) in [("a", 3), ("b", 4), ("c", 1), ("d", 0)].iter() {
            assert_eq!(h.count(s), *c);
        }

        assert_eq!(h.total_count(), 8);
    }

    #[test]
    fn test_float_count() {
        let mut h = HashHistogram::new();
        for (s, weight) in [("a", 0.25), ("b", 0.5), ("a", 0.3), ("c", 0.4), ("b", 0.1)].iter() {
            h.bump_by(s, *weight);
        }

        for (s, total) in [("a", 0.55), ("b", 0.6), ("c", 0.4)].iter() {
            assert_eq!(h.count(s), *total);
        }

        assert_f64_near!(h.total_count(), 1.55, 4);
        assert_eq!(
            h.ranking_with_counts(),
            vec![("b", 0.6), ("a", 0.55), ("c", 0.4)]
        );
    }
}
