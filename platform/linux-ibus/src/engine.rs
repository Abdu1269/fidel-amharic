//! Implementation of the IBus.Engine D-Bus interface using `zbus`.

use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::interface;
use zbus::object_server::SignalEmitter;

use crate::types::IBusText;
use fidel_engine::{Composer, EngineAction, KeyEvent, Modifiers, SpecialKey};

const IBUS_RELEASE_MASK: u32 = 1 << 28;
const IBUS_SHIFT_MASK: u32 = 1 << 0;
const IBUS_LOCK_MASK: u32 = 1 << 1;
const IBUS_CONTROL_MASK: u32 = 1 << 2;
const IBUS_MOD1_MASK: u32 = 1 << 3;

const IBUS_KEY_BACKSPACE: u32 = 0xFF08;
const IBUS_KEY_TAB: u32 = 0xFF09;
const IBUS_KEY_RETURN: u32 = 0xFF0D;
const IBUS_KEY_ESCAPE: u32 = 0xFF1B;
const IBUS_KEY_DELETE: u32 = 0xFFFF;
const IBUS_KEY_LEFT: u32 = 0xFF51;
const IBUS_KEY_UP: u32 = 0xFF52;
const IBUS_KEY_RIGHT: u32 = 0xFF53;
const IBUS_KEY_DOWN: u32 = 0xFF54;

/// D-Bus object implementing `org.freedesktop.IBus.Engine`.
pub struct FidelIbusEngine {
    composer: Arc<Mutex<Composer>>,
}

impl FidelIbusEngine {
    /// Creates a new engine instance wrapping a shared `Composer`.
    pub fn new(composer: Arc<Mutex<Composer>>) -> Self {
        Self { composer }
    }
}

#[interface(name = "org.freedesktop.IBus.Engine")]
impl FidelIbusEngine {
    /// Processes key events forwarded by IBus.
    async fn process_key_event(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        keyval: u32,
        _keycode: u32,
        state: u32,
    ) -> bool {
        // Ignore key release events
        if (state & IBUS_RELEASE_MASK) != 0 {
            return false;
        }

        let modifiers = Modifiers {
            shift: (state & IBUS_SHIFT_MASK) != 0,
            caps_lock: (state & IBUS_LOCK_MASK) != 0,
            ctrl: (state & IBUS_CONTROL_MASK) != 0,
            alt: (state & IBUS_MOD1_MASK) != 0,
            meta: false,
        };

        let key_event = if let Some(special) = match keyval {
            IBUS_KEY_BACKSPACE => Some(SpecialKey::Backspace),
            IBUS_KEY_RETURN => Some(SpecialKey::Enter),
            IBUS_KEY_ESCAPE => Some(SpecialKey::Escape),
            IBUS_KEY_TAB => Some(SpecialKey::Tab),
            IBUS_KEY_DELETE => Some(SpecialKey::Delete),
            IBUS_KEY_LEFT => Some(SpecialKey::Left),
            IBUS_KEY_RIGHT => Some(SpecialKey::Right),
            IBUS_KEY_UP => Some(SpecialKey::Up),
            IBUS_KEY_DOWN => Some(SpecialKey::Down),
            _ => None,
        } {
            KeyEvent {
                character: None,
                special_key: Some(special),
                modifiers,
                is_release: false,
            }
        } else if let Some(ch) = char::from_u32(keyval) {
            KeyEvent {
                character: Some(ch),
                special_key: if ch == ' ' {
                    Some(SpecialKey::Space)
                } else {
                    None
                },
                modifiers,
                is_release: false,
            }
        } else {
            return false;
        };

        let action = {
            let mut comp = self.composer.lock().await;
            comp.process_key(&key_event)
        };

        match action {
            EngineAction::PassThrough => false,
            EngineAction::Consumed => true,
            EngineAction::Commit { text } => {
                let _ = Self::commit_text(&emitter, &IBusText::new(&text)).await;
                let _ = Self::hide_preedit_text(&emitter).await;
                true
            }
            EngineAction::SetComposition { display, .. } => {
                let len = display.chars().count() as u32;
                let _ =
                    Self::update_preedit_text(&emitter, &IBusText::new(&display), len, true).await;
                true
            }
            EngineAction::CommitAndCompose {
                committed, display, ..
            } => {
                let _ = Self::commit_text(&emitter, &IBusText::new(&committed)).await;
                let len = display.chars().count() as u32;
                let _ =
                    Self::update_preedit_text(&emitter, &IBusText::new(&display), len, true).await;
                true
            }
            EngineAction::ClearComposition => {
                let _ = Self::hide_preedit_text(&emitter).await;
                true
            }
        }
    }

    /// Called when the input context gains focus.
    async fn focus_in(&mut self) {}

    /// Called when the input context loses focus.
    async fn focus_out(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        let mut comp = self.composer.lock().await;
        if comp.is_composing() {
            comp.reset();
            let _ = Self::hide_preedit_text(&emitter).await;
        }
    }

    /// Resets the engine state.
    async fn reset(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        let mut comp = self.composer.lock().await;
        comp.reset();
        let _ = Self::hide_preedit_text(&emitter).await;
    }

    /// Enables the engine.
    async fn enable(&mut self) {}

    /// Disables the engine.
    async fn disable(&mut self) {}

    // Signals emitted by the engine to the IBus daemon
    #[zbus(signal)]
    async fn commit_text(emitter: &SignalEmitter<'_>, text: &IBusText) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn update_preedit_text(
        emitter: &SignalEmitter<'_>,
        text: &IBusText,
        cursor_pos: u32,
        visible: bool,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn hide_preedit_text(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
}
