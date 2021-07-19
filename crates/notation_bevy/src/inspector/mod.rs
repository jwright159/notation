use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;

use bevy_inspector_egui::{InspectableRegistry, InspectorPlugin};

pub mod syllable;

pub struct NotationInspectorPlugins;

impl PluginGroup for NotationInspectorPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        //external plugins
        group.add(bevy_inspector_egui::WorldInspectorPlugin::new());
        //internal plugins
        group.add(InspectPlugin);
    }
}

pub struct InspectPlugin;

impl Plugin for InspectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<crate::prelude::BevyConfig>::new());
        app.add_startup_system(register_inspectors.system());
    }
}

fn register_inspectors(mut registry: ResMut<InspectableRegistry>) {
    registry.register_raw(syllable::inspector_ui);
}