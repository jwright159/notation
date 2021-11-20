use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{shape::{Shape, SingleShape}, shapes::DoubleShape};

#[derive(Clone, Debug)]
pub struct FillPath {
    pub size: Vec2,
    pub path: String,
    pub color: Color,
    pub offset: Vec3,
    pub scale: f32,
    pub angle: f32,
}

impl Shape for FillPath {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::SvgPathShape> for FillPath {
    fn get_shape(&self) -> shapes::SvgPathShape {
        shapes::SvgPathShape {
            svg_doc_size_in_px: self.size,
            svg_path_string: self.path.clone(),
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.offset,
            rotation: Quat::from_rotation_z(self.angle),
            scale: Vec3::new(self.scale, self.scale, 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StrokePath {
    pub size: Vec2,
    pub path: String,
    pub color: Color,
    pub line_width: f32,
    pub offset: Vec3,
    pub scale: f32,
    pub angle: f32,
}

impl Shape for StrokePath {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::SvgPathShape> for StrokePath {
    fn get_shape(&self) -> shapes::SvgPathShape {
        shapes::SvgPathShape {
            svg_doc_size_in_px: self.size,
            svg_path_string: self.path.clone(),
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Stroke(
            StrokeOptions::default().with_line_width(self.line_width),
        )
    }
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.offset,
            rotation: Quat::from_rotation_z(self.angle),
            scale: Vec3::new(self.scale, self.scale, 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StrokeCirclePath {
    pub radius: f32,
    pub path: StrokePath,
}

impl Shape for StrokeCirclePath {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl DoubleShape<shapes::Circle, shapes::SvgPathShape> for StrokeCirclePath {
    fn get_shape1(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.radius,
        }
    }
    fn get_shape2(&self) -> shapes::SvgPathShape {
        self.path.get_shape()
    }
    fn get_colors(&self) -> ShapeColors {
        self.path.get_colors()
    }
    fn get_draw_mode(&self) -> DrawMode {
        self.path.get_draw_mode()
    }
    fn get_transform(&self) -> Transform {
        self.path.get_transform()
    }
}

#[derive(Clone, Debug)]
pub struct StrokeRectanglePath {
    pub width: f32,
    pub height: f32,
    pub origin: shapes::RectangleOrigin,
    pub path: StrokePath,
}

impl Shape for StrokeRectanglePath {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl DoubleShape<shapes::Rectangle, shapes::SvgPathShape> for StrokeRectanglePath {
    fn get_shape1(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.width,
            height: self.height,
            origin: self.origin,
        }
    }
    fn get_shape2(&self) -> shapes::SvgPathShape {
        self.path.get_shape()
    }
    fn get_colors(&self) -> ShapeColors {
        self.path.get_colors()
    }
    fn get_draw_mode(&self) -> DrawMode {
        self.path.get_draw_mode()
    }
    fn get_transform(&self) -> Transform {
        self.path.get_transform()
    }
}