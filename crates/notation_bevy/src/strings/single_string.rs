use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{LaneData, LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct SingleStringValue {
    pub string: u8,
    pub bar_size: f32,
}

pub type SingleStringData = LaneData<SingleStringValue>;

pub struct SingleString<'a> {
    theme: &'a NotationTheme,
    data: SingleStringData,
}

impl<'a> LyonShape<shapes::Line> for SingleString<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:String {}",
            self.data.bar_props.bar_ordinal, self.data.value.string
        )
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(Vec2::ZERO, Vec2::new(self.data.value.bar_size, 0.0))
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.strings.string_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.guitar.get_string_width(self.data.value.string);
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let y = -1.0 * (self.data.value.string as f32 - 0.5) * self.theme.strings.string_space;
        Transform::from_xyz(0.0, y, self.theme.strings.string_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, SingleStringData, shapes::Line, SingleString<'a>>
    for SingleString<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: SingleStringData) -> SingleString<'a> {
        SingleString::<'a> { theme, data }
    }
}