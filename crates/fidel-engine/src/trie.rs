//! Trie (prefix tree) data structure for efficient phonetic sequence matching.

use std::collections::HashMap;

/// Result of querying a sequence in the phonetic Trie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrieLookup {
    /// No mapping exists with this sequence, nor is it a valid prefix of any rule.
    NoMatch,
    /// The sequence is a prefix of one or more rules, but does not match any complete rule.
    Prefix,
    /// The sequence matches a complete rule and cannot be extended into a longer rule.
    ExactMatch(String),
    /// The sequence matches a rule, but could also be extended into a longer rule
    /// (e.g., 's' matches a base consonant, but 'sh' is also a valid longer sequence).
    MatchWithMore(String),
}

/// A node in the phonetic mapping Trie.
#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    output: Option<String>,
    is_terminal: bool,
}

/// Prefix tree for fast O(k) lookups of multi-character phonetic key sequences.
#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Creates a new empty Trie.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Inserts a mapping rule from `sequence` to `output`.
    pub fn insert(&mut self, sequence: &str, output: &str) {
        if sequence.is_empty() {
            return;
        }

        let mut node = &mut self.root;
        for ch in sequence.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.output = Some(output.to_string());
        node.is_terminal = true;
    }

    /// Looks up a key sequence in the Trie.
    pub fn lookup(&self, sequence: &str) -> TrieLookup {
        if sequence.is_empty() {
            return TrieLookup::NoMatch;
        }

        let mut node = &self.root;
        for ch in sequence.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return TrieLookup::NoMatch,
            }
        }

        let has_children = !node.children.is_empty();
        match (&node.output, has_children) {
            (Some(out), true) => TrieLookup::MatchWithMore(out.clone()),
            (Some(out), false) => TrieLookup::ExactMatch(out.clone()),
            (None, true) => TrieLookup::Prefix,
            (None, false) => TrieLookup::NoMatch,
        }
    }

    /// Returns true if `sequence` is a valid prefix (or exact match) for at least one rule.
    pub fn is_valid_prefix(&self, sequence: &str) -> bool {
        !matches!(self.lookup(sequence), TrieLookup::NoMatch)
    }
}
