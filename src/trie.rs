//! Struct and functions for the `Trie` data structure

use crate::error::TrieError;
use crate::trie_node::TrieNode;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, Ord};

/// Prefix tree object
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Trie<K, V> {
    /// Root of the prefix tree
    nodes: Vec<TrieNode<K>>,
    values: Vec<V>,
}

impl<K: Eq + Ord + Clone, V: Clone> Trie<K, V> {
    /// Creates a new `Trie` object
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let t = Trie::<char, String>::new();
    /// ```
    pub fn new() -> Trie<K, V> {
        Trie {
            nodes: Vec::<TrieNode<K>>::new(),
            values: Vec::<V>::new(),
        }
    }

    /// Checks that trie is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let t = Trie::<char, f64>::new();
    /// assert!(t.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Adds a new key to the `Trie`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// t.insert(data.clone(), 42);
    /// t.insert(data, 42);
    /// t.insert("test2".bytes(), 43);
    /// assert!(!t.is_empty());
    /// ```
    pub fn insert<I: Iterator<Item = K>>(&mut self, key: I, value: V) {
        let mut node_id = if self.is_empty() {
            self.create_new_node()
        } else {
            0usize
        };
        for c in key {
            if let Some(id) = self.nodes[node_id].find(&c) {
                node_id = id;
            } else {
                let new_node_id = self.create_new_node();
                self.nodes[node_id].insert(&c, new_node_id);
                node_id = new_node_id;
            }
        }
        // NOTE: nicer syntax, but some lines missed by coverage
        // for c in key {
        //     node_id = self.nodes[node_id]
        //         .find(&c)
        //         .unwrap_or_else(|| {
        //             let new_node_id = self.create_new_node();
        //             self.nodes[node_id].insert(&c, new_node_id);
        //             new_node_id
        //         });
        // }
        let value_id = match self.nodes[node_id].get_value() {
            Some(id) => {
                self.values[id] = value;
                id
            }
            None => {
                self.values.push(value);
                self.values.len() - 1
            }
        };
        self.nodes[node_id].set_value(value_id);
    }

    /// Clears the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    ///
    /// t.insert(data, String::from("test"));
    /// t.clear();
    /// assert!(t.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.values.clear();
    }

    /// Looks for the key in trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// let another_data = "notintest".bytes();
    /// assert!(!t.contains_key(data.clone()));
    /// t.insert(data.clone(), 42);
    ///
    /// assert!(!t.is_empty());
    /// assert!(t.contains_key(data));
    /// assert!(!t.contains_key(another_data));
    /// ```
    pub fn contains_key<I: Iterator<Item = K>>(&self, key: I) -> bool {
        if self.values.is_empty() && self.nodes.is_empty() {
            return false;
        }
        match self.find_node(key) {
            Some(node_id) => self.nodes[node_id].may_be_leaf(),
            None => false,
        }
    }

    /// Gets the value from the tree by key
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// let another_data = "notintest".bytes();
    /// assert_eq!(t.get(data.clone()), None);
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.get(data), Some(42).as_ref());
    /// assert_eq!(t.get(another_data), None);
    /// ```
    pub fn get<I: Iterator<Item = K>>(&self, key: I) -> Option<&V> {
        self.find_node(key)
            .and_then(|node_id| self.nodes[node_id].get_value())
            .and_then(|value_id| self.values.get(value_id))
        // .cloned()
    }

    /// Sets the value pointed by a key
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// let another_data = "notintest".bytes();
    ///
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.get(data.clone()), Some(42).as_ref());
    /// assert!(t.set_value(data.clone(), 43).is_ok());
    /// assert_eq!(t.get(data), Some(43).as_ref());
    /// assert!(t.set_value(another_data, 39)
    ///     .map_err(|e| assert!(e.to_string().starts_with("Key not found")))
    ///     .is_err());
    /// ```
    pub fn set_value<I: Iterator<Item = K>>(&mut self, key: I, value: V) -> Result<(), TrieError> {
        self.find_node(key)
            .ok_or_else(|| TrieError::NotFound("Key not found".to_string()))
            .and_then(|node_id| {
                self.nodes[node_id]
                    .get_value()
                    .ok_or_else(|| TrieError::NotFound(format!("Value not found {}", node_id)))
                    .map(|value_id| {
                        self.values[value_id] = value;
                    })
            })
    }

    /// Returns a list of all prefixes in the trie for a given string, ordered from smaller to longer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("abc".bytes(), "ABC");
    /// trie.insert("abcd".bytes(), "ABCD");
    /// trie.insert("abcde".bytes(), "ABCDE");
    ///
    /// let prefixes = trie.find_prefixes("abcd".bytes());
    /// assert_eq!(prefixes, vec!["ABC", "ABCD"]);
    /// assert_eq!(trie.find_prefixes("efghij".bytes()), Vec::<&str>::new());
    /// assert_eq!(trie.find_prefixes("abz".bytes()), Vec::<&str>::new());
    /// ```
    pub fn find_prefixes<I: Iterator<Item = K>>(&self, key: I) -> Vec<V> {
        let mut node_id = 0usize;
        let mut prefixes = Vec::new();
        for c in key {
            if let Some(child_id) = self.nodes[node_id].find(&c) {
                node_id = child_id;
                if let Some(value_id) = self.nodes[node_id].get_value() {
                    prefixes.push(self.values[value_id].clone());
                }
            } else {
                break;
            }
        }
        prefixes
    }

    /// Finds the longest prefix in the `Trie` for a given string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut trie = Trie::default();
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/DOID_1234".bytes()), None);
    /// trie.insert("http://purl.obolibrary.org/obo/DOID_".bytes(), "doid");
    /// trie.insert("http://purl.obolibrary.org/obo/".bytes(), "obo");
    ///
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/DOID_1234".bytes()), Some("doid"));
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/1234".bytes()), Some("obo"));
    /// assert_eq!(trie.find_longest_prefix("notthere".bytes()), None);
    /// assert_eq!(trie.find_longest_prefix("httno".bytes()), None);
    /// ```
    pub fn find_longest_prefix<I: Iterator<Item = K>>(&self, key: I) -> Option<V> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut node_id = 0usize;
        let mut last_value_id: Option<usize> = None;
        for c in key {
            if let Some(child_id) = self.nodes[node_id].find(&c) {
                node_id = child_id;
                if self.nodes[node_id].may_be_leaf() {
                    last_value_id = self.nodes[node_id].get_value();
                }
            } else {
                break;
            }
        }
        last_value_id.map(|id| self.values[id].clone())
    }

    /// Returns a list of all strings in the trie that start with the given prefix.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("app".bytes(), "App");
    /// trie.insert("apple".bytes(), "Apple");
    /// trie.insert("applet".bytes(), "Applet");
    /// trie.insert("apricot".bytes(), "Apricot");
    ///
    /// let strings = trie.find_postfixes("app".bytes());
    /// assert_eq!(strings, vec!["App", "Apple", "Applet"]);
    /// assert_eq!(trie.find_postfixes("bpp".bytes()), Vec::<&str>::new());
    /// assert_eq!(trie.find_postfixes("apzz".bytes()), Vec::<&str>::new());
    /// ```
    pub fn find_postfixes<I: Iterator<Item = K>>(&self, prefix: I) -> Vec<V> {
        match self.find_node(prefix) {
            Some(node_id) => {
                // Collects all values from the subtree rooted at the given node.
                let mut values = Vec::new();
                self.dfs(node_id, &mut values);
                values
            }
            None => Vec::new(),
        }
    }

    /// Depth-first search to collect values.
    fn dfs(&self, node_id: usize, values: &mut Vec<V>) {
        if let Some(value_id) = self.nodes[node_id].get_value() {
            values.push(self.values[value_id].clone());
        }
        for &(_, child_id) in &self.nodes[node_id].children {
            self.dfs(child_id, values);
        }
    }

    /// Finds the node in the trie by the key
    ///
    /// Internal API
    fn find_node<I: Iterator<Item = K>>(&self, key: I) -> Option<usize> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut node_id = 0usize;
        for c in key {
            match self.nodes[node_id].find(&c) {
                Some(child_id) => node_id = child_id,
                None => return None,
            }
        }
        Some(node_id)
    }

    /// Creates a new node and returns the node id
    ///
    /// Internal API
    fn create_new_node(&mut self) -> usize {
        self.nodes.push(TrieNode::new(None));
        self.nodes.len() - 1
    }

    /// Iterate the nodes in the trie
    ///
    /// # Example
    ///
    /// ```
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let test = "test".bytes();
    /// let tes = "tes".bytes();
    ///
    /// t.insert(test.clone(), String::from("test"));
    /// t.insert(tes.clone(), String::from("tes"));
    /// for (k, v) in t.iter() {
    ///     assert!(std::str::from_utf8(&k).unwrap().starts_with("tes"));
    ///     assert!(v.starts_with("tes"));
    /// }
    /// ```
    pub fn iter(&self) -> TrieIterator<K, V> {
        TrieIterator::new(self)
    }
}

/// Implement the `Default` trait for `Trie` since we have a constructor that does not need arguments
impl<T: Eq + Ord + Clone, U: Clone> Default for Trie<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the `Trie` struct
pub struct TrieIterator<'a, K, V> {
    trie: &'a Trie<K, V>,
    stack: Vec<(usize, Vec<K>)>, // Stack with node id and current path
}

impl<'a, K, V> TrieIterator<'a, K, V> {
    fn new(trie: &'a Trie<K, V>) -> Self {
        TrieIterator {
            trie,
            stack: vec![(0, Vec::new())], // Start with root node and empty path
        }
    }
}

impl<'a, K, V> Iterator for TrieIterator<'a, K, V>
where
    K: Eq + Ord + Clone,
    V: Clone,
{
    type Item = (Vec<K>, V); // Yield key-value pairs

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node_id, path)) = self.stack.pop() {
            let node = &self.trie.nodes[node_id];
            // Push children to the stack with updated path
            for &(ref key_part, child_id) in &node.children {
                let mut new_path = path.clone();
                new_path.push(key_part.clone());
                self.stack.push((child_id, new_path));
            }
            // Return value if it exists
            if let Some(value_id) = node.get_value() {
                return Some((path, self.trie.values[value_id].clone()));
            }
        }
        None
    }
}
