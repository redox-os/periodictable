use orbclient::{Color, Renderer};
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use orbtk::event::Event;
use orbtk::point::Point;
use orbtk::rect::Rect;
use orbtk::traits::{Click, Place};
use orbtk::widgets::Widget;

use orbfont::Font;

use gfxutils::{mult_color, draw_beveled_rect, draw_text};
use element_colors::{ColorizationMode, get_category_color, get_state_color};
use element_data::{Category, State};

pub struct LegendWidget {
    rect: Cell<Rect>,
    colorization: Cell<ColorizationMode>,
}

impl LegendWidget {
    pub fn new() -> Arc<Self> {
        Arc::new(LegendWidget {
            rect: Cell::new(Rect::default()),
            colorization: Cell::new(ColorizationMode::ByCategories),
        })
    }

    fn draw_legend_item(&self, renderer: &mut Renderer, index: i32, text: &'static str, color: &Color) {
        let rect = self.rect.get();
        let x = rect.x + (index / 6) * 160;
        let y = rect.y + (index % 6) * 20;
        let font = Font::find(None, None, None).unwrap();

        draw_beveled_rect(renderer, x, y, 16, 16, color);
        draw_text(renderer, &font, 12.0, text, x + 18, y, 160, 16, &mult_color(color, 0.5), true, false);
    }
}

impl Place for LegendWidget {}

impl Widget for LegendWidget {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        // TODO: Is there a better way to do this?
        match self.colorization.get() {
            ColorizationMode::None => { },
            ColorizationMode::ByCategories => {
                self.draw_legend_item(renderer, 0, "Alkali metal", &get_category_color(Category::AlkaliMetal));
                self.draw_legend_item(renderer, 1, "Alkaline earth metal", &get_category_color(Category::AlkalineEarthMetal));
                self.draw_legend_item(renderer, 2, "Transition metal", &get_category_color(Category::TransitionMetal));
                self.draw_legend_item(renderer, 3, "Post-transition metal", &get_category_color(Category::PostTransitionMetal));
                self.draw_legend_item(renderer, 4, "Metalloid", &get_category_color(Category::Metalloid));
                self.draw_legend_item(renderer, 5, "Polyatomic nonmetal", &get_category_color(Category::PolyatomicNonmetal));
                self.draw_legend_item(renderer, 6, "Diatomic nonmetal", &get_category_color(Category::DiatomicNonmetal));
                self.draw_legend_item(renderer, 7, "Noble gas", &get_category_color(Category::NobleGas));
                self.draw_legend_item(renderer, 8, "Lanthanide", &get_category_color(Category::Lanthanide));
                self.draw_legend_item(renderer, 9, "Actinide", &get_category_color(Category::Actinide));
                self.draw_legend_item(renderer, 10, "Unknown", &get_category_color(Category::Unknown));
            },
            ColorizationMode::ByStates => {
                self.draw_legend_item(renderer, 2, "Solid", &get_state_color(State::Solid));
                self.draw_legend_item(renderer, 3, "Liquid", &get_state_color(State::Liquid));
                self.draw_legend_item(renderer, 4, "Gas", &get_state_color(State::Gas));
                self.draw_legend_item(renderer, 8, "Plasma", &get_state_color(State::Plasma));
                self.draw_legend_item(renderer, 9, "Unknown", &get_state_color(State::Unknown));
            },
            _ =>  { },
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        focused
    }
}
