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

/// Renders a string slice as a widget.
pub fn render_str(s: &str, area: Rect, buf: &mut Buffer) {
    buf.set_string(area.x, area.y, s, crate::style::Style::default());
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 1))
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
