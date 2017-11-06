mod atom_widget;
mod legend_widget;

pub use self::atom_widget::AtomWidget;
pub use self::legend_widget::LegendWidget;

use std::cmp;

use orbclient::{Color, Renderer};
use orbfont::Font;

lazy_static! {
    pub static ref FONT: Font = Font::find(None, None, None).expect("Failed to load FONT");
    pub static ref BOLD_FONT: Font = Font::find(None, None, Some("Bold")).expect("Failed to load BOLD_FONT");
}

pub fn multiply_color(color: &Color, n: f32) -> Color {
    Color::rgb(
        cmp::max(cmp::min((color.r() as f32 * n) as i32, 255), 0) as u8,
        cmp::max(cmp::min((color.g() as f32 * n) as i32, 255), 0) as u8,
        cmp::max(cmp::min((color.b() as f32 * n) as i32, 255), 0) as u8,
    )
}

pub fn draw_beveled_rect<R: Renderer + ?Sized>(renderer: &mut R, x1: i32, y1: i32, w: u32, h: u32, color: &Color) {
    let x2 = x1 + w as i32 - 1;
    let y2 = y1 + h as i32 - 1;

    renderer.linear_gradient(x1, y1, w, h, x1, y1, x2, y2, *color, multiply_color(&color, 0.80));
    let shadow = multiply_color(&color, 0.35);
    let highlight = multiply_color(&color, 1.25);
    renderer.line(x1, y1, x2, y1, highlight);
    renderer.line(x1, y1, x1, y2, highlight);
    renderer.line(x2, y2, x2, y1, shadow);
    renderer.line(x2, y2, x1, y2, shadow);
}

pub fn draw_text<R: Renderer + ?Sized>(renderer: &mut R, font: &Font, size: f32, text: &str, x1: i32, y1: i32, w: u32, h: u32, color: &Color, vcenter: bool, hcenter: bool) {
    let text = font.render(text, size);

    let x = if hcenter { x1 + w as i32 / 2 - text.width() as i32 / 2 } else { x1 };
    let y = if vcenter { y1 + h as i32 / 2 - text.height() as i32 / 2 } else { y1 };

    text.draw(renderer, x, y, *color);
}
