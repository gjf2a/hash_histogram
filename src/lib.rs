use core::fmt;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::collections::hash_map::Iter;

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct HashHistogram<T:Hash+Clone+Eq> {
    histogram: HashMap<T,usize>
}

impl <T:Hash+Clone+Eq> HashHistogram<T> {
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

impl<K: Hash + Eq + Copy + std::cmp::Ord + fmt::Display> fmt::Display for HashHistogram<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut in_order: Vec<K> = self.iter().map(|(k,_)| k).copied().collect();
        in_order.sort();
        for label in in_order {
            write!(f, "{}:{}; ", label, self.count(&label))?;
        }
        Ok(())
    }
}

pub fn mode<K: Eq + Copy + Hash, I: Iterator<Item=K>>(items: &mut I) -> K {
    let mut counts = HashHistogram::new();
    for k in items {
        counts.bump(&k);
    }
    counts.mode().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple<'a>() -> HashHistogram<&'a str> {
        let mut h = HashHistogram::new();
        for s in ["a", "b", "a", "c", "a", "b"].iter() {
            h.bump(s);
        }
        h
    }

    #[test]
    fn it_works() {
        let h = make_simple();
        for (s, c) in [("a", 3), ("b", 2), ("c", 1), ("d", 0)].iter() {
            assert_eq!(h.count(s), *c);
        }
    }

    #[test]
    fn iterator() {
        let h = make_simple();
        let mut itered: Vec<_> = h.iter().map(|(s,c)| (*s, *c)).collect();
        itered.sort();
        assert_eq!(itered, vec![("a", 3), ("b", 2), ("c", 1)]);
    }

    #[test]
    fn sorting() {
        let h = make_simple();
        let ranking = h.ranking();
        assert_eq!(ranking, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_mode() {
        let nums = vec![5, 4, 3, 4, 5, 6, 5];
        assert_eq!(5, *mode(&mut nums.iter()));

        let mut hist = HashHistogram::new();
        for num in nums.iter() {
            hist.bump(num);
        }
        assert_eq!((5, 3), hist.mode().unwrap())
    }

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

