use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use orbtk::event::Event;
use orbtk::point::Point;
use orbtk::rect::Rect;
use orbtk::traits::{Click, Place};
use orbtk::widgets::Widget;

use orbfont::Font;

use natural_constants::chemistry::AtomInfo;
use gfxutils::{mult_color, draw_beveled_rect, draw_text};
use element_colors::{get_element_color, ColorizationMode};

pub struct ElementWidget {
    rect: Cell<Rect>,
    element: Cell<&'static AtomInfo>,
    colorization: Cell<ColorizationMode>,
    click_callback: RefCell<Option<Arc<Fn(&ElementWidget, Point)>>>,
}

impl ElementWidget {
    pub fn new(e: &'static AtomInfo) -> Arc<Self> {
        Arc::new(ElementWidget {
            rect: Cell::new(Rect::default()),
            element: Cell::new(e),
            colorization: Cell::new(ColorizationMode::ByCategories),
            click_callback: RefCell::new(None)
        })
    }

    pub fn element(&self) -> &'static AtomInfo {
        &self.element.get()
    }
}

impl Click for ElementWidget {
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

impl Place for ElementWidget {}

impl Widget for ElementWidget {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        let e = self.element.get();

        let font = Font::find(None, None, None).unwrap();
        let boldfont = Font::find(None, None, Some("Bold")).unwrap();

        let color = get_element_color(e, self.colorization.get());
        let textcolor = mult_color(&color, 0.2);

        let large_text_size = rect.width as f32 * 0.6;
        let medium_text_size = rect.width as f32 * 0.3;
        let small_text_size = rect.width as f32 * 0.2;

        draw_beveled_rect(renderer, rect.x, rect.y, rect.width, rect.height, &color);
        draw_text(renderer, &boldfont, large_text_size, e.name, rect.x, rect.y, rect.width, rect.height, &textcolor, true, true);
        draw_text(renderer, &boldfont, medium_text_size, &(e.atomic_number.to_string()), rect.x + 3, rect.y + 3, 0, 0, &textcolor, false, false);

        if small_text_size >= 6.0 {
            draw_text(renderer, &font, small_text_size, e.full_name, rect.x, rect.y + rect.height as i32 - small_text_size as i32 * 2 - 2, rect.width, 0, &textcolor, false, true);
            draw_text(renderer, &font, small_text_size, &(e.mass.to_string()), rect.x, rect.y  + rect.height as i32 - small_text_size as i32 - 2, rect.width, 0, &textcolor, false, true);
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
