use std::sync::Arc;

use bevy::prelude::*;
use notation_model::prelude::{BarPosition, Duration, ProtoEntry, Tab};

use crate::prelude::{BevyConfig, ConfigChangedEvent, EntryState, LyonShapeOp, TabState};

use super::pos_indicator::{PosIndicator, PosIndicatorData};

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
        app.add_system(on_add_tab_state.system());
        app.add_system(on_time.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<ConfigChangedEvent>,
    config: Res<BevyConfig>,
    indicator_query: Query<(Entity, &PosIndicatorData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in indicator_query.iter() {
            PosIndicator::update(&mut commands, &config, entity, data);
        }
    }
}

fn on_add_tab_state(
    mut commands: Commands,
    config: Res<BevyConfig>,
    state_query: Query<(Entity, &TabState), Added<TabState>>,
) {
    for (entity, _state) in state_query.iter() {
        PosIndicator::create(&mut commands, entity, &config, PosIndicatorData::default());
    }
}

fn on_time(
    _commands: Commands,
    time: Res<Time>,
    config: Res<BevyConfig>,
    mut query: Query<(&Arc<Tab>, &mut TabState, &mut Transform)>,
    mut entry_query: Query<(
        Entity,
        &Arc<ProtoEntry>,
        &Duration,
        &BarPosition,
        &mut EntryState,
    )>,
) {
    for (tab, mut state, mut transform) in query.iter_mut() {
        let (changed, end_passed) = state.tick(time.delta_seconds());
        if changed {
            *transform = config.grid.calc_pos_transform(tab, state.pos.tab);
            for (_entity, _entry, duration, position, mut entry_state) in entry_query.iter_mut() {
                if state.is_in_range(*position) {
                    if entry_state.is_idle() && state.pos.is_passed(position) {
                        *entry_state = EntryState::Playing;
                    } else if entry_state.is_playing()
                        && state.pos.is_passed_with(position, duration)
                    {
                        *entry_state = EntryState::Played;
                    } else if end_passed && entry_state.is_played() {
                        *entry_state = EntryState::Idle;
                    }
                }
            }
        }
    }
}
