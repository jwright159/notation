use std::path::PathBuf;
use edger_bevy::bevy_prelude::*;
use edger_bevy::egui::Ui;
use edger_bevy::prelude::AppState;

use crate::prelude::{NotationState, NotationAssets, NotationTheme};

use super::kb_page::{KbPage};
use edger_bevy::prelude::{MarkDownAsset, easy_mark, EasyLinkEvent};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct MarkDownPage {
    path: String,
}

impl KbPage for MarkDownPage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        app_state: &AppState,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        Self::markdown_ui(ui, texts, app_state, state, theme, link_evts, self.path.as_str());
    }
}

impl MarkDownPage {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    pub fn markdown_ui(
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        app_state: &AppState,
        _state: &NotationState,
        _theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
        path: &str,
    ) {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        if let Some(handle) = app_state.get_asset::<MarkDownAsset>(path_buf) {
            if let Some(text) = texts.get(&handle) {
                easy_mark(ui, text.text.as_str(), link_evts);
                return;
            }
        }
        ui.label(format!("Asset Not Found: {}", path));
    }
}