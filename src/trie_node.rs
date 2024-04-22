//! Struct and functions for the `Trie` nodes

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{clone::Clone, iter::Peekable};

/// A node in the `Trie`, it holds a value, and a list of children nodes
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TrieNode<K: Eq + Ord + Clone, V> {
    pub value: Option<V>,
    /// sorted
    pub children: Vec<(K, TrieNode<K, V>)>,
}

impl<K: Eq + Ord + Clone, V> TrieNode<K, V> {
    pub fn new() -> Self {
        TrieNode {
            value: None,
            children: Vec::new(),
        }
    }

    /// Insert a node in the trie
    pub fn insert<I: Iterator<Item = (usize, K)>>(
        &mut self,
        mut key: I,
        mut value_cb: impl FnMut(&mut TrieNode<K, V>, Option<usize>),
        cur: Option<usize>,
    ) -> Option<&mut V> {
        value_cb(self, cur);
        if let Some((iterx, part)) = key.next() {
            match self.children.binary_search_by_key(&&part, |(k, n)| k) {
                Ok(ix) => self.children[ix].1.insert(key, value_cb, Some(iterx)),
                Err(ix) => {
                    let new_node = TrieNode::new();
                    self.children.insert(ix, (part, new_node));
                    self.children
                        .get_mut(ix)
                        .unwrap()
                        .1
                        .insert(key, value_cb, Some(iterx))
                }
            }
        } else {
            self.value.as_mut()
        }
    }

    pub fn remove_subtree<I: Iterator<Item = K>>(&mut self, mut key: Peekable<I>) {
        if let Some(next) = key.next() {
            if let Some(ix) = self.children.binary_search_by_key(&&next, |(k, n)| k).ok() {
                if key.peek().is_none() {
                    self.children.remove(ix);
                } else {
                    self.children[ix].1.remove_subtree(key);
                }
            }
        }
    }

    /// Recursively find a node searching through children
    pub fn find_node<I: Iterator<Item = K>>(&self, mut key: I) -> Option<&Self> {
        if let Some(p) = key.next() {
            self.children
                .binary_search_by_key(&&p, |(k, n)| k)
                .ok() // each prefix must exist
                .and_then(|f| self.children[f].1.find_node(key))
        } else {
            Some(self)
        }
    }

    pub fn find_node_mut<I: Iterator<Item = K>>(&mut self, mut key: I) -> Option<&mut Self> {
        if let Some(p) = key.next() {
            self.children
                .binary_search_by_key(&&p, |(k, n)| k)
                .ok() // each prefix must exist
                .and_then(|f| self.children[f].1.find_node_mut(key))
        } else {
            Some(self)
        }
    }

    pub fn set_value(&mut self, value: V) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn may_be_leaf(&self) -> bool {
        self.value.is_some()
    }
}

impl<T: Eq + Ord + Clone, U> Default for TrieNode<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
