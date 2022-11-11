
mod ui;
mod checkoffs;

pub fn main() {
    let opt = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(320.0, 100.0)),

        ..Default::default()
    };
    let mut truck_checks = checkoffs::Checkoffs::new(None);

    truck_checks.add(checkoffs::TruckCheck::new("M-01".to_owned(), "BLS".to_owned()));
    truck_checks.add(checkoffs::TruckCheck::new("M-02".to_owned(), "BLS".to_owned()));
    eframe::run_native(
        "Checkoff Generator",
        opt,
        Box::new(
            |ctx| Box::new(ui::ui::CheckoffApp::new(ctx, Some(truck_checks)))
        )
    );
}
