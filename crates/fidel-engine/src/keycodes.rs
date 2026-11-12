//! Platform-agnostic keyboard event representation.

use serde::{Deserialize, Serialize};

/// Modifier keys state during a key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Modifiers {
    /// Shift key is active.
    pub shift: bool,
    /// Control key is active.
    pub ctrl: bool,
    /// Alt / Option key is active.
    pub alt: bool,
    /// Meta / Command / Windows key is active.
    pub meta: bool,
    /// CapsLock is toggled on.
    pub caps_lock: bool,
}

impl Modifiers {
    /// Returns true if any standard shortcut modifier (Ctrl, Alt, or Meta) is active.
    #[inline]
    pub fn is_command_shortcut(&self) -> bool {
        self.ctrl || self.alt || self.meta
    }
}

/// Abstract representation of special non-character navigation/control keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialKey {
    /// Backspace key.
    Backspace,
    /// Delete key.
    Delete,
    /// Return / Enter key.
    Enter,
    /// Escape key.
    Escape,
    /// Tab key.
    Tab,
    /// Spacebar key.
    Space,
    /// Cursor moving Left.
    Left,
    /// Cursor moving Right.
    Right,
    /// Cursor moving Up.
    Up,
    /// Cursor moving Down.
    Down,
    /// Other non-handled key.
    Other,
}

/// A normalized keyboard event received from any native platform adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvent {
    /// The Unicode character produced by the key press, if any (e.g. 'h', 'A', ':').
    pub character: Option<char>,
    /// Special non-character key code, if applicable.
    pub special_key: Option<SpecialKey>,
    /// Active modifier keys.
    pub modifiers: Modifiers,
    /// True if this is a key release (up) event; false for key down / press.
    pub is_release: bool,
}

impl KeyEvent {
    /// Creates a key press event for a character.
    pub fn from_char(c: char) -> Self {
        Self {
            character: Some(c),
            special_key: if c == ' ' {
                Some(SpecialKey::Space)
            } else {
                None
            },
            modifiers: Modifiers::default(),
            is_release: false,
        }
    }

    /// Creates a key press event for a special key.
    pub fn from_special(key: SpecialKey) -> Self {
        Self {
            character: if key == SpecialKey::Space {
                Some(' ')
            } else {
                None
            },
            special_key: Some(key),
            modifiers: Modifiers::default(),
            is_release: false,
        }
    }
}
