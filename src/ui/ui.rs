use super::ui_util;
use eframe::egui;

pub struct CheckoffApp{
    test_label: String
}

impl Default for CheckoffApp {
    fn default() -> Self {
        Self {
            test_label: "Testing 123".to_owned(),
        }
    }
}
impl CheckoffApp{
    pub fn new(_ctx: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for CheckoffApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba{
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {test_label: _} = self;

        ui_util::custom_window_frame(ctx, frame, "Checkoff Generator", |ui|{
            ui.button("Add New");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
    }
}