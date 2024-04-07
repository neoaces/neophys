use eframe::egui;

static TITLE: &str = "Root Viewport";
static V1_TITLE: &str = "Viewport 1";

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native("neophys", options, Box::new(|cc| Box::new(MyApp::new(cc))))
}

struct MyApp {
    show_immediate_viewport: bool,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            show_immediate_viewport: true,
        }
    }
}

impl eframe::App for MyApp {
    // 1. fn update() to update the screen
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(TITLE);

            ui.checkbox(&mut self.show_immediate_viewport, "Show viewport_1")
        });

        if self.show_immediate_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("viewport_1"),
                egui::ViewportBuilder::default()
                    .with_title(V1_TITLE)
                    .with_inner_size([300.0, 300.0]),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from immediate viewport");
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_immediate_viewport = false;
                    }
                },
            )
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
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
