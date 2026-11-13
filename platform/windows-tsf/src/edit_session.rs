//! TSF Edit Sessions for safely modifying document contexts and compositions.

#[cfg(windows)]
use windows_core::{implement, Interface, Result};
#[cfg(windows)]
use windows::Win32::UI::TextServices::{
    ITfComposition, ITfContext, ITfEditSession_Impl, ITfInsertAtSelection,
    TF_IAS_NOQUERY,
};

/// Edit session that inserts or updates text in the focused context.
#[cfg(windows)]
#[implement(windows::Win32::UI::TextServices::ITfEditSession)]
pub struct TextEditSession {
    context: ITfContext,
    text: String,
    is_commit: bool,
    active_composition: Option<ITfComposition>,
}

#[cfg(windows)]
impl TextEditSession {
    /// Creates a new edit session for committing or updating preedit text.
    pub fn new(
        context: ITfContext,
        text: String,
        is_commit: bool,
        active_composition: Option<ITfComposition>,
    ) -> Self {
        Self {
            context,
            text,
            is_commit,
            active_composition,
        }
    }
}

#[cfg(windows)]
impl ITfEditSession_Impl for TextEditSession_Impl {
    fn DoEditSession(&self, ec: u32) -> Result<()> {
        let wide_chars: Vec<u16> = self.text.encode_utf16().collect();

        if self.is_commit {
            // End any active composition first
            if let Some(comp) = &self.active_composition {
                unsafe { comp.EndComposition(ec)? };
            }

            // Insert text at selection
            let insert_at_sel: ITfInsertAtSelection = self.context.cast()?;
            unsafe {
                let _ = insert_at_sel.InsertTextAtSelection(
                    ec,
                    TF_IAS_NOQUERY,
                    &wide_chars,
                )?;
            }
        } else {
            // Update composition / preedit
            if let Some(comp) = &self.active_composition {
                let range = unsafe { comp.GetRange()? };
                unsafe { range.SetText(ec, 0, &wide_chars)? };
            } else {
                let insert_at_sel: ITfInsertAtSelection = self.context.cast()?;
                unsafe {
                    let _ = insert_at_sel.InsertTextAtSelection(
                        ec,
                        TF_IAS_NOQUERY,
                        &wide_chars,
                    )?;
                }
            }
        }

        Ok(())
    }
}
