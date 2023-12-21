#[cfg(test)]
mod tests {
    use ptrie::Trie;

    #[test]
    fn new_trie_is_is_empty() {
        assert!(Trie::<char, String>::new().is_empty());
    }

    #[test]
    fn add_word_to_trie() {
        let mut t = Trie::new();
        t.insert("test".bytes(), String::from("test"));
        assert!(!t.is_empty());
    }

    #[test]
    fn contains_key_test() {
        let mut t = Trie::new();
        let test = "test".bytes();
        let tes = "tes".bytes();
        let notintest = "notintest".bytes();

        t.insert(test.clone(), String::from("test"));
        assert!(!t.is_empty());
        assert!(t.contains_key(test));
        assert!(!t.contains_key(tes));
        assert!(!t.contains_key(notintest));
    }

    #[test]
    fn contains_key_sub_path_test() {
        let mut t = Trie::new();
        let test = "test".bytes();
        let tes = "tes".bytes();
        let notintest = "notintest".bytes();

        t.insert(test.clone(), String::from("test"));
        t.insert(tes.clone(), String::from("tes"));
        assert!(!t.is_empty());
        assert!(t.contains_key(test));
        assert!(t.contains_key(tes));
        assert!(!t.contains_key(notintest));
    }

    #[test]
    fn clear_test() {
        let mut t = Trie::new();
        let data = "test".bytes();

        t.insert(data.clone(), String::from("test"));
        assert!(!t.is_empty());
        assert!(t.contains_key(data.clone()));

        t.clear();
        assert!(t.is_empty());
        assert!(!t.contains_key(data));
    }

    #[test]
    fn iterator() {
        let mut t = Trie::new();
        let test = "test".bytes();
        let tes = "tes".bytes();

        t.insert(test.clone(), String::from("test"));
        t.insert(tes.clone(), String::from("tes"));
        for (k, v) in t.iter() {
            assert!(std::str::from_utf8(&k).unwrap().starts_with("tes"));
            assert!(v.starts_with("tes"));
        }
    }
}
