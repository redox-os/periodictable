#![feature(const_fn)]

extern crate orbclient;
extern crate orbfont;
extern crate orbtk;

use orbtk::{Window, Rect, Point};
use orbtk::traits::{Place, Click};

use element_widget::ElementWidget;
use element_data::{ELEMENTS, Position};
use element_colors::ColorizationMode;

pub mod element_colors;
pub mod element_data;
pub mod element_widget;
pub mod gfxutils;

const ELEMENT_WIDTH: u32 = 52;
const ELEMENT_HEIGHT: u32 = 64;
const PADDING: u32 = 16;
const WINDOW_WIDTH: u32 = ELEMENT_WIDTH * 18 + PADDING * 2;
const WINDOW_HEIGHT: u32 = ELEMENT_HEIGHT * 10 + PADDING * 2;

fn main() {
    let window = Window::new(Rect::new(10, 10, WINDOW_WIDTH, WINDOW_HEIGHT), "Periodic table");

    for e in ELEMENTS.iter() {
        let x = (match e.position {
            Position::MainTable { group, .. } => group - 1,
            Position::Lanthanides { column } => 2 + (column - 1),
            Position::Actinides { column } => 2 + (column - 1),
        } * ELEMENT_WIDTH + PADDING) as i32;

        let y = (match e.position {
            Position::MainTable { period, .. } => period - 1,
            Position::Lanthanides { .. } => 8,
            Position::Actinides { .. } => 9,
        } * ELEMENT_HEIGHT + PADDING) as i32;

        let widget = ElementWidget::new(&e);
        widget.position(x, y)
            .size(ELEMENT_WIDTH, ELEMENT_HEIGHT)
            .on_click(move |_widget: &ElementWidget, _point: Point| {
                // on_click test
                _widget.colorization.set(ColorizationMode::ByStates);
            });
        window.add(&widget);
    }

    window.exec();
}