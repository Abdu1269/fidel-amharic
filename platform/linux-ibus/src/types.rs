//! D-Bus data structures for IBus protocol serialization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Type};

/// An empty IBus attribute list representing plain unstyled or default-styled text.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct IBusAttrList {
    name: String,
    attachments: HashMap<String, OwnedValue>,
    attributes: Vec<OwnedValue>,
}

impl Default for IBusAttrList {
    fn default() -> Self {
        Self {
            name: "IBusAttrList".to_string(),
            attachments: HashMap::new(),
            attributes: Vec::new(),
        }
    }
}

/// An IBusText object containing a string and styling attributes.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct IBusText {
    name: String,
    attachments: HashMap<String, OwnedValue>,
    text: String,
    attrs: IBusAttrList,
}

impl IBusText {
    /// Constructs a new `IBusText` from a string slice with default attributes.
    pub fn new(text: &str) -> Self {
        Self {
            name: "IBusText".to_string(),
            attachments: HashMap::new(),
            text: text.to_string(),
            attrs: IBusAttrList::default(),
        }
    }
}
