mod element_widget;
mod legend_widget;

pub use self::element_widget::ElementWidget;
pub use self::legend_widget::LegendWidget;

use orbfont::Font;

lazy_static! {
    pub static ref FONT: Font = Font::find(None, None, None).expect("Failed to load FONT");
    pub static ref BOLD_FONT: Font = Font::find(None, None, Some("Bold")).expect("Failed to load BOLD_FONT");
}
