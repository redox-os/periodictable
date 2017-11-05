use std::cell::Cell;
use std::sync::Arc;

use element_colors::{ColorizationMode, get_category_color, get_state_color};
use gfxutils::{mult_color, draw_beveled_rect, draw_text};
use natural_constants::chemistry::{SubCategory, StateOfMatter};
use orbclient::{Color, Renderer};
use orbtk::event::Event;
use orbtk::rect::Rect;
use orbtk::traits::Place;
use orbtk::widgets::Widget;

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

        draw_beveled_rect(renderer, x, y, 16, 16, color);
        draw_text(renderer, &super::FONT, 12.0, text, x + 18, y, 160, 16, &mult_color(color, 0.5), true, false);
    }
}

impl Place for LegendWidget {}

impl Widget for LegendWidget {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        match self.colorization.get() {
            ColorizationMode::None => { },
            ColorizationMode::ByCategories => {
                self.draw_legend_item(renderer, 0,  "Alkali metal",          &get_category_color(&SubCategory::AlkaliMetal        ));
                self.draw_legend_item(renderer, 1,  "Alkaline earth metal",  &get_category_color(&SubCategory::AlkalineEarthMetal ));
                self.draw_legend_item(renderer, 2,  "Transition metal",      &get_category_color(&SubCategory::TransitionMetal    ));
                self.draw_legend_item(renderer, 3,  "Post-transition metal", &get_category_color(&SubCategory::PostTransitionMetal));
                self.draw_legend_item(renderer, 4,  "Metalloid",             &get_category_color(&SubCategory::Metalloid          ));
                self.draw_legend_item(renderer, 5,  "Polyatomic nonmetal",   &get_category_color(&SubCategory::PolyatomicNonMetal ));
                self.draw_legend_item(renderer, 6,  "Diatomic nonmetal",     &get_category_color(&SubCategory::DiatomicNonMetal   ));
                self.draw_legend_item(renderer, 7,  "Noble gas",             &get_category_color(&SubCategory::NobleGas           ));
                self.draw_legend_item(renderer, 8,  "Lanthanide",            &get_category_color(&SubCategory::Lanthanide         ));
                self.draw_legend_item(renderer, 9,  "Actinide",              &get_category_color(&SubCategory::Actinide           ));
                self.draw_legend_item(renderer, 10, "Unknown",               &get_category_color(&SubCategory::Unknown            ));
            },
            ColorizationMode::ByStates => {
                self.draw_legend_item(renderer, 2, "Solid",   &get_state_color(&StateOfMatter::Solid  ));
                self.draw_legend_item(renderer, 3, "Liquid",  &get_state_color(&StateOfMatter::Liquid ));
                self.draw_legend_item(renderer, 4, "Gas",     &get_state_color(&StateOfMatter::Gas    ));
                self.draw_legend_item(renderer, 8, "Unknown", &get_state_color(&StateOfMatter::Unknown));
            },
        }
    }

    fn event(&self, _event: Event, focused: bool, _redraw: &mut bool) -> bool {
        focused
    }
}
