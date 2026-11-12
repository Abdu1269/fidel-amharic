//! Utility to discover the active IBus session bus address.

use std::fs;
use std::path::PathBuf;

/// Locates the running IBus daemon socket address.
pub fn get_ibus_address() -> Option<String> {
    // 1. Check environment variable
    if let Ok(addr) = std::env::var("IBUS_ADDRESS") {
        if !addr.trim().is_empty() {
            return Some(addr);
        }
    }

    // 2. Discover from ~/.config/ibus/bus/
    let home = std::env::var("HOME").ok()?;
    let bus_dir = PathBuf::from(home).join(".config/ibus/bus");
    let entries = fs::read_dir(bus_dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if let Some(rest) = trimmed.strip_prefix("IBUS_ADDRESS=") {
                        let addr = rest.trim();
                        // IBus address format may include ",guid=..." which zbus parses
                        return Some(addr.to_string());
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_ibus_address_if_daemon_running() {
        if let Some(addr) = get_ibus_address() {
            assert!(addr.starts_with("unix:path="), "Unexpected address: {addr}");
        }
    }
}
