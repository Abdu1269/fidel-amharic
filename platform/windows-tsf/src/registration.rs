//! Registration and unregistration logic for Windows TSF and COM.

#[cfg(windows)]
use windows::core::Result;
#[cfg(windows)]
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
#[cfg(windows)]
use windows::Win32::UI::TextServices::{
    CLSID_TF_CategoryMgr, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr,
    ITfInputProcessorProfiles,
};

#[cfg(windows)]
use crate::guids::{
    CLSID_FIDEL_TEXTSERVICE, GUID_FIDEL_PROFILE, GUID_TFCAT_TIP_KEYBOARD, LANGID_AMHARIC,
};

/// Registers the Text Service with Windows TSF and COM.
#[cfg(windows)]
pub fn register_server() -> Result<()> {
    unsafe {
        // 1. Register TSF Category
        let cat_mgr: ITfCategoryMgr =
            CoCreateInstance(&CLSID_TF_CategoryMgr, None, CLSCTX_INPROC_SERVER)?;
        cat_mgr.RegisterCategory(
            &CLSID_FIDEL_TEXTSERVICE,
            &GUID_TFCAT_TIP_KEYBOARD,
            &CLSID_FIDEL_TEXTSERVICE,
        )?;

        // 2. Register Language Profile (Amharic 0x045E)
        let profiles: ITfInputProcessorProfiles =
            CoCreateInstance(&CLSID_TF_InputProcessorProfiles, None, CLSCTX_INPROC_SERVER)?;

        let desc: Vec<u16> = "Fidel Amharic (Power Geez Unicode)\0".encode_utf16().collect();
        profiles.Register(&CLSID_FIDEL_TEXTSERVICE)?;

        profiles.AddLanguageProfile(
            &CLSID_FIDEL_TEXTSERVICE,
            LANGID_AMHARIC,
            &GUID_FIDEL_PROFILE,
            &desc,
            &[],
            0,
        )?;
    }

    Ok(())
}

/// Unregisters the Text Service from Windows TSF and COM.
#[cfg(windows)]
pub fn unregister_server() -> Result<()> {
    unsafe {
        if let Ok(cat_mgr) =
            CoCreateInstance::<_, ITfCategoryMgr>(&CLSID_TF_CategoryMgr, None, CLSCTX_INPROC_SERVER)
        {
            let _ = cat_mgr.UnregisterCategory(
                &CLSID_FIDEL_TEXTSERVICE,
                &GUID_TFCAT_TIP_KEYBOARD,
                &CLSID_FIDEL_TEXTSERVICE,
            );
        }

        if let Ok(profiles) = CoCreateInstance::<_, ITfInputProcessorProfiles>(
            &CLSID_TF_InputProcessorProfiles,
            None,
            CLSCTX_INPROC_SERVER,
        ) {
            let _ = profiles.Unregister(&CLSID_FIDEL_TEXTSERVICE);
        }
    }

    Ok(())
}
