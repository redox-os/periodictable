use orbclient::Color;
use natural_constants::chemistry::{SubCategory, StateOfMatter, AtomInfo};

const ACTIVE_COLOR: Color = Color::rgb(230, 114, 114);
const INACTIVE_COLOR: Color = Color::rgb(165, 165, 165);

pub fn get_category_color(sub_category: &SubCategory) -> Color {
    match *sub_category {
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

pub fn get_state_color(state_of_matter: &StateOfMatter) -> Color {
    match *state_of_matter {
        StateOfMatter::Solid   => Color::rgb(230, 149, 114),
        StateOfMatter::Gas     => Color::rgb(172, 230, 114),
        StateOfMatter::Liquid  => Color::rgb(114, 230, 196),
        StateOfMatter::Unknown => INACTIVE_COLOR,
    }
}

#[derive(Copy, Clone)]
pub enum ColorizationMode {
    None,
    ByCategories,
    ByStates,
}

pub fn get_element_color(element: &AtomInfo, cm: ColorizationMode) -> Color {
    match cm {
        ColorizationMode::None         => INACTIVE_COLOR,
        ColorizationMode::ByCategories => get_category_color(&element.sub_category),
        ColorizationMode::ByStates     => get_state_color(&element.state_of_matter),
    }
}
