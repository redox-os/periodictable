use natural_constants::chemistry::AtomInfo;
use orbtk::traits::Place;
use orbtk::{Rect, Window};

use widgets::ElementWidget;

pub fn create_atom_window(atom: &'static AtomInfo) -> Window {
    let window = Window::new(Rect::new(-1, -1, 400, 300), atom.full_name);

    let widget = ElementWidget::new(atom);
    widget.position(16, 16)
        .size(192, 256);
    window.add(&widget);

    window
}
