use std::thread;

use natural_constants::chemistry::*;
use orbtk::traits::{Place, Click};
use orbtk::{Rect, Window};

use widgets::{AtomWidget, LegendWidget};

const ATOM_WIDTH: u32 = 36;
const ATOM_HEIGHT: u32 = 48;
const PADDING: u32 = 16;
const WINDOW_WIDTH: u32 = ATOM_WIDTH * 18 + PADDING * 2;
const WINDOW_HEIGHT: u32 = ATOM_HEIGHT * 10 + PADDING * 2;

static ATOMS: [&AtomInfo; 118] = [
    &atom_h,  &atom_he, &atom_li, &atom_be, &atom_b,  &atom_c,  &atom_n,  &atom_o,
    &atom_f,  &atom_ne, &atom_na, &atom_mg, &atom_al, &atom_si, &atom_p,  &atom_s,
    &atom_cl, &atom_ar, &atom_k,  &atom_ca, &atom_sc, &atom_ti, &atom_v,  &atom_cr,
    &atom_mn, &atom_fe, &atom_co, &atom_ni, &atom_cu, &atom_zn, &atom_ga, &atom_ge,
    &atom_as, &atom_se, &atom_br, &atom_kr, &atom_rb, &atom_sr, &atom_y,  &atom_zr,
    &atom_nb, &atom_mo, &atom_tc, &atom_ru, &atom_rh, &atom_pd, &atom_ag, &atom_cd,
    &atom_in, &atom_sn, &atom_sb, &atom_te, &atom_i,  &atom_xe, &atom_cs, &atom_ba,
    &atom_la, &atom_ce, &atom_pr, &atom_nd, &atom_pm, &atom_sm, &atom_eu, &atom_gd,
    &atom_tb, &atom_dy, &atom_ho, &atom_er, &atom_tm, &atom_yb, &atom_lu, &atom_hf,
    &atom_ta, &atom_w,  &atom_re, &atom_os, &atom_ir, &atom_pt, &atom_au, &atom_hg,
    &atom_tl, &atom_pb, &atom_bi, &atom_po, &atom_at, &atom_rn, &atom_fr, &atom_ra,
    &atom_ac, &atom_th, &atom_pa, &atom_u,  &atom_np, &atom_pu, &atom_am, &atom_cm,
    &atom_bk, &atom_cf, &atom_es, &atom_fm, &atom_md, &atom_no, &atom_lr, &atom_rf,
    &atom_db, &atom_sg, &atom_bh, &atom_hs, &atom_mt, &atom_ds, &atom_rg, &atom_cn,
    &atom_nh, &atom_fl, &atom_mc, &atom_lv, &atom_ts, &atom_og,
];

pub fn create_main_window() -> Window {
    let window = Window::new(Rect::new(10, 10, WINDOW_WIDTH, WINDOW_HEIGHT), "Periodic Table");

    for atom in ATOMS.iter() {
        let (x, y) = match atom.atomic_number {
            71 => (2, 5), // Lutetium
            103 => (2, 6), // Lawrencium
            _ => {
                match atom.sub_category {
                    SubCategory::Lanthanide => (2 + (atom.atomic_number - 57), 8),
                    SubCategory::Actinide => (2 + (atom.atomic_number - 89), 9),
                    _ => (atom.group - 1, atom.period as u32 - 1),
                }
            }
        };

        let widget = AtomWidget::new(&atom);
        widget.position((x * ATOM_WIDTH + PADDING) as i32, (y * ATOM_HEIGHT + PADDING) as i32)
            .size(ATOM_WIDTH, ATOM_HEIGHT)
            .on_click(move |sender: &AtomWidget, _| {
                let atom = sender.atom();
                //thread::spawn(move || {
                    let mut atom_window = ::windows::create_atom_window(&atom);
                    atom_window.exec();
                //});
            });
        window.add(&widget);
    }

    // Legend widget
    let legend = LegendWidget::new();
    // TODO: Calculate widget bounds properly
    legend.position((PADDING + 3 * ATOM_WIDTH) as i32, PADDING as i32 + (ATOM_HEIGHT / 4) as i32)
        .size(8 * ATOM_WIDTH, 2 * ATOM_HEIGHT);
    window.add(&legend);

    window
}


