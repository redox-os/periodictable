extern crate orbclient;

use orbclient::Color;
use element_data::{Category, State, Element};

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

pub fn get_category_color(category: Category) -> Color {
    match category {
        // Color::hsv(18 + i * 36, 128, 230)
        Category::AlkaliMetal         => Color::rgb(230, 149, 114),
        Category::AlkalineEarthMetal  => Color::rgb(230, 219, 114),
        Category::TransitionMetal     => Color::rgb(172, 230, 114),
        Category::PostTransitionMetal => Color::rgb(114, 230, 126),
        Category::Metalloid           => Color::rgb(114, 230, 196),
        Category::PolyatomicNonmetal  => Color::rgb(114, 196, 230),
        Category::DiatomicNonmetal    => Color::rgb(114, 126, 230),
        Category::NobleGas            => Color::rgb(172, 114, 230),
        Category::Lanthanide          => Color::rgb(230, 114, 219),
        Category::Actinide            => Color::rgb(230, 114, 153),
        Category::Unknown             => INACTIVE_COLOR,
    }
}

pub fn get_state_color(state: State) -> Color {
    match state {
        State::Solid   => Color::rgb(230, 149, 114),
        State::Gas     => Color::rgb(172, 230, 114),
        State::Liquid  => Color::rgb(114, 230, 196),
        State::Plasma  => Color::rgb(114, 126, 230),
        State::Unknown => INACTIVE_COLOR,
    }
}

#[derive(Copy, Clone)]
pub enum ColorizationMode {
    None,
    ByCategories,
    ByCategory(Category),
    ByStates,
    ByState(State),
    ByWeightThreshold(Threshold<f32>),
    ByAtomicNumberThreshold(Threshold<i32>),
}

fn bool_to_color(value: bool) -> Color {
    if value {
        ACTIVE_COLOR
    } else {
        INACTIVE_COLOR
    }
}

pub fn get_element_color(element: &Element, cm: ColorizationMode) -> Color {
    match cm {
        ColorizationMode::None => {
            bool_to_color(false)
        },
        ColorizationMode::ByCategories => {
            get_category_color(element.category)
        },
        ColorizationMode::ByCategory(c) => {
            bool_to_color(element.category == c)
        },
        ColorizationMode::ByStates => {
            get_state_color(element.state)
        },
        ColorizationMode::ByState(s) => {
            bool_to_color(element.state == s)
        },
        ColorizationMode::ByWeightThreshold(threshold) => {
            bool_to_color(eval_threshold_option(threshold, element.weight))
        },
        ColorizationMode::ByAtomicNumberThreshold(threshold) => {
            bool_to_color(eval_threshold(threshold, element.atomic_number))
        },
    }
}
