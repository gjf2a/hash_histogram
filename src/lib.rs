use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;

pub struct HashHistogram<K: Hash + Eq + Copy> {
    map: HashMap<K,usize>
}

impl<K: Hash + Eq + Copy> HashHistogram<K> {
    pub fn new() -> HashHistogram<K> {HashHistogram {map: HashMap::new()}}

    pub fn get(&self, key: K) -> usize {
        *(self.map.get(&key).unwrap_or(&0))
    }

    pub fn bump(&mut self, key: K) {
        self.map.insert(key, self.get(key) + 1);
    }

    pub fn all_labels(&self) -> HashSet<K> {
        self.map.iter()
            .map(|entry| *entry.0)
            .collect()
    }

    pub fn mode(&self) -> K {
        *(self.map.iter()
            .max_by_key(|entry| entry.1)
            .unwrap().0)
    }

    pub fn total_count(&self) -> usize {
        self.map.iter().map(|entry| entry.1).sum()
    }
}

impl<K: Hash + Eq + Copy + std::cmp::Ord + fmt::Display> fmt::Display for HashHistogram<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut in_order: Vec<K> = self.all_labels().iter().copied().collect();
        in_order.sort();
        for label in in_order {
            write!(f, "{}:{}; ", label, self.get(label))?;
        }
        Ok(())
    }
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
            hist.bump(0);
        }

        for _ in 0..ones {
            hist.bump(1);
        }

        for _ in 0..twos {
            hist.bump(2);
        }

        assert_eq!(3, hist.all_labels().len());
        assert_eq!(zeros, hist.get(0));
        assert_eq!(ones, hist.get(1));
        assert_eq!(twos, hist.get(2));
        assert_eq!(2, hist.mode());
        assert_eq!(zeros + ones + twos, hist.total_count());
    }
}