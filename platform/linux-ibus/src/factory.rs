//! Implementation of the IBus.Factory D-Bus interface.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::interface;
use zbus::zvariant::OwnedObjectPath;

use crate::engine::FidelIbusEngine;
use fidel_core::profile::Profile;
use fidel_engine::Composer;

/// Factory object implementing `org.freedesktop.IBus.Factory`.
pub struct FidelIbusFactory {
    profile: Profile,
    engine_counter: AtomicUsize,
}

impl FidelIbusFactory {
    /// Creates a new factory with the given profile.
    pub fn new(profile: Profile) -> Self {
        Self {
            profile,
            engine_counter: AtomicUsize::new(0),
        }
    }
}

#[interface(name = "org.freedesktop.IBus.Factory")]
impl FidelIbusFactory {
    /// Creates an engine instance for the requested engine name.
    async fn create_engine(
        &self,
        #[zbus(connection)] connection: &zbus::Connection,
        _engine_name: &str,
    ) -> zbus::fdo::Result<OwnedObjectPath> {
        let id = self.engine_counter.fetch_add(1, Ordering::SeqCst);
        let path_str = format!("/org/freedesktop/IBus/Engine/{}", id);
        let object_path = OwnedObjectPath::try_from(path_str.as_str())
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        let composer = Arc::new(Mutex::new(Composer::new(&self.profile)));
        let engine = FidelIbusEngine::new(composer);

        connection
            .object_server()
            .at(&object_path, engine)
            .await
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        Ok(object_path)
    }
}
