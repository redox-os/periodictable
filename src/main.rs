extern crate natural_constants;
extern crate orbclient;
extern crate orbfont;
extern crate orbtk;
#[macro_use]
extern crate lazy_static;

mod colors;
mod threshold;
mod widgets;
mod windows;

fn main() {
    let mut main_window = windows::create_main_window();
    main_window.exec();
}
