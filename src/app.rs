use std::{ops::Add, vec};

use nannou::{draw::mesh::vertex::Color, prelude::*};
use nannou_egui::{
    self,
    egui::{self, Color32, Visuals},
    Egui,
};
use neophys::engine::{self, Engine};

static TITLE: &str = "Root Viewport";
static V1_TITLE: &str = "Viewport 1";

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    resolution: u32,
    scale: f32,
    color: Srgb<u8>,
    engine: Engine,
}

struct Model {
    settings: Settings,
    egui: Egui,
    x: f32,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        egui,
        settings: Settings {
            resolution: 10,
            scale: 50.0,
            color: WHITE,
            engine: Engine::default(),
        },
        x: 0.0,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let frame = &egui.begin_frame();

    egui::Window::new("Settings").show(frame, |ui| ui.label("Viewport"));
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;

    let draw = app.draw();
    draw.background().color(srgba(0.0, 0.0, 0.0, 1.0));

    if app.keys.down.contains(&Key::Right) {
        println!("Key down.");
        let mut x = model.x;
        x += 0.1;
    }

    println!("{}", model.x);

    draw.ellipse()
        .resolution(settings.resolution as f32)
        .xy(vec2(model.x, 0.0))
        .color(settings.color)
        .radius(settings.scale);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
