#![feature(const_fn)]

extern crate natural_constants;
extern crate orbclient;
extern crate orbfont;
extern crate orbtk;

use std::thread;

use natural_constants::chemistry::*;
use orbtk::traits::{Place, Click};
use orbtk::{Window, Rect, Point};
use widgets::{ElementWidget, LegendWidget};

mod element_colors;
mod widgets;
mod gfxutils;

const ELEMENT_WIDTH: u32 = 36; //52;
const ELEMENT_HEIGHT: u32 = 48; //64;
const PADDING: u32 = 16;
const WINDOW_WIDTH: u32 = ELEMENT_WIDTH * 18 + PADDING * 2;
const WINDOW_HEIGHT: u32 = ELEMENT_HEIGHT * 10 + PADDING * 2;

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

fn main() {
    let mut window = Window::new(Rect::new(10, 10, WINDOW_WIDTH, WINDOW_HEIGHT), "Periodic Table of the Elements");

    // Element widgets
    for e in ATOMS.iter() {
        let x = (match e.sub_category {
            SubCategory::Lanthanide => 2 + (e.atomic_number - 57),
            SubCategory::Actinide => 2 + (e.atomic_number - 89),
            _ => e.group - 1,
        } * ELEMENT_WIDTH + PADDING) as i32;

        let y = (match e.sub_category {
            SubCategory::Lanthanide => 8,
            SubCategory::Actinide => 9,
            _ => e.period as u32 - 1,
        } * ELEMENT_HEIGHT + PADDING) as i32;

        let widget = ElementWidget::new(&e);
        widget.position(x, y)
            .size(ELEMENT_WIDTH, ELEMENT_HEIGHT)
            .on_click(move |_widget: &ElementWidget, _point: Point| {
                let element = _widget.element();
                thread::spawn(move || {
                    let mut window = Window::new(Rect::new(-1, -1, 400, 300), element.name);

                    let widget = ElementWidget::new(element);
                    widget.position(PADDING as i32, PADDING as i32)
                        .size(ELEMENT_WIDTH * 4, ELEMENT_HEIGHT * 4);
                    window.add(&widget);

                    window.exec();
                });
            });
        window.add(&widget);
    }

    // Legend widget
    let legend = LegendWidget::new();
    // TODO: Calculate widget bounds properly
    legend.position((PADDING + 3 * ELEMENT_WIDTH) as i32, PADDING as i32 + (ELEMENT_HEIGHT / 4) as i32)
        .size(8 * ELEMENT_WIDTH, 2 * ELEMENT_HEIGHT);
    window.add(&legend);

    window.exec();
}
