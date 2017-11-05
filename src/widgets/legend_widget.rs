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
}

impl Place for LegendWidget {}

impl Widget for LegendWidget {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        let mut draw_item = |index: i32, text: &'static str, color: &Color| {
            let x = rect.x + (index / 6) * 160;
            let y = rect.y + (index % 6) * 20;
            draw_beveled_rect(renderer, x, y, 16, 16, color);
            draw_text(renderer, &super::FONT, 12.0, text, x + 18, y, 160, 16, &mult_color(color, 0.5), true, false);
        };

        match self.colorization.get() {
            ColorizationMode::None => { },
            ColorizationMode::ByCategories => {
                draw_item(0,  "Alkali metal",          &get_category_color(&SubCategory::AlkaliMetal        ));
                draw_item(1,  "Alkaline earth metal",  &get_category_color(&SubCategory::AlkalineEarthMetal ));
                draw_item(2,  "Transition metal",      &get_category_color(&SubCategory::TransitionMetal    ));
                draw_item(3,  "Post-transition metal", &get_category_color(&SubCategory::PostTransitionMetal));
                draw_item(4,  "Metalloid",             &get_category_color(&SubCategory::Metalloid          ));
                draw_item(5,  "Polyatomic nonmetal",   &get_category_color(&SubCategory::PolyatomicNonMetal ));
                draw_item(6,  "Diatomic nonmetal",     &get_category_color(&SubCategory::DiatomicNonMetal   ));
                draw_item(7,  "Noble gas",             &get_category_color(&SubCategory::NobleGas           ));
                draw_item(8,  "Lanthanide",            &get_category_color(&SubCategory::Lanthanide         ));
                draw_item(9,  "Actinide",              &get_category_color(&SubCategory::Actinide           ));
                draw_item(10, "Unknown",               &get_category_color(&SubCategory::Unknown            ));
            },
            ColorizationMode::ByStates => {
                draw_item(2, "Solid",   &get_state_color(&StateOfMatter::Solid  ));
                draw_item(3, "Liquid",  &get_state_color(&StateOfMatter::Liquid ));
                draw_item(4, "Gas",     &get_state_color(&StateOfMatter::Gas    ));
                draw_item(8, "Unknown", &get_state_color(&StateOfMatter::Unknown));
            },
        }
    }

    fn event(&self, _event: Event, focused: bool, _redraw: &mut bool) -> bool {
        focused
    }
}
