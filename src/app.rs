use log::{debug, error, info};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self},
    Egui,
};
use neophys::{constants::PPM, prelude::*, utils::scale};

static GROUND: f32 = 20.0;
static CIRCLE_RES: u32 = 8;
static CIRCLE_RAD: f32 = 1.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    resolution: u32,
    scale: f32,
    color: Srgb<u8>,
    bounding_box: bool,
}

#[allow(dead_code)]
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
            scale: scale(CIRCLE_RAD), // In pixels
            color: RED,
            bounding_box: true,
        },
        pos: Vec2::default(),
        t: 0.0,
        m: 10.0,
        engine: Engine::new(
            vec![Body::new(
                BodyType::Ball(scale(CIRCLE_RAD)),
                10.0,
                scale(CIRCLE_RAD),
            )],
            window.rect(),
        ), // Careful of hardcoded radius value. Should be with settings.scale
        ground_y: window.rect().bottom() + GROUND,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let frame = &egui.begin_frame();
    let body = &model.engine.bodies[0];

    egui::Window::new("Settings").show(frame, |ui| {
        ui.label("Mass of body");
        ui.add(egui::Slider::new(&mut model.m, 0.1..=100.0).text("Mass"));
        ui.add(egui::Slider::new(&mut model.settings.scale, 5.0..=100.0).text("Radius"));
        ui.checkbox(&mut model.settings.bounding_box, "Enable bounding boxes");
        ui.label(format!("Mass (kg): {}", body.m));
        ui.label(format!(
            "Dimensions of window (m): {}, {}",
            app.window_rect().w() / PPM,
            app.window_rect().h() / PPM,
        ));
    });

    if model.engine.bodies[0].m != model.m {
        model.engine.update_mass(model.m, 0).ok();
    };

    info!(
        "Current time: {}s \nTime since start: {}s",
        model.t, app.time
    );
    let old_t = model.t;
    model.t = app.time;

    info!("Timestep {}s", model.t - old_t);

    if model.engine.calc(model.t - old_t).is_err() {
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
        let pos = body.s;
        let x = pos.x;
        let y = pos.y.clamp(win.bottom(), 0.0);
        let new_pos = Vec2::new(scale(x), scale(y));

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
