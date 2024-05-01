use nannou_egui::egui;
use std::any;

use crate::constants::PPM;

pub fn type_print<T>(a: &T) {
    println!("{:?}", any::type_name_of_val(a));
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Inter".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/Inter-Regular.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "Inter".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter".to_owned());

    ctx.set_fonts(fonts)
}

pub fn scale(a: f32) -> f32 {
    a * PPM
}

pub fn descale(a: f32) -> f32 {
    a / PPM
}
