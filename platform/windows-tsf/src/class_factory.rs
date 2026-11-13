//! COM ClassFactory for creating instances of the Fidel Text Service.

#[cfg(windows)]
use windows_core::{implement, IUnknown, Interface, Ref, Result, GUID};
#[cfg(windows)]
use windows::Win32::Foundation::{CLASS_E_NOAGGREGATION, E_POINTER};
#[cfg(windows)]
use windows::Win32::System::Com::IClassFactory_Impl;

#[cfg(windows)]
use crate::text_service::TextService;

#[cfg(windows)]
#[implement(windows::Win32::System::Com::IClassFactory)]
pub struct ClassFactory;

#[cfg(windows)]
impl IClassFactory_Impl for ClassFactory_Impl {
    #[expect(
        clippy::not_unsafe_ptr_arg_deref,
        reason = "COM interface method requires raw pointer parameters"
    )]
    fn CreateInstance(
        &self,
        punkouter: Ref<'_, IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut std::ffi::c_void,
    ) -> Result<()> {
        if ppvobject.is_null() {
            return Err(windows_core::Error::from_hresult(E_POINTER));
        }

        unsafe { *ppvobject = std::ptr::null_mut() };

        if punkouter.ok().is_ok() {
            return Err(windows_core::Error::from_hresult(CLASS_E_NOAGGREGATION));
        }

        let text_service: windows::Win32::UI::TextServices::ITfTextInputProcessor =
            TextService::new().into();

        unsafe { text_service.query(riid, ppvobject).ok() }
    }

    fn LockServer(&self, _flock: windows::Win32::Foundation::BOOL) -> Result<()> {
        Ok(())
    }
}
