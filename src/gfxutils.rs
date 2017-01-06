extern crate orbclient;
extern crate orbfont;

use orbfont::Font;
use orbclient::{Color, Renderer};
use std::cmp;

pub fn mult_color(color: &Color, n: f32) -> Color {
    Color::rgb(
        cmp::max(cmp::min((color.r() as f32 * n) as i32, 255), 0) as u8,
        cmp::max(cmp::min((color.g() as f32 * n) as i32, 255), 0) as u8,
        cmp::max(cmp::min((color.b() as f32 * n) as i32, 255), 0) as u8,
    )
}

pub fn draw_beveled_rect(renderer: &mut Renderer, x1: i32, y1: i32, w: u32, h: u32, color: &Color) {
    let shadow = mult_color(&color, 0.35);
    let highlight = mult_color(&color, 1.25);

    let x2 = x1 + w as i32 - 1;
    let y2 = y1 + h as i32 - 1;

    renderer.rect(x1, y1, w, h, *color);
    renderer.line(x1, y1, x2, y1, highlight);
    renderer.line(x1, y1, x1, y2, highlight);
    renderer.line(x2, y2, x2, y1, shadow);
    renderer.line(x2, y2, x1, y2, shadow);
}

pub fn draw_text(renderer: &mut Renderer, font: &Font, size: f32, text: &str, x1: i32, y1: i32, w: u32, h: u32, color: &Color, vcenter: bool, hcenter: bool) {
    let text = font.render(text, size);

    let x = if hcenter { x1 + w as i32 / 2 - text.width() as i32 / 2 } else { x1 };
    let y = if vcenter { y1 + h as i32 / 2 - text.height() as i32 / 2 } else { y1 };

    // TODO: Fix this
    //text.draw(renderer, x, y, *color);
}
