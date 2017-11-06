use natural_constants::chemistry::AtomInfo;
use orbtk::traits::Place;
use orbtk::{Rect, Text, TextBox, Window};

use widgets::AtomWidget;
use colors::ColorizationMode;

pub fn create_atom_window(atom: &'static AtomInfo, colorization: ColorizationMode) -> Window {
    let window = Window::new(Rect::new(-1, -1, 532, 288), atom.full_name);

    {
        let widget = AtomWidget::new(atom);
        widget.position(16, 16)
            .size(192, 256)
            .colorization(colorization);
        window.add(&widget);
    }

    {
        let description = format!("\
            | Atomic number: {}\n\
            |         Group: {}\n\
            |        Period: {}\n\
            |      Neutrons: {}\n\
            \n\
            |      Mass: {}\n\
            |   Density: {} g/cm3\n\
            | Abundance: {} mg/kg\n\
            \n\
            |     Melting point: {} K\n\
            |     Boiling point: {} K\n\
            | Electronegativity: {}\n\
            |     Heat capacity: {} J/(g K)\n\
            ",
            atom.atomic_number,
            atom.group,
            atom.period,
            atom.number_of_neutrons,
            atom.mass,
            atom.density,
            atom.abundance,
            atom.melt.unwrap_or(0.0),
            atom.boil.unwrap_or(0.0),
            atom.electronegativity.unwrap_or(0.0),
            atom.heat_capacity.unwrap_or(0.0),

        );

        let widget = TextBox::new();
        widget.position(192 + 16 + 16, 16)
            .size(292, 256)
            .text(description);
        window.add(&widget);
    }

    window
}
