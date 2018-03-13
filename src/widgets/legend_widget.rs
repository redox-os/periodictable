use std::cell::{Cell, RefCell};
use std::sync::Arc;

use colors::{ColorizationMode, sub_category_color, state_of_matter_color};
use widgets::{draw_beveled_rect, draw_text, multiply_color};
use natural_constants::chemistry::{SubCategory, StateOfMatter};
use orbclient::{Color, Renderer};
use orbtk::event::Event;
use orbtk::point::Point;
use orbtk::rect::Rect;
use orbtk::theme::Theme;
use orbtk::thickness::Thickness;
use orbtk::traits::Place;
use orbtk::widgets::{HorizontalPlacement, VerticalPlacement, Widget};

pub struct LegendWidget {
    rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<Widget>>>,
    pub colorization: Cell<ColorizationMode>,
}

impl LegendWidget {
    pub fn new() -> Arc<Self> {
        Arc::new(LegendWidget {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![]),
            colorization: Cell::new(ColorizationMode::ByCategories),
        })
    }

    pub fn colorization(&self, colorization: ColorizationMode) -> &Self {
        self.colorization.set(colorization);
        self
    }
}

impl Place for LegendWidget {}

impl Widget for LegendWidget {
    fn name(&self) -> &str {
        "Legend"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn margin(&self) -> &Cell<Thickness> {
        &self.margin
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool, _theme: &Theme) {
        let rect = self.rect.get();

        let mut draw_item = |index: i32, text: &'static str, color: &Color| {
            let x = rect.x + (index / 6) * 160;
            let y = rect.y + (index % 6) * 20;
            draw_beveled_rect(renderer, x, y, 16, 16, color);
            draw_text(renderer, &super::FONT, 12.0, text, x + 18, y, 160, 16, &multiply_color(color, 0.5), true, false);
        };

        match self.colorization.get() {
            ColorizationMode::None => { },
            ColorizationMode::ByCategories => {
                draw_item(0,  "Alkali metal",          &sub_category_color(&SubCategory::AlkaliMetal        ));
                draw_item(1,  "Alkaline earth metal",  &sub_category_color(&SubCategory::AlkalineEarthMetal ));
                draw_item(2,  "Transition metal",      &sub_category_color(&SubCategory::TransitionMetal    ));
                draw_item(3,  "Post-transition metal", &sub_category_color(&SubCategory::PostTransitionMetal));
                draw_item(4,  "Metalloid",             &sub_category_color(&SubCategory::Metalloid          ));
                draw_item(5,  "Polyatomic nonmetal",   &sub_category_color(&SubCategory::PolyatomicNonMetal ));
                draw_item(6,  "Diatomic nonmetal",     &sub_category_color(&SubCategory::DiatomicNonMetal   ));
                draw_item(7,  "Noble gas",             &sub_category_color(&SubCategory::NobleGas           ));
                draw_item(8,  "Lanthanide",            &sub_category_color(&SubCategory::Lanthanide         ));
                draw_item(9,  "Actinide",              &sub_category_color(&SubCategory::Actinide           ));
                draw_item(10, "Unknown",               &sub_category_color(&SubCategory::Unknown            ));
            },
            ColorizationMode::ByStates => {
                draw_item(1, "Solid",   &state_of_matter_color(&StateOfMatter::Solid  ));
                draw_item(2, "Liquid",  &state_of_matter_color(&StateOfMatter::Liquid ));
                draw_item(3, "Gas",     &state_of_matter_color(&StateOfMatter::Gas    ));
                draw_item(4, "Unknown", &state_of_matter_color(&StateOfMatter::Unknown));
            },
        }
    }

    fn event(&self, _event: Event, focused: bool, _redraw: &mut bool) -> bool {
        focused
    }
}
