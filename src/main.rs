mod ui;
mod checkoffs;
use tectonic;

pub fn main() {
    let opt = eframe::NativeOptions {
        decorated: false,
        transparent: true,

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
