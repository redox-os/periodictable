use orbclient::Color;
use natural_constants::chemistry::{SubCategory, StateOfMatter, AtomInfo};

const ACTIVE_COLOR: Color = Color::rgb(230, 114, 114);
const INACTIVE_COLOR: Color = Color::rgb(165, 165, 165);

#[derive(Copy, Clone)]
pub enum Threshold<T: PartialOrd> {
    LessOrEqual(T),
    GreaterOrEqual(T),
    Between(T, T),
    Equals(T),
}

fn eval_threshold<T: PartialOrd>(threshold: Threshold<T>, value: T) -> bool {
    match threshold {
        Threshold::LessOrEqual(v) => { value <= v },
        Threshold::GreaterOrEqual(v) => { value >= v },
        Threshold::Between(v1, v2) => { value >= v1 && value <= v2 },
        Threshold::Equals(v) => { value == v },
    }
}

fn eval_threshold_option<T: PartialOrd>(threshold: Threshold<T>, value: Option<T>) -> bool {
    if let Some(v) = value {
        eval_threshold(threshold, v)
    } else {
        false
    }
}

pub fn get_category_color(category: &SubCategory) -> Color {
    match *category {
        // Color::hsv(18 + i * 36, 128, 230)
        SubCategory::AlkaliMetal         => Color::rgb(230, 149, 114),
        SubCategory::AlkalineEarthMetal  => Color::rgb(230, 219, 114),
        SubCategory::TransitionMetal     => Color::rgb(172, 230, 114),
        SubCategory::PostTransitionMetal => Color::rgb(114, 230, 126),
        SubCategory::Metalloid           => Color::rgb(114, 230, 196),
        SubCategory::PolyatomicNonMetal  => Color::rgb(114, 196, 230),
        SubCategory::DiatomicNonMetal    => Color::rgb(114, 126, 230),
        SubCategory::NobleGas            => Color::rgb(172, 114, 230),
        SubCategory::Lanthanide          => Color::rgb(230, 114, 219),
        SubCategory::Actinide            => Color::rgb(230, 114, 153),
        SubCategory::Unknown             => INACTIVE_COLOR,
    }
}

pub fn get_state_color(state: &StateOfMatter) -> Color {
    match *state {
        StateOfMatter::Solid   => Color::rgb(230, 149, 114),
        StateOfMatter::Gas     => Color::rgb(172, 230, 114),
        StateOfMatter::Liquid  => Color::rgb(114, 230, 196),
        //StateOfMatter::Plasma  => Color::rgb(114, 126, 230),
        StateOfMatter::Unknown => INACTIVE_COLOR,
    }
}

#[derive(Copy, Clone)]
pub enum ColorizationMode {
    None,
    ByCategories,
    ByStates,
}

fn bool_to_color(value: bool) -> Color {
    if value {
        ACTIVE_COLOR
    } else {
        INACTIVE_COLOR
    }
}

pub fn get_element_color(element: &AtomInfo, cm: ColorizationMode) -> Color {
    match cm {
        ColorizationMode::None => {
            bool_to_color(false)
        },
        ColorizationMode::ByCategories => {
            get_category_color(&element.sub_category)
        },
        ColorizationMode::ByStates => {
            get_state_color(&element.state_of_matter)
        },
    }
}
