//! Struct and functions for the `Trie` data structure

use crate::error::TrieError;
use crate::trie_node::TrieNode;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Prefix tree object, contains 1 field for the `root` node of the tree
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Trie<K: Eq + Ord + Clone, V> {
    /// Root of the prefix tree
    root: TrieNode<K, V>,
}

impl<K: Eq + Ord + Clone, V> Trie<K, V> {
    /// Creates a new `Trie` object
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let t = Trie::<char, String>::new();
    /// ```
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
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
        if self.is_empty() {
            return false;
        }
        // self.root.find_node(key).is_some()
        match self.find_node(key) {
            Some(node) => node.may_be_leaf(),
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
        self.find_node(key).and_then(|node| node.get_value())
    }

    pub fn get_mut<I: Iterator<Item = K>>(&mut self, key: I) -> Option<&mut V> {
        self.find_node_mut(key)
            .and_then(|node| Some(node.value.as_mut().unwrap()))
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
        self.find_node_mut(key)
            .ok_or_else(|| TrieError::NotFound("Key not found".to_string()))
            .map(|node| node.set_value(value))
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
    /// assert_eq!(prefixes, vec![&"ABC", &"ABCD"]);
    /// assert_eq!(trie.find_prefixes("efghij".bytes()), Vec::<&&str>::new());
    /// assert_eq!(trie.find_prefixes("abz".bytes()), Vec::<&&str>::new());
    /// ```

    pub fn find_prefixes<I: Iterator<Item = K>>(&self, key: I) -> Vec<(usize, &V)> {
        let mut node = &self.root;
        let mut prefixes = Vec::new();
        for (i, k) in key.enumerate() {
            if let Some((nk, next)) = node
                .children
                .binary_search_by_key(&&k, |(k, n)| k)
                .ok()
                .and_then(|ix| Some(&node.children[ix]))
            {
                if let Some(value) = &next.value {
                    prefixes.push((i, value));
                }
                node = next;
            } else {
                break;
            }
        }
        prefixes
    }

    pub fn iter_prefixes<I: Iterator<Item = K>>(
        &mut self,
        key: I,
        mut cb: impl FnMut(usize, &mut TrieNode<K, V>),
    ) {
        let mut node = &mut self.root;
        for (i, k) in key.enumerate() {
            if let Ok(ix) = node.children.binary_search_by_key(&&k, |(k, n)| k) {
                let (nk, next) = &mut node.children[ix];
                if let Some(_) = &mut next.value {
                    cb(i, next);
                }
                node = next;
            } else {
                cb(i, node);
                break;
            }
        }
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
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/DOID_1234".bytes()), Some("doid").as_ref());
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/1234".bytes()), Some("obo").as_ref());
    /// assert_eq!(trie.find_longest_prefix("notthere".bytes()), None.as_ref());
    /// assert_eq!(trie.find_longest_prefix("httno".bytes()), None.as_ref());
    /// ```
    pub fn find_longest_prefix<I: Iterator<Item = K>>(&self, key: I) -> Option<&V> {
        {
            let mut current = &self.root;
            let mut last_value: Option<&V> = None.as_ref();
            for k in key {
                if let Some((_, next_node)) = current.children.iter().find(|(key, _)| key == &k) {
                    if next_node.value.is_some() {
                        last_value = next_node.value.as_ref();
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
            last_value
        }
    }

    /// Returns a list of all strings in the `Trie` that start with the given prefix.
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
    /// assert_eq!(strings, vec![&"App", &"Apple", &"Applet"]);
    /// assert_eq!(trie.find_postfixes("bpp".bytes()), Vec::<&&str>::new());
    /// assert_eq!(trie.find_postfixes("apzz".bytes()), Vec::<&&str>::new());
    /// ```
    pub fn find_postfixes<I: Iterator<Item = K>>(&self, prefix: I) -> Vec<&V> {
        let mut postfixes = Vec::new();
        if let Some(node) = self.find_node(prefix) {
            self.collect_values(node, &mut postfixes);
        }
        postfixes
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_values<'a>(&self, node: &'a TrieNode<K, V>, values: &mut Vec<&'a V>) {
        if let Some(ref value) = node.value {
            values.push(value);
        }
        for (_, child) in &node.children {
            self.collect_values(child, values);
        }
    }

    /// Checks if the `Trie` is empty
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
        self.root.children.is_empty()
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
        self.root = TrieNode::default();
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
    pub fn insert<I: Iterator<Item = K>>(
        &mut self,
        key: I,
        value_cb: impl FnMut(&mut TrieNode<K, V>, Option<usize>),
    ) -> Option<&mut V> {
        self.root.insert(key.enumerate(), value_cb, None)
    }

    pub fn remove_subtree<I: Iterator<Item = K>>(&mut self, key: I) {
        self.root.remove_subtree(key.peekable())
    }

    /// Finds the node in the `Trie` for a given key
    ///
    /// Internal API
    fn find_node<I: Iterator<Item = K>>(&self, key: I) -> Option<&TrieNode<K, V>> {
        self.root.find_node(key)
    }

    fn find_node_mut<I: Iterator<Item = K>>(&mut self, key: I) -> Option<&mut TrieNode<K, V>> {
        self.root.find_node_mut(key)
    }

    /// Iterate the nodes in the `Trie`
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
        TrieIterator::new(&self)
    }
}

/// Implement the `Default` trait for `Trie` since we have a constructor that does not need arguments
impl<T: Eq + Ord + Clone, U> Default for Trie<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the `Trie` struct
pub struct TrieIterator<'a, K: Eq + Ord + Clone, V> {
    // Stack with node reference and current path
    stack: Vec<(&'a TrieNode<K, V>, Vec<K>)>,
}

impl<'a, K: Eq + Ord + Clone, V> TrieIterator<'a, K, V> {
    fn new(trie: &'a Trie<K, V>) -> Self {
        TrieIterator {
            // Start with root node and empty path
            stack: vec![(&trie.root, Vec::new())],
        }
    }
}

impl<'a, K: Eq + Ord + Clone, V> Iterator for TrieIterator<'a, K, V> {
    // Yield key-value pairs
    type Item = (Vec<K>, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, path)) = self.stack.pop() {
            // Push children to the stack with updated path
            for (key_part, child) in &node.children {
                let mut new_path = path.clone();
                new_path.push(key_part.clone());
                self.stack.push((child, new_path));
            }
            // Return value if it exists
            if let Some(ref value) = node.value {
                return Some((path, value));
            }
        }
        None
    }
}
