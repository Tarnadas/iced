use crate::backend::{self, Backend};
use crate::{Primitive, Renderer};
use iced_native::{
    mouse, Font, HorizontalAlignment, Point, Rectangle, VerticalAlignment,
};
use iced_style::menu;

pub use iced_native::combo_box::State;
pub use iced_style::combo_box::{Style, StyleSheet};

/// A widget allowing the selection of a single value from a list of options.
pub type ComboBox<'a, T, Message, Backend> =
    iced_native::ComboBox<'a, T, Message, Renderer<Backend>>;

impl<B> iced_native::combo_box::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    const DEFAULT_PADDING: u16 = 5;

    fn menu_style(style: &Box<dyn StyleSheet>) -> menu::Style {
        style.menu()
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        selected: Option<String>,
        padding: u16,
        text_size: u16,
        font: Font,
        style: &Box<dyn StyleSheet>,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_mouse_over {
            style.hovered()
        } else {
            style.active()
        };

        let background = Primitive::Quad {
            bounds,
            background: style.background,
            border_color: style.border_color,
            border_width: style.border_width,
            border_radius: style.border_radius,
        };

        let arrow_down = Primitive::Text {
            content: B::ARROW_DOWN_ICON.to_string(),
            font: B::ICON_FONT,
            size: bounds.height * style.icon_size,
            bounds: Rectangle {
                x: bounds.x + bounds.width - f32::from(padding) * 2.0,
                y: bounds.center_y(),
                ..bounds
            },
            color: style.text_color,
            horizontal_alignment: HorizontalAlignment::Right,
            vertical_alignment: VerticalAlignment::Center,
        };

        (
            Primitive::Group {
                primitives: if let Some(label) = selected {
                    let label = Primitive::Text {
                        content: label,
                        size: f32::from(text_size),
                        font,
                        color: style.text_color,
                        bounds: Rectangle {
                            x: bounds.x + f32::from(padding),
                            y: bounds.center_y(),
                            ..bounds
                        },
                        horizontal_alignment: HorizontalAlignment::Left,
                        vertical_alignment: VerticalAlignment::Center,
                    };

                    vec![background, label, arrow_down]
                } else {
                    vec![background, arrow_down]
                },
            },
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}
