#![warn(missing_docs)]
//! `widgets` is a collection of types that implement [`Widget`] or [`StatefulWidget`] or both.
//!
//! Widgets are created for each frame as they are consumed after rendered.
//! They are not meant to be stored but used as *commands* to draw common figures in the UI.
//!
//! The available widgets are:
//! - [`Block`]: a basic widget that draws a block with optional borders, titles and styles.
//! - [`BarChart`]: displays multiple datasets as bars with optional grouping.
//! - [`calendar::Monthly`]: displays a single month.
//! - [`Canvas`]: draws arbitrary shapes using drawing characters.
//! - [`Chart`]: displays multiple datasets as a lines or scatter graph.
//! - [`Clear`]: clears the area it occupies. Useful to render over previously drawn widgets.
//! - [`Gauge`]: displays progress percentage using block characters.
//! - [`LineGauge`]: display progress as a line.
//! - [`List`]: displays a list of items and allows selection.
//! - [`Paragraph`]: displays a paragraph of optionally styled and wrapped text.
//! - [`Scrollbar`]: displays a scrollbar.
//! - [`Sparkline`]: display a single data set as a sparkline.
//! - [`Table`]: displays multiple rows and columns in a grid and allows selection.
//! - [`Tabs`]: displays a tab bar and allows selection.
//!
//! [`Canvas`]: crate::widgets::canvas::Canvas
mod barchart;
pub mod block;
mod borders;
#[cfg(feature = "widget-calendar")]
pub mod calendar;
pub mod canvas;
mod chart;
mod clear;
mod gauge;
mod list;
mod paragraph;
mod reflow;
mod scrollbar;
mod sparkline;
mod table;
mod tabs;

pub use self::{
    barchart::{Bar, BarChart, BarGroup},
    block::{Block, BorderType, Padding},
    borders::*,
    chart::{Axis, Chart, Dataset, GraphType, LegendPosition},
    clear::Clear,
    gauge::{Gauge, LineGauge},
    list::{List, ListDirection, ListItem, ListState},
    paragraph::{Paragraph, Wrap},
    scrollbar::{ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState},
    sparkline::{RenderDirection, Sparkline},
    table::{Cell, HighlightSpacing, Row, Table, TableState},
    tabs::Tabs,
};
use crate::{buffer::Buffer, layout::Rect};

/// A `Widget` is a type that can be drawn on a [`Buffer`] in a given [`Rect`].
///
/// Prior to Ratatui 0.26.0, widgets generally were created for each frame as they were consumed
/// during rendering. This meant that they were not meant to be stored but used as *commands* to
/// draw common figures in the UI.
///
/// Starting with Ratatui 0.26.0, we added a new [`WidgetRef`] trait and implemented this on all the
/// internal widgets. This allows you to store a reference to a widget and render it later. It also
/// allows you to render boxed widgets. This is useful when you want to store a collection of
/// widgets with different types. You can then iterate over the collection and render each widget.
///
/// The `Widget` trait can still be implemented, however, it is recommended to implement `WidgetRef`
/// and add an implementation of `Widget` that calls `WidgetRef::render_ref`. This pattern should be
/// used where backwards compatibility is required (all the internal widgets use this approach).
///
/// A blanket implementation of `Widget` for `&W` where `W` implements `WidgetRef` is provided.
/// Widget is also implemented for `&str` and `String` types.
///
/// # Examples
///
/// ```rust,no_run
/// use ratatui::{backend::TestBackend, prelude::*, widgets::*};
/// # let backend = TestBackend::new(5, 5);
/// # let mut terminal = Terminal::new(backend).unwrap();
///
/// terminal.draw(|frame| {
///     frame.render_widget(Clear, frame.size());
/// });
/// ```
///
/// It's common to render widgets inside other widgets:
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// struct MyWidget;
///
/// impl Widget for MyWidget {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         Line::raw("Hello").render(area, buf);
///     }
/// }
/// ```
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That is the only method required
    /// to implement a custom widget.
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized;
}

/// A `WidgetRef` is a trait that allows rendering a widget by reference.
///
/// This trait is useful when you want to store a reference to a widget and render it later. It also
/// allows you to render boxed widgets.
///
/// Boxed widgets allow you to store widgets with a type that is not known at compile time. This is
/// useful when you want to store a collection of widgets with different types. You can then iterate
/// over the collection and render each widget.
///
/// This trait was introduced in Ratatui 0.26.0 and is implemented for all the internal widgets.
/// Implementors should prefer to implement this over the `Widget` trait and add an implementation
/// of `Widget` that calls `WidgetRef::render_ref` where backwards compatibility is required.
///
/// A blanket implementation of `Widget` for `&W` where `W` implements `WidgetRef` is provided.
///
/// A blanket implementation of `WidgetRef` for `Option<W>` where `W` implements `WidgetRef` is
/// provided. This is a convenience approach to make it easier to attach child widgets to parent
/// widgets. It allows you to render an optional widget by reference.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use ratatui::{prelude::*, widgets::*};
///
/// struct Greeting;
///
/// struct Farewell;
///
/// impl WidgetRef for Greeting {
///     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
///         Line::raw("Hello").render(area, buf);
///     }
/// }
///
/// /// Only needed for backwards compatibility
/// impl Widget for Greeting {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         self.render_ref(area, buf);
///     }
/// }
///
/// impl WidgetRef for Farewell {
///     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
///         Line::raw("Goodbye").right_aligned().render(area, buf);
///     }
/// }
///
/// /// Only needed for backwards compatibility
/// impl Widget for Farewell {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         self.render_ref(area, buf);
///     }
/// }
///
/// # fn render(area: Rect, buf: &mut Buffer) {
/// let greeting = Greeting;
/// let farewell = Farewell;
///
/// // these calls do not consume the widgets, so they can be used again later
/// greeting.render_ref(area, buf);
/// farewell.render_ref(area, buf);
///
/// // a collection of widgets with different types
/// let widgets: Vec<Box<dyn WidgetRef>> = vec![Box::new(greeting), Box::new(farewell)];
/// for widget in widgets {
///     widget.render_ref(area, buf);
/// }
/// # }
/// # }
/// ```
#[stability::unstable(feature = "widget-ref")]
pub trait WidgetRef {
    /// Draws the current state of the widget in the given buffer. That is the only method required
    /// to implement a custom widget.
    fn render_ref(&self, area: Rect, buf: &mut Buffer);
}

/// This allows you to render a widget by reference.
impl<W: WidgetRef> Widget for &W {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

/// Renders a string slice as a widget.
pub fn render_str(s: &str, area: Rect, buf: &mut Buffer) {
    buf.set_string(area.x, area.y, s, crate::style::Style::default());
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;
    use crate::prelude::*;

    #[fixture]
    fn buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 1))
    }

    mod widget {
        use super::*;

        struct Greeting;

        impl Widget for Greeting {
            fn render(self, area: Rect, buf: &mut Buffer) {
                Line::from("Hello").render(area, buf);
            }
        }

        #[rstest]
        fn render(mut buf: Buffer) {
            let widget = Greeting;
            widget.render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }
    }

    mod widget_ref {
        use super::*;

        struct Greeting;
        struct Farewell;

        impl WidgetRef for Greeting {
            fn render_ref(&self, area: Rect, buf: &mut Buffer) {
                Line::from("Hello").render(area, buf);
            }
        }

        impl WidgetRef for Farewell {
            fn render_ref(&self, area: Rect, buf: &mut Buffer) {
                Line::from("Goodbye").right_aligned().render(area, buf);
            }
        }

        #[rstest]
        fn render_ref(mut buf: Buffer) {
            let widget = Greeting;
            widget.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }

        /// Ensure that the blanket implementation of `Widget` for `&W` where `W` implements
        /// `WidgetRef` works as expected.
        #[rstest]
        fn blanket_render(mut buf: Buffer) {
            let widget = &Greeting;
            widget.render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }

        #[rstest]
        fn box_render_ref(mut buf: Buffer) {
            let widget: Box<dyn WidgetRef> = Box::new(Greeting);
            widget.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }

        #[rstest]
        fn vec_box_render(mut buf: Buffer) {
            let widgets: Vec<Box<dyn WidgetRef>> = vec![Box::new(Greeting), Box::new(Farewell)];
            for widget in widgets {
                widget.render_ref(buf.area, &mut buf);
            }
            assert_eq!(buf, Buffer::with_lines(["Hello        Goodbye"]));
        }
    }

    #[fixture]
    fn state() -> String {
        "world".to_string()
    }

    mod str {
        use super::*;

        #[rstest]
        fn render(mut buf: Buffer) {
            render_str("hello world", buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }
    }

    mod string {
        use super::*;

        #[rstest]
        fn render(mut buf: Buffer) {
            render_str(&String::from("hello world"), buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }
    }
}
