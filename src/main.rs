mod ui;
mod checkoffs;

pub fn main() {
    let opt = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(320.0, 100.0)),

        ..Default::default()
    };

    eframe::run_native(
        "Checkoff Generator",
        opt,
        Box::new(
            |ctx| Box::new(ui::ui::CheckoffApp::new(ctx))
        )
    );
}
