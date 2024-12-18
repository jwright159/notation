use std::sync::Arc;

use edger_bevy::bevy_prelude::*;

use notation_model::prelude::*;

use crate::bar::bar_view::BarView;
use crate::prelude::TabBars;
use crate::rhythm::rhythm_view::RhythmView;
use crate::prelude::NotationLayout;
use edger_bevy::prelude::{DoLayoutEvent, LayoutData};

use super::tab_chords::TabChords;
use super::tab_content::TabContent;
use super::tab_control::TabControl;
use super::tab_header::TabHeader;
use super::tab_view::TabView;

#[derive(Event, Debug)]
pub struct AddTabEvent(pub Arc<Tab>);

#[derive(Event, Debug)]
pub struct TabBarsResizedPreEvent(pub Entity);

#[derive(Event, Debug)]
pub struct TabBarsResizedEvent(pub Arc<Vec<(BarView, LayoutData)>>);

pub type TabViewDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabView>;
pub type TabContentDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabContent>;
pub type TabHeaderDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabHeader>;
pub type TabControlDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabControl>;
pub type TabChordsDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabChords>;
pub type TabBarsDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabBars>;
pub type BarViewDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, BarView>;
pub type RhythmViewDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, RhythmView>;
