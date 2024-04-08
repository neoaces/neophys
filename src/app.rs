use nannou::{draw::mesh::vertex::Color, prelude::*};
use nannou_egui::{
    self,
    egui::{self, Color32, Visuals},
    Egui,
};
use neophys::{
    body::{
        Body,
        BodyType::{self, *},
    },
    engine::{self, Engine},
};
use std::{borrow::Borrow, cell::RefCell, ops::Add, vec};

static TITLE: &str = "Root Viewport";
static V1_TITLE: &str = "Viewport 1";

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    resolution: u32,
    scale: f32,
    color: Srgb<u8>,
    time: f32,
}

struct Model {
    engine: Engine,
    settings: Settings,
    egui: Egui,
    x: f32,
    t: f32,
    m: f32,
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
            scale: 25.0,
            color: WHITE,
            time: 100.0,
        },
        x: 0.0,
        t: 0.0,
        m: 10.0,
        engine: Engine::new(vec![RefCell::new(Body::new(BodyType::GPoint, 10.0))]),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let frame = &egui.begin_frame();

    egui::Window::new("Settings").show(frame, |ui| {
        ui.label("Mass of body");
        ui.add(egui::Slider::new(&mut model.m, 0.1..=100.0).text("Mass"));
        ui.label(format!(
            "Mass (kg): {}",
            model.engine.bodies().get(0).unwrap().borrow().mass()
        ));
        ui.label(format!(
            "Gravity force on the object: {}",
            model.engine.body(0).unwrap().borrow_mut().sum_forces()
        ))
    });

    if model.engine.peek_bodies().first().unwrap().borrow_mut().m != model.m {
        model.engine.update_mass(model.m, 0).ok();
    };

    model.t = _app.elapsed_frames() as f32 / model.settings.time;

    model.engine.calc(model.t)
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let mut engine = &model.engine;
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(srgba(0.0, 0.0, 0.0, 1.0));
    println!("{:?}", engine);

    draw.ellipse()
        .resolution(settings.resolution as f32)
        .xy(vec2(model.x, 0.0))
        .color(settings.color)
        .radius(settings.scale);

    draw.line()
        .weight(1.0)
        .color(WHITE)
        .points(win.mid_left(), win.mid_right());

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
