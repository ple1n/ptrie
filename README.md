<h1 align="center">
  🎄 Prefix Trie
</h1>

<p align="center">
    <a href="https://crates.io/crates/ptrie">
        <img alt="Crates.io" src="https://img.shields.io/crates/v/ptrie" />
    </a>
    <a href="https://github.com/vemonet/ptrie/actions/workflows/test.yml">
        <img alt="Test" src="https://github.com/vemonet/ptrie/actions/workflows/test.yml/badge.svg" />
    </a>
    <a href="https://github.com/vemonet/ptrie/actions/workflows/release.yml">
        <img alt="Release" src="https://github.com/vemonet/ptrie/actions/workflows/release.yml/badge.svg" />
    </a>
    <a href="https://docs.rs/ptrie">
        <img alt="Documentation" src="https://docs.rs/ptrie/badge.svg" />
    </a>
    <a href="https://codecov.io/gh/vemonet/ptrie/branch/main">
        <img src="https://codecov.io/gh/vemonet/ptrie/branch/main/graph/badge.svg" alt="Codecov status" />
    </a>
    <a href="https://github.com/vemonet/ptrie/blob/main/LICENSE">
        <img alt="MIT license" src="https://img.shields.io/badge/License-MIT-brightgreen.svg" />
    </a>
</p>

`PTrie` is a versatile implementation of the [trie data structure](https://en.wikipedia.org/wiki/Trie), tailored for efficient prefix searching within a collection of objects, such as strings, with no dependencies.

The structure is defined as `Trie<K, V>`, where `K` represents the type of keys in each node, and `V` is the type of the associated values.

## 💭 Motivation

The trie is particularly effective for operations involving common  prefix identification and retrieval, making it a good choice for  applications that require fast and efficient prefix-based search  functionalities.

## 🚀 Usage

### ✨ Find prefixes

PTrie can return all prefixes in the trie corresponding to a given string, sorted in ascending order of their length.

```rust
use ptrie::Trie;

let mut trie = Trie::new();

trie.insert("a".bytes(), "A");
trie.insert("ab".bytes(), "AB");
trie.insert("abc".bytes(), "ABC");
trie.insert("abcde".bytes(), "ABCDE");

let prefixes = trie.find_prefixes("abcd".bytes());
assert_eq!(prefixes, vec!["A", "AB", "ABC"]);
```

### 🔍 Find postfixes

PTrie can also find all strings in the trie that begin with a specified prefix.

```rust
use ptrie::Trie;

let mut trie = Trie::new();

trie.insert("app".bytes(), "App");
trie.insert("apple".bytes(), "Apple");
trie.insert("applet".bytes(), "Applet");
trie.insert("apricot".bytes(), "Apricot");

let strings = trie.find_postfixes("app".bytes());
assert_eq!(strings, vec!["App", "Apple", "Applet"]);
```

### 🔑 Key-based Retrieval Functions

PTrie provides functions to check for the existence of a key and to retrieve the associated value.

```rust
use ptrie::Trie;

let mut trie = Trie::new();
trie.insert("app".bytes(), "App");

assert!(trie.contains_key("app".bytes()));
assert!(!trie.contains_key("not_existing_key".bytes()));
assert_eq!(trie.get_value("app".bytes()), Some("App"));
assert_eq!(trie.get_value("none".bytes()), None);
```

## 🏷️ Features

The `serde` feature adds Serde `Serialize` and `Deserialize` traits to the `Trie` and `TrieNode` struct.

## 📜 License

[MIT License](https://opensource.org/licenses/MIT)
