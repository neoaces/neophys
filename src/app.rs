use log::{debug, error, info};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self},
    Egui,
};
use neophys::prelude::*;
use std::cell::RefCell;

static GROUND: f32 = 20.0;
static TIMESTEP: f32 = 0.1;
static CIRCLE_RES: u32 = 8;

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    resolution: u32,
    scale: f32,
    color: Srgb<u8>,
    time: f32,
    bounding_box: bool,
}

struct Model {
    engine: Engine,
    settings: Settings,
    egui: Egui,
    pos: Vec2,
    ground_y: f32,
    t: f32,
    m: f32,
}

fn model(app: &App) -> Model {
    env_logger::init();

    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    neophys::utils::setup_custom_fonts(egui.ctx());

    info!("Bottom y-coor: {}", window.rect().bottom());

    Model {
        egui,
        settings: Settings {
            resolution: CIRCLE_RES,
            scale: 25.0,
            color: RED,
            time: 100.0,
            bounding_box: true,
        },
        pos: Vec2::default(),
        t: 0.0,
        m: 10.0,
        engine: Engine::new(vec![RefCell::new(Body::new(BodyType::GPoint, 10.0))]),
        ground_y: window.rect().bottom() + GROUND,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let frame = &egui.begin_frame();

    let bodies = model.engine.bodies().first().unwrap().borrow();

    egui::Window::new("Settings").show(frame, |ui| {
        ui.label("Mass of body");
        ui.add(egui::Slider::new(&mut model.m, 0.1..=100.0).text("Mass"));
        ui.add(egui::Slider::new(&mut model.settings.scale, 5.0..=100.0).text("Radius"));
        ui.checkbox(&mut model.settings.bounding_box, "Enable bounding boxes");
        ui.label(format!("Mass (kg): {}", bodies.m));
        ui.label(format!(
            "Gravity force on the object: {}",
            bodies.sum_forces()
        ))
    });

    if model.engine.bodies().first().unwrap().borrow().m != model.m {
        model.engine.update_mass(model.m, 0).ok();
    };

    model.t = _app.elapsed_frames() as f32 / model.settings.time;

    if let Err(a) = model.engine.calc(model.t, TIMESTEP) {
        error!(
            "No bodies found in the engine. Please add a body using the engine::add_body function."
        )
    };
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let engine = &model.engine;
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(srgba(0.0, 0.0, 0.0, 1.0));
    debug!("{:?}", engine);
    for body in model.engine.bodies().iter() {
        let pos = body.borrow().s;
        let x = pos.x;
        let y = pos.y.clamp(model.ground_y + model.settings.scale, 0.0);
        let new_pos = Vec2::new(x, y);

        draw.ellipse()
            .resolution(settings.resolution as f32)
            // TODO: Change this when
            .xy(new_pos)
            .color(settings.color)
            .radius(settings.scale);

        if model.settings.bounding_box {
            neophys::debug::body::draw_bounding_circle(&draw, model.settings.scale, new_pos);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
