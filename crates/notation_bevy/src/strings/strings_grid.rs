use bevy::prelude::*;

use crate::prelude::{LyonShapeOp, NotationTheme};
use notation_model::prelude::BarLane;

use super::single_string::{SingleString, SingleStringData, SingleStringValue};

macro_rules! impl_strings_grid {
    ($type:ident, $strings: literal) => {
        #[derive(Debug, Default)]
        pub struct $type();

        impl $type {
            pub fn add_strings(
                &self,
                commands: &mut Commands,
                theme: &NotationTheme,
                entity: Entity,
                lane: &BarLane,
            ) {
                for string in 1..=$strings {
                    SingleString::create(
                        commands,
                        theme,
                        entity,
                        SingleStringData::new(lane, SingleStringValue{
                            string: string as u8,
                            bar_size: 0.0,
                        }),
                    );
                }
            }
        }
    };
}

impl_strings_grid!(StringsGrid6, 6);
impl_strings_grid!(StringsGrid4, 4);
