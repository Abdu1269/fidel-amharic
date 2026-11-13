//! Windows Text Services Framework (TSF) in-process COM DLL for Fidel Amharic.

#![cfg_attr(windows, expect(unsafe_code, reason = "Windows COM in-process text service"))]

pub mod guids;

#[cfg(windows)]
pub mod class_factory;
#[cfg(windows)]
pub mod edit_session;
#[cfg(windows)]
pub mod registration;
#[cfg(windows)]
pub mod text_service;

#[cfg(windows)]
use windows::core::{Interface, GUID, HRESULT};
#[cfg(windows)]
use windows::Win32::Foundation::{CLASS_E_CLASSNOTAVAILABLE, E_POINTER, S_OK};
#[cfg(windows)]
use windows::Win32::System::Com::IClassFactory;

#[cfg(windows)]
use crate::class_factory::ClassFactory;
#[cfg(windows)]
use crate::guids::CLSID_FIDEL_TEXTSERVICE;

/// COM DLL entry point to retrieve the class factory.
///
/// # Safety
///
/// `rclsid`, `riid`, and `ppv` must be valid non-dangling pointers provided by the COM subsystem.
#[cfg(windows)]
#[no_mangle]
pub unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut std::ffi::c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }

    unsafe { *ppv = std::ptr::null_mut() };

    if unsafe { *rclsid } != CLSID_FIDEL_TEXTSERVICE {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    let factory: IClassFactory = ClassFactory.into();
    unsafe { factory.query(riid, ppv) }
}

/// Informs COM whether the DLL can be safely unloaded.
#[cfg(windows)]
#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}

/// Self-registration entry point (`regsvr32.exe fidel_amharic.dll`).
#[cfg(windows)]
#[no_mangle]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    match registration::register_server() {
        Ok(_) => S_OK,
        Err(e) => e.code(),
    }
}

/// Self-unregistration entry point (`regsvr32.exe /u fidel_amharic.dll`).
#[cfg(windows)]
#[no_mangle]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    match registration::unregister_server() {
        Ok(_) => S_OK,
        Err(e) => e.code(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guids_are_defined_and_valid() {
        assert_eq!(guids::LANGID_AMHARIC, 0x045E);
    }
}
