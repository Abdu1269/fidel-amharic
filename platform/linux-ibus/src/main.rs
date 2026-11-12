//! Main executable entry point for the Fidel Amharic IBus engine.

mod engine;
mod factory;
mod ibus_address;
mod types;

use std::error::Error;
use std::fs;
use std::path::Path;
use tracing::{error, info};
use zbus::connection::Builder;

use crate::factory::FidelIbusFactory;
use fidel_core::profile::Profile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting Fidel Amharic IBus Engine...");

    // 1. Locate the IBus socket address
    let ibus_addr = match ibus_address::get_ibus_address() {
        Some(addr) => addr,
        None => {
            error!("Could not locate IBus daemon socket. Is ibus-daemon running?");
            std::process::exit(1);
        }
    };
    info!("Connecting to IBus daemon at {}", ibus_addr);

    // 2. Load the Power Geez Unicode profile
    let profile_candidates = [
        "profiles/power_geez_phonetic_unicode.toml",
        "/usr/share/fidel-amharic/profiles/power_geez_phonetic_unicode.toml",
        "/usr/local/share/fidel-amharic/profiles/power_geez_phonetic_unicode.toml",
    ];

    let mut profile_content = None;
    for candidate in &profile_candidates {
        if Path::new(candidate).exists() {
            if let Ok(content) = fs::read_to_string(candidate) {
                profile_content = Some(content);
                info!("Loaded profile from {}", candidate);
                break;
            }
        }
    }

    // Fallback: embedded profile so the binary is always 100% self-contained!
    let profile = match profile_content {
        Some(content) => Profile::from_toml(&content)?,
        None => {
            const EMBEDDED_PROFILE: &str =
                include_str!("../../../profiles/power_geez_phonetic_unicode.toml");
            info!("Using embedded Power Geez Unicode profile");
            Profile::from_toml(EMBEDDED_PROFILE)?
        }
    };

    // 3. Connect to the IBus daemon via D-Bus
    let factory = FidelIbusFactory::new(profile);
    let connection = Builder::address(ibus_addr.as_str())?
        .serve_at("/org/freedesktop/IBus/Factory", factory)?
        .build()
        .await?;

    info!("Fidel Amharic IBus Factory registered at /org/freedesktop/IBus/Factory");

    // 4. Wait for termination signal
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Fidel Amharic IBus Engine.");

    let _ = connection.close().await;
    Ok(())
}
