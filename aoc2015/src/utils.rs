use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, s: &str) {
        let mut node = &mut self.root;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
        }
    }

    pub fn contains(&self, s: &str) -> bool {
        let mut node = &self.root;
        for c in s.chars() {
            if !node.children.contains_key(&c) {
                return false;
            }
            node = node.children.get(&c).unwrap();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("foo");
        assert!(trie.contains("foo"));
        assert!(!trie.contains("bar"));
    }
}
