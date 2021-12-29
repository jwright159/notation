//! Experimental markup language

mod easy_mark_editor;
pub mod easy_mark_parser;
mod easy_mark_viewer;

pub use easy_mark_editor::EasyMarkEditor;
pub use easy_mark_parser as parser;
pub use easy_mark_viewer::easy_mark;
pub use easy_mark_viewer::label_from_style;
pub use easy_mark_parser::Style as EasyMarkStyle;