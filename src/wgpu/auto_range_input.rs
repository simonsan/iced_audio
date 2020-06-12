//! wgpu renderer for the [`AutoRangeInput`] widget
//!
//! [`AutoRangeInput`]: ../native/auto_range_input/struct.AutoRangeInput.html

use crate::native::auto_range_input;

use iced_native::{Background, MouseCursor, Point, Rectangle};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::auto_range_input::State;
pub use crate::style::auto_range_input::{
    CircleStyle, SquareStyle, Style, StyleSheet, DefaultInvisible
};

/// This is an alias of a `crate::native` [`AutoRangeInput`] with an
/// `iced_wgpu::Renderer`.
///
/// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
pub type AutoRangeInput<'a, Message, ID> =
    auto_range_input::AutoRangeInput<'a, Message, Renderer, ID>;

impl auto_range_input::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        is_dragging: bool,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let dot: Primitive = match style {
            Style::Circle(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                let radius = bounds_size / 2.0;

                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y,
                        width: bounds_size,
                        height: bounds_size,
                    },
                    background: Background::Color(style.color),
                    border_radius: radius as u16,
                    border_width: style.border_width,
                    border_color: style.border_color,
                }
            },
            Style::Square(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y,
                        width: bounds_size,
                        height: bounds_size,
                    },
                    background: Background::Color(style.color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                }
            },
            Style::Invisible => Primitive::None,
        };

        (dot, MouseCursor::default())
    }
}
