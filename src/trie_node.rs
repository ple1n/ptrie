#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, Ord};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TrieNode<T> {
    pub value: Option<usize>,
    pub children: Vec<(T, usize)>,
}

impl<T: Eq + Ord + Clone> TrieNode<T> {
    pub fn new(value: Option<usize>) -> TrieNode<T> {
        TrieNode {
            value,
            children: Vec::<(T, usize)>::new(),
        }
    }

    pub fn find(&self, key: &T) -> Option<usize> {
        if self.children.is_empty() {
            // Slightly improves performance by avoiding closure creation in further code
            return None;
        }
        if let Ok(idx) = self.children.binary_search_by(|x| x.0.cmp(key)) {
            return Some(self.children[idx].1);
        }
        None
    }

    pub fn insert(&mut self, key: &T, child_id: usize) {
        self.children.push((key.clone(), child_id));
        self.children.sort_by(|a, b| a.0.cmp(&b.0));
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<usize> {
        self.value
    }

    pub fn may_be_leaf(&self) -> bool {
        self.value.is_some()
    }
}
