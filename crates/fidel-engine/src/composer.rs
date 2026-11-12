//! The central composition state machine for Fidel Amharic.

use fidel_core::profile::Profile;

use crate::actions::EngineAction;
use crate::keycodes::{KeyEvent, SpecialKey};
use crate::trie::{Trie, TrieLookup};

/// A stateful composer session managing keystroke buffering, phonetic lookup,
/// and preedit/commit action generation.
#[derive(Debug)]
pub struct Composer {
    trie: Trie,
    raw_buffer: String,
    last_matched_output: Option<String>,
    terminators: Vec<char>,
}

impl Composer {
    /// Creates a new `Composer` initialized from a validated `Profile`.
    pub fn new(profile: &Profile) -> Self {
        let mut trie = Trie::new();
        for rule in &profile.rules {
            trie.insert(&rule.keys, &rule.output);
        }

        Self {
            trie,
            raw_buffer: String::new(),
            last_matched_output: None,
            terminators: profile.options.terminators.clone(),
        }
    }

    /// Resets the composition state, clearing all uncommitted buffers.
    pub fn reset(&mut self) {
        self.raw_buffer.clear();
        self.last_matched_output = None;
    }

    /// Returns true if there is an active composition in progress.
    #[inline]
    pub fn is_composing(&self) -> bool {
        !self.raw_buffer.is_empty()
    }

    /// Returns the raw ASCII composition buffer.
    #[inline]
    pub fn raw_buffer(&self) -> &str {
        &self.raw_buffer
    }

    /// Processes a single incoming `KeyEvent` and produces the resulting `EngineAction`.
    pub fn process_key(&mut self, event: &KeyEvent) -> EngineAction {
        // Ignore key release events
        if event.is_release {
            return EngineAction::Consumed;
        }

        // Pass through shortcut combinations (Ctrl/Alt/Meta)
        if event.modifiers.is_command_shortcut() {
            if self.is_composing() {
                // If a command shortcut is triggered during composition, commit current match first
                let committed = self
                    .last_matched_output
                    .take()
                    .unwrap_or_else(|| self.raw_buffer.clone());
                self.raw_buffer.clear();
                return EngineAction::Commit { text: committed };
            }
            return EngineAction::PassThrough;
        }

        // Handle special navigation and control keys
        if let Some(special) = event.special_key {
            return self.handle_special_key(special);
        }

        // Handle standard character input
        if let Some(ch) = event.character {
            return self.handle_character(ch);
        }

        EngineAction::PassThrough
    }

    fn handle_special_key(&mut self, key: SpecialKey) -> EngineAction {
        match key {
            SpecialKey::Backspace => {
                if !self.is_composing() {
                    return EngineAction::PassThrough;
                }

                self.raw_buffer.pop();
                if self.raw_buffer.is_empty() {
                    self.last_matched_output = None;
                    EngineAction::ClearComposition
                } else {
                    match self.trie.lookup(&self.raw_buffer) {
                        TrieLookup::ExactMatch(out) | TrieLookup::MatchWithMore(out) => {
                            self.last_matched_output = Some(out.clone());
                            EngineAction::SetComposition {
                                display: out,
                                raw: self.raw_buffer.clone(),
                            }
                        }
                        TrieLookup::Prefix => {
                            self.last_matched_output = None;
                            EngineAction::SetComposition {
                                display: self.raw_buffer.clone(),
                                raw: self.raw_buffer.clone(),
                            }
                        }
                        TrieLookup::NoMatch => {
                            self.last_matched_output = None;
                            EngineAction::SetComposition {
                                display: self.raw_buffer.clone(),
                                raw: self.raw_buffer.clone(),
                            }
                        }
                    }
                }
            }
            SpecialKey::Escape => {
                if self.is_composing() {
                    self.reset();
                    EngineAction::ClearComposition
                } else {
                    EngineAction::PassThrough
                }
            }
            SpecialKey::Enter => {
                if self.is_composing() {
                    let text = self
                        .last_matched_output
                        .take()
                        .unwrap_or_else(|| self.raw_buffer.clone());
                    self.raw_buffer.clear();
                    EngineAction::Commit { text }
                } else {
                    EngineAction::PassThrough
                }
            }
            SpecialKey::Space => {
                if self.is_composing() {
                    let mut text = self
                        .last_matched_output
                        .take()
                        .unwrap_or_else(|| self.raw_buffer.clone());
                    text.push(' ');
                    self.raw_buffer.clear();
                    EngineAction::Commit { text }
                } else {
                    EngineAction::PassThrough
                }
            }
            SpecialKey::Tab
            | SpecialKey::Left
            | SpecialKey::Right
            | SpecialKey::Up
            | SpecialKey::Down
            | SpecialKey::Delete => {
                if self.is_composing() {
                    // Commit before navigating
                    let text = self
                        .last_matched_output
                        .take()
                        .unwrap_or_else(|| self.raw_buffer.clone());
                    self.raw_buffer.clear();
                    EngineAction::Commit { text }
                } else {
                    EngineAction::PassThrough
                }
            }
            SpecialKey::Other => EngineAction::PassThrough,
        }
    }

    fn handle_character(&mut self, ch: char) -> EngineAction {
        // Check if this character is an explicit terminator (e.g. '\'')
        if self.terminators.contains(&ch) {
            if self.is_composing() {
                let text = self
                    .last_matched_output
                    .take()
                    .unwrap_or_else(|| self.raw_buffer.clone());
                self.raw_buffer.clear();
                return EngineAction::Commit { text };
            } else {
                return EngineAction::PassThrough;
            }
        }

        let mut candidate = self.raw_buffer.clone();
        candidate.push(ch);

        match self.trie.lookup(&candidate) {
            TrieLookup::ExactMatch(output) => {
                self.raw_buffer.clear();
                self.last_matched_output = None;
                EngineAction::Commit { text: output }
            }
            TrieLookup::MatchWithMore(output) => {
                self.raw_buffer = candidate;
                self.last_matched_output = Some(output.clone());
                EngineAction::SetComposition {
                    display: output,
                    raw: self.raw_buffer.clone(),
                }
            }
            TrieLookup::Prefix => {
                self.raw_buffer = candidate;
                EngineAction::SetComposition {
                    display: self.raw_buffer.clone(),
                    raw: self.raw_buffer.clone(),
                }
            }
            TrieLookup::NoMatch => {
                // If candidate didn't match, can we commit what was previously buffered?
                if self.is_composing() {
                    if let Some(prev_match) = self.last_matched_output.take() {
                        self.raw_buffer.clear();
                        // Now start fresh evaluation for the new character `ch`
                        let single_char = ch.to_string();
                        match self.trie.lookup(&single_char) {
                            TrieLookup::ExactMatch(fresh_out) => EngineAction::Commit {
                                text: format!("{}{}", prev_match, fresh_out),
                            },
                            TrieLookup::MatchWithMore(fresh_out) => {
                                self.raw_buffer = single_char;
                                self.last_matched_output = Some(fresh_out.clone());
                                EngineAction::CommitAndCompose {
                                    committed: prev_match,
                                    display: fresh_out,
                                    raw: self.raw_buffer.clone(),
                                }
                            }
                            TrieLookup::Prefix => {
                                self.raw_buffer = single_char;
                                EngineAction::CommitAndCompose {
                                    committed: prev_match,
                                    display: self.raw_buffer.clone(),
                                    raw: self.raw_buffer.clone(),
                                }
                            }
                            TrieLookup::NoMatch => {
                                // Neither combo nor single character match: commit previous and pass through new
                                EngineAction::Commit {
                                    text: format!("{}{}", prev_match, ch),
                                }
                            }
                        }
                    } else {
                        // Previous was an incomplete prefix with no match; commit raw + ch
                        let uncommitted = std::mem::take(&mut self.raw_buffer);
                        EngineAction::Commit {
                            text: format!("{}{}", uncommitted, ch),
                        }
                    }
                } else {
                    // Not currently composing; pass through character if not in trie
                    EngineAction::PassThrough
                }
            }
        }
    }
}
