//! Core TSF Text Service implementation for Fidel Amharic.

#[cfg(windows)]
use std::sync::Mutex;
#[cfg(windows)]
use windows_core::{implement, IUnknownImpl, Interface, Ref, Result, GUID};
#[cfg(windows)]
use windows::Win32::Foundation::{BOOL, LPARAM, WPARAM};
#[cfg(windows)]
use windows::Win32::UI::TextServices::{
    ITfComposition, ITfCompositionSink, ITfCompositionSink_Impl, ITfContext, ITfEditSession,
    ITfKeyEventSink, ITfKeyEventSink_Impl, ITfKeystrokeMgr, ITfTextInputProcessor,
    ITfTextInputProcessorEx, ITfTextInputProcessorEx_Impl, ITfTextInputProcessor_Impl,
    ITfThreadMgr, TF_ES_READWRITE, TF_ES_SYNC,
};

#[cfg(windows)]
use fidel_core::profile::Profile;
#[cfg(windows)]
use fidel_engine::{Composer, EngineAction, KeyEvent, SpecialKey};

#[cfg(windows)]
use crate::edit_session::TextEditSession;

#[cfg(windows)]
const EMBEDDED_PROFILE: &str =
    include_str!("../../../profiles/power_geez_phonetic_unicode.toml");

#[cfg(windows)]
#[implement(
    ITfTextInputProcessor,
    ITfTextInputProcessorEx,
    ITfKeyEventSink,
    ITfCompositionSink
)]
pub struct TextService {
    client_id: Mutex<u32>,
    thread_mgr: Mutex<Option<ITfThreadMgr>>,
    composer: Mutex<Composer>,
    active_composition: Mutex<Option<ITfComposition>>,
}

#[cfg(windows)]
impl TextService {
    pub fn new() -> Self {
        let profile = Profile::from_toml(EMBEDDED_PROFILE)
            .expect("Embedded Power Geez profile must be valid");
        let composer = Composer::new(&profile);

        Self {
            client_id: Mutex::new(0),
            thread_mgr: Mutex::new(None),
            composer: Mutex::new(composer),
            active_composition: Mutex::new(None),
        }
    }
}

#[cfg(windows)]
impl Default for TextService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl ITfTextInputProcessor_Impl for TextService_Impl {
    fn Activate(&self, ptim: Ref<'_, ITfThreadMgr>, tid: u32) -> Result<()> {
        let thread_mgr: ITfThreadMgr = ptim.ok()?.clone();

        *self.client_id.lock().unwrap() = tid;
        *self.thread_mgr.lock().unwrap() = Some(thread_mgr.clone());

        let keystroke_mgr: ITfKeystrokeMgr = thread_mgr.cast()?;
        let sink: ITfKeyEventSink = self.to_object().cast()?;
        unsafe { keystroke_mgr.AdviseKeyEventSink(tid, &sink, true)? };

        Ok(())
    }

    fn Deactivate(&self) -> Result<()> {
        let tid = *self.client_id.lock().unwrap();
        if let Some(thread_mgr) = self.thread_mgr.lock().unwrap().take() {
            if let Ok(keystroke_mgr) = thread_mgr.cast::<ITfKeystrokeMgr>() {
                unsafe {
                    let _ = keystroke_mgr.UnadviseKeyEventSink(tid);
                }
            }
        }
        self.composer.lock().unwrap().reset();
        *self.active_composition.lock().unwrap() = None;
        Ok(())
    }
}

#[cfg(windows)]
impl ITfTextInputProcessorEx_Impl for TextService_Impl {
    fn ActivateEx(&self, ptim: Ref<'_, ITfThreadMgr>, tid: u32, _dwflags: u32) -> Result<()> {
        self.Activate(ptim, tid)
    }
}

#[cfg(windows)]
impl ITfKeyEventSink_Impl for TextService_Impl {
    fn OnSetFocus(&self, _fforeground: BOOL) -> Result<()> {
        Ok(())
    }

    fn OnTestKeyDown(
        &self,
        _pic: Ref<'_, ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        let vk = wparam.0 as u32;
        let is_handled = (0x20..=0x7E).contains(&vk)
            || vk == 0x08 // VK_BACK
            || vk == 0x1B // VK_ESCAPE
            || vk == 0x0D // VK_RETURN
            || vk == 0x20; // VK_SPACE

        Ok(BOOL::from(is_handled))
    }

    fn OnKeyDown(
        &self,
        pic: Ref<'_, ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        let context = match pic.ok() {
            Ok(c) => c,
            Err(_) => return Ok(BOOL(0)),
        };

        let vk = wparam.0 as u32;
        let key_event = if vk == 0x08 {
            KeyEvent::from_special(SpecialKey::Backspace)
        } else if vk == 0x1B {
            KeyEvent::from_special(SpecialKey::Escape)
        } else if vk == 0x0D {
            KeyEvent::from_special(SpecialKey::Enter)
        } else if vk == 0x20 {
            KeyEvent::from_special(SpecialKey::Space)
        } else if let Some(ch) = char::from_u32(vk) {
            KeyEvent::from_char(ch)
        } else {
            return Ok(BOOL(0));
        };

        let action = {
            let mut comp = self.composer.lock().unwrap();
            comp.process_key(&key_event)
        };

        match action {
            EngineAction::PassThrough => Ok(BOOL(0)),
            EngineAction::Consumed => Ok(BOOL(1)),
            EngineAction::Commit { text } => {
                let comp_opt = self.active_composition.lock().unwrap().take();
                let session: ITfEditSession =
                    TextEditSession::new(context.clone(), text, true, comp_opt).into();
                let tid = *self.client_id.lock().unwrap();
                unsafe {
                    let _ = context.RequestEditSession(tid, &session, TF_ES_READWRITE | TF_ES_SYNC);
                }
                Ok(BOOL(1))
            }
            EngineAction::SetComposition { display, .. } => {
                let comp_opt = self.active_composition.lock().unwrap().clone();
                let session: ITfEditSession =
                    TextEditSession::new(context.clone(), display, false, comp_opt).into();
                let tid = *self.client_id.lock().unwrap();
                unsafe {
                    let _ = context.RequestEditSession(tid, &session, TF_ES_READWRITE | TF_ES_SYNC);
                }
                Ok(BOOL(1))
            }
            EngineAction::CommitAndCompose { committed, display, .. } => {
                let comp_opt = self.active_composition.lock().unwrap().take();
                let session: ITfEditSession =
                    TextEditSession::new(context.clone(), committed, true, comp_opt).into();
                let tid = *self.client_id.lock().unwrap();
                unsafe {
                    let _ = context.RequestEditSession(tid, &session, TF_ES_READWRITE | TF_ES_SYNC);
                }
                let session2: ITfEditSession =
                    TextEditSession::new(context.clone(), display, false, None).into();
                unsafe {
                    let _ = context.RequestEditSession(tid, &session2, TF_ES_READWRITE | TF_ES_SYNC);
                }
                Ok(BOOL(1))
            }
            EngineAction::ClearComposition => {
                *self.active_composition.lock().unwrap() = None;
                Ok(BOOL(1))
            }
        }
    }

    fn OnTestKeyUp(
        &self,
        _pic: Ref<'_, ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL(0))
    }

    fn OnKeyUp(
        &self,
        _pic: Ref<'_, ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL(0))
    }

    fn OnPreservedKey(
        &self,
        _pic: Ref<'_, ITfContext>,
        _rguid: *const GUID,
    ) -> Result<BOOL> {
        Ok(BOOL(0))
    }
}

#[cfg(windows)]
impl ITfCompositionSink_Impl for TextService_Impl {
    fn OnCompositionTerminated(
        &self,
        _ecwrite: u32,
        _pcomposition: Ref<'_, ITfComposition>,
    ) -> Result<()> {
        self.composer.lock().unwrap().reset();
        *self.active_composition.lock().unwrap() = None;
        Ok(())
    }
}
