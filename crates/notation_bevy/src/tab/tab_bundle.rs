use bevy::prelude::*;
use bevy_utils::prelude::LayoutData;
use std::sync::Arc;

use notation_model::prelude::Tab;

use crate::bar::bar_layout::BarLayoutData;

use super::tab_state::TabState;
use super::tab_view::TabView;

#[derive(Bundle)]
pub struct TabBundle {
    pub name: Name,
    pub tab: Arc<Tab>,
    pub bar_layouts: Arc<Vec<BarLayoutData>>,
    pub state: TabState,
    pub view: Arc<TabView>,
    pub layout: LayoutData,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl TabBundle {
    pub fn new(tab: Arc<Tab>, bar_layouts: Arc<Vec<BarLayoutData>>) -> Self {
        let name = tab.to_string().as_str().into();
        let state = TabState::new(&tab);
        let view = Arc::new(TabView::new(tab.clone()));
        Self {
            name,
            tab,
            bar_layouts,
            state,
            view,
            layout: LayoutData::default(),
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
