//! GUID constants for Fidel Amharic Windows TSF Text Service.

#[cfg(windows)]
use windows::core::GUID;

#[cfg(not(windows))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GUID(pub [u8; 16]);

/// CLSID for the Fidel Amharic In-Process COM Text Service.
/// {328c04e2-63b7-4a0f-90e6-5c5cb2cb3901}
#[cfg(windows)]
pub const CLSID_FIDEL_TEXTSERVICE: GUID = GUID::from_u128(0x328c04e2_63b7_4a0f_90e6_5c5cb2cb3901);
#[cfg(not(windows))]
pub const CLSID_FIDEL_TEXTSERVICE: GUID = GUID([
    0xe2, 0x04, 0x8c, 0x32, 0xb7, 0x63, 0x0f, 0x4a, 0x90, 0xe6, 0x5c, 0x5c, 0xb2, 0xcb, 0x39, 0x01,
]);

/// Language Profile GUID for Fidel Amharic (Power Geez Unicode).
/// {86d6fe78-a029-43c2-a8c6-a6fc469cb1e1}
#[cfg(windows)]
pub const GUID_FIDEL_PROFILE: GUID = GUID::from_u128(0x86d6fe78_a029_43c2_a8c6_a6fc469cb1e1);
#[cfg(not(windows))]
pub const GUID_FIDEL_PROFILE: GUID = GUID([
    0x78, 0xfe, 0xd6, 0x86, 0x29, 0xa0, 0xc2, 0x43, 0xa8, 0xc6, 0xa6, 0xfc, 0x46, 0x9c, 0xb1, 0xe1,
]);

/// Category ID for Keyboard Text Input Processors in TSF:
/// GUID_TFCAT_TIP_KEYBOARD = {587391F1-30DE-11D4-8FAC-005004F7549C}
#[cfg(windows)]
pub const GUID_TFCAT_TIP_KEYBOARD: GUID = GUID::from_u128(0x587391f1_30de_11d4_8fac_005004f7549c);
#[cfg(not(windows))]
pub const GUID_TFCAT_TIP_KEYBOARD: GUID = GUID([
    0xf1, 0x91, 0x73, 0x58, 0xde, 0x30, 0xd4, 0x11, 0x8f, 0xac, 0x00, 0x50, 0x04, 0xf7, 0x54, 0x9c,
]);

/// Windows Language Identifier for Amharic (0x045E).
pub const LANGID_AMHARIC: u16 = 0x045E;
