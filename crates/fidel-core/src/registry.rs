//! In-memory profile registry and profile discovery.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::error::CoreError;
use crate::profile::Profile;

/// Thread-safe registry holding available keyboard layout profiles.
#[derive(Debug, Default)]
pub struct ProfileRegistry {
    profiles: RwLock<HashMap<String, Profile>>,
    active_profile_id: RwLock<Option<String>>,
}

impl ProfileRegistry {
    /// Creates a new, empty profile registry.
    pub fn new() -> Self {
        Self {
            profiles: RwLock::new(HashMap::new()),
            active_profile_id: RwLock::new(None),
        }
    }

    /// Registers a new profile into the registry.
    /// Returns an error if the profile fails validation.
    pub fn register(&self, profile: Profile) -> Result<(), CoreError> {
        profile.validate()?;
        let id = profile.metadata.id.clone();

        let mut lock = self
            .profiles
            .write()
            .map_err(|_| CoreError::ParseError("Registry lock poisoned".to_string()))?;

        // If no active profile yet, make this one active by default
        let mut active_lock = self
            .active_profile_id
            .write()
            .map_err(|_| CoreError::ParseError("Registry lock poisoned".to_string()))?;
        if active_lock.is_none() {
            *active_lock = Some(id.clone());
        }

        lock.insert(id, profile);
        Ok(())
    }

    /// Retrieves a cloned profile by its unique identifier.
    pub fn get(&self, id: &str) -> Result<Profile, CoreError> {
        let lock = self
            .profiles
            .read()
            .map_err(|_| CoreError::ParseError("Registry lock poisoned".to_string()))?;
        lock.get(id)
            .cloned()
            .ok_or_else(|| CoreError::ProfileNotFound(id.to_string()))
    }

    /// Sets the currently active profile identifier.
    pub fn set_active(&self, id: &str) -> Result<(), CoreError> {
        let profiles = self
            .profiles
            .read()
            .map_err(|_| CoreError::ParseError("Registry lock poisoned".to_string()))?;

        if !profiles.contains_key(id) {
            return Err(CoreError::ProfileNotFound(id.to_string()));
        }

        let mut active = self
            .active_profile_id
            .write()
            .map_err(|_| CoreError::ParseError("Registry lock poisoned".to_string()))?;
        *active = Some(id.to_string());
        Ok(())
    }

    /// Gets the currently active profile, if any is registered.
    pub fn get_active(&self) -> Option<Profile> {
        let active_id = self.active_profile_id.read().ok()?.clone()?;
        self.get(&active_id).ok()
    }

    /// Lists the IDs of all registered profiles.
    pub fn list_ids(&self) -> Vec<String> {
        if let Ok(lock) = self.profiles.read() {
            lock.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }
}
