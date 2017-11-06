use std::cell::{Cell, RefCell};
use std::sync::Arc;

use natural_constants::chemistry::AtomInfo;
use orbclient::Renderer;
use orbtk::event::Event;
use orbtk::point::Point;
use orbtk::rect::Rect;
use orbtk::traits::{Click, Place};
use orbtk::widgets::Widget;

use colors::{atom_color, ColorizationMode};
use widgets::{draw_beveled_rect, draw_text, multiply_color};

pub struct AtomWidget {
    rect: Cell<Rect>,
    atom: Cell<&'static AtomInfo>,
    colorization: Cell<ColorizationMode>,
    click_callback: RefCell<Option<Arc<Fn(&AtomWidget, Point)>>>,
}

impl AtomWidget {
    pub fn new(e: &'static AtomInfo) -> Arc<Self> {
        Arc::new(AtomWidget {
            rect: Cell::new(Rect::default()),
            atom: Cell::new(e),
            colorization: Cell::new(ColorizationMode::ByCategories),
            click_callback: RefCell::new(None)
        })
    }

    pub fn atom(&self) -> &'static AtomInfo {
        &self.atom.get()
    }
}

impl Click for AtomWidget {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = *self.click_callback.borrow() {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self {
        *self.click_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl Place for AtomWidget {}

impl Widget for AtomWidget {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        let e = self.atom.get();

        let color = atom_color(e, self.colorization.get());
        let textcolor = multiply_color(&color, 0.2);

        let large_text_size = rect.width as f32 * 0.6;
        let medium_text_size = rect.width as f32 * 0.3;
        let small_text_size = rect.width as f32 * 0.2;

        draw_beveled_rect(renderer, rect.x, rect.y, rect.width, rect.height, &color);
        draw_text(renderer, &super::BOLD_FONT, large_text_size, e.name, rect.x, rect.y, rect.width, rect.height, &textcolor, true, true);
        draw_text(renderer, &super::BOLD_FONT, medium_text_size, &(e.atomic_number.to_string()), rect.x + 3, rect.y + 3, 0, 0, &textcolor, false, false);

        if small_text_size >= 6.0 {
            draw_text(renderer, &super::FONT, small_text_size, e.full_name, rect.x, rect.y + rect.height as i32 - small_text_size as i32 * 2 - 2, rect.width, 0, &textcolor, false, true);
            draw_text(renderer, &super::FONT, small_text_size, &(e.mass.to_string()), rect.x, rect.y  + rect.height as i32 - small_text_size as i32 - 2, rect.width, 0, &textcolor, false, true);
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let rect = self.rect.get();
                if rect.contains(point) && left_button {
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                    *redraw = true;
                }
            }
            _ => (),
        }

        focused
    }
}
