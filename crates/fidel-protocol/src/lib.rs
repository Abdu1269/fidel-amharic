//! Stable C ABI and FFI boundary for native platform adapters (TSF, IMK, Fcitx5).

#![expect(
    unsafe_code,
    reason = "FFI boundary requiring C ABI and raw pointer manipulation"
)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use fidel_core::profile::Profile;
use fidel_engine::{Composer, EngineAction, KeyEvent, SpecialKey};

/// Opaque pointer handle to a heap-allocated `Composer`.
#[repr(C)]
pub struct FidelEngineHandle {
    composer: Composer,
    // Buffer storing temporary CString allocations for FFI return
    last_text: Option<CString>,
    last_display: Option<CString>,
    last_raw: Option<CString>,
}

/// Action types matching `EngineAction` across the C ABI.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FidelActionType {
    /// Pass through without processing.
    PassThrough = 0,
    /// Consumed without visible change.
    Consumed = 1,
    /// Commit final text directly to application.
    Commit = 2,
    /// Set or update the inline preedit / composition string.
    SetComposition = 3,
    /// Commit previous match and begin composing remainder.
    CommitAndCompose = 4,
    /// Clear composition without committing.
    ClearComposition = 5,
}

/// Output action struct passed to native C/C++/Swift adapters.
#[repr(C)]
pub struct CFidelAction {
    /// Action category.
    pub action_type: FidelActionType,
    /// UTF-8 null-terminated commit string (valid until next engine call).
    pub text: *const c_char,
    /// UTF-8 null-terminated preedit display string (valid until next engine call).
    pub display: *const c_char,
    /// UTF-8 null-terminated raw ASCII buffer (valid until next engine call).
    pub raw: *const c_char,
}

/// Creates a new `Composer` instance from a UTF-8 TOML profile string.
/// Returns null pointer if the profile fails validation.
///
/// # Safety
/// `profile_toml_ptr` must be a valid, null-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn fidel_engine_create(
    profile_toml_ptr: *const c_char,
) -> *mut FidelEngineHandle {
    if profile_toml_ptr.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(profile_toml_ptr) };
    let toml_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let profile = match Profile::from_toml(toml_str) {
        Ok(p) => p,
        Err(_) => return ptr::null_mut(),
    };

    let composer = Composer::new(&profile);
    Box::into_raw(Box::new(FidelEngineHandle {
        composer,
        last_text: None,
        last_display: None,
        last_raw: None,
    }))
}

/// Destroys a previously created `FidelEngineHandle`.
///
/// # Safety
/// `handle` must be a valid pointer obtained from `fidel_engine_create` or null.
#[no_mangle]
pub unsafe extern "C" fn fidel_engine_destroy(handle: *mut FidelEngineHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}

/// Resets the engine's active composition buffer.
///
/// # Safety
/// `handle` must be a valid, non-null pointer to `FidelEngineHandle`.
#[no_mangle]
pub unsafe extern "C" fn fidel_engine_reset(handle: *mut FidelEngineHandle) {
    if let Some(engine) = unsafe { handle.as_mut() } {
        engine.composer.reset();
        engine.last_text = None;
        engine.last_display = None;
        engine.last_raw = None;
    }
}

/// Processes a Unicode scalar value character (e.g. 'a', 'h', ':').
///
/// # Safety
/// `handle` and `out_action` must be valid, non-null pointers.
#[no_mangle]
pub unsafe extern "C" fn fidel_engine_process_char(
    handle: *mut FidelEngineHandle,
    unicode_char: u32,
    out_action: *mut CFidelAction,
) -> c_int {
    if handle.is_null() || out_action.is_null() {
        return -1;
    }

    let engine = unsafe { &mut *handle };
    let ch = match char::from_u32(unicode_char) {
        Some(c) => c,
        None => return -1,
    };

    let action = engine.composer.process_key(&KeyEvent::from_char(ch));
    translate_action(engine, action, out_action);
    0
}

/// Processes a special key (e.g., 0 = Backspace, 1 = Escape, 2 = Enter, 3 = Space).
///
/// # Safety
/// `handle` and `out_action` must be valid, non-null pointers.
#[no_mangle]
pub unsafe extern "C" fn fidel_engine_process_special(
    handle: *mut FidelEngineHandle,
    special_code: c_int,
    out_action: *mut CFidelAction,
) -> c_int {
    if handle.is_null() || out_action.is_null() {
        return -1;
    }

    let engine = unsafe { &mut *handle };
    let key = match special_code {
        0 => SpecialKey::Backspace,
        1 => SpecialKey::Escape,
        2 => SpecialKey::Enter,
        3 => SpecialKey::Space,
        _ => SpecialKey::Other,
    };

    let action = engine.composer.process_key(&KeyEvent::from_special(key));
    translate_action(engine, action, out_action);
    0
}

/// Returns the engine library version string.
#[no_mangle]
pub extern "C" fn fidel_engine_version() -> *const c_char {
    c"0.1.0".as_ptr()
}

fn translate_action(
    engine: &mut FidelEngineHandle,
    action: EngineAction,
    out_action: *mut CFidelAction,
) {
    let out = unsafe { &mut *out_action };
    engine.last_text = None;
    engine.last_display = None;
    engine.last_raw = None;

    match action {
        EngineAction::PassThrough => {
            out.action_type = FidelActionType::PassThrough;
            out.text = ptr::null();
            out.display = ptr::null();
            out.raw = ptr::null();
        }
        EngineAction::Consumed => {
            out.action_type = FidelActionType::Consumed;
            out.text = ptr::null();
            out.display = ptr::null();
            out.raw = ptr::null();
        }
        EngineAction::Commit { text } => {
            let c_text = CString::new(text).unwrap_or_default();
            out.action_type = FidelActionType::Commit;
            out.text = c_text.as_ptr();
            out.display = ptr::null();
            out.raw = ptr::null();
            engine.last_text = Some(c_text);
        }
        EngineAction::SetComposition { display, raw } => {
            let c_display = CString::new(display).unwrap_or_default();
            let c_raw = CString::new(raw).unwrap_or_default();
            out.action_type = FidelActionType::SetComposition;
            out.text = ptr::null();
            out.display = c_display.as_ptr();
            out.raw = c_raw.as_ptr();
            engine.last_display = Some(c_display);
            engine.last_raw = Some(c_raw);
        }
        EngineAction::CommitAndCompose {
            committed,
            display,
            raw,
        } => {
            let c_committed = CString::new(committed).unwrap_or_default();
            let c_display = CString::new(display).unwrap_or_default();
            let c_raw = CString::new(raw).unwrap_or_default();
            out.action_type = FidelActionType::CommitAndCompose;
            out.text = c_committed.as_ptr();
            out.display = c_display.as_ptr();
            out.raw = c_raw.as_ptr();
            engine.last_text = Some(c_committed);
            engine.last_display = Some(c_display);
            engine.last_raw = Some(c_raw);
        }
        EngineAction::ClearComposition => {
            out.action_type = FidelActionType::ClearComposition;
            out.text = ptr::null();
            out.display = ptr::null();
            out.raw = ptr::null();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TOML: &str = r#"
[metadata]
id = "ffi_test"
display_name = "FFI Test"
language = "am"
script = "Ethi"
version = "1.0.0"
license = "MIT"

[[rules]]
keys = "h"
output = "ህ"

[[rules]]
keys = "ha"
output = "ሃ"
"#;

    #[test]
    fn c_abi_lifecycle_and_actions() {
        let toml_c = CString::new(TEST_TOML).unwrap();
        unsafe {
            let handle = fidel_engine_create(toml_c.as_ptr());
            assert!(!handle.is_null());

            let mut action = CFidelAction {
                action_type: FidelActionType::PassThrough,
                text: ptr::null(),
                display: ptr::null(),
                raw: ptr::null(),
            };

            // Press 'h' -> SetComposition
            let rc = fidel_engine_process_char(handle, 'h' as u32, &mut action);
            assert_eq!(rc, 0);
            assert_eq!(action.action_type, FidelActionType::SetComposition);
            let display_str = CStr::from_ptr(action.display).to_str().unwrap();
            assert_eq!(display_str, "ህ");

            // Press 'a' -> Commit "ሃ"
            let rc = fidel_engine_process_char(handle, 'a' as u32, &mut action);
            assert_eq!(rc, 0);
            assert_eq!(action.action_type, FidelActionType::Commit);
            let commit_str = CStr::from_ptr(action.text).to_str().unwrap();
            assert_eq!(commit_str, "ሃ");

            fidel_engine_destroy(handle);
        }
    }
}
