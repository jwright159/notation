use bevy::prelude::*;
use bevy_egui::{EguiSettings};

pub mod top_panel;

pub struct NotationUiPlugin;

impl Plugin for NotationUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_ui_scale_factor.system());
        app.insert_resource(top_panel::TopPanelState::default());
        app.add_system(top_panel::top_panel_ui.system());
    }
}

pub fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}
