use std::borrow::Borrow;
use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;
use super::ui_util;
use eframe::egui;
use crate::checkoffs::Checkoffs;

#[derive(Debug)]
pub struct CheckoffApp{
    test_label: String,
    checkoffs: Rc<Checkoffs>,
}

impl Default for CheckoffApp {
    fn default() -> Self {
        Self {
            test_label: "Testing 123".to_owned(),
            checkoffs: Rc::new(Checkoffs::new(None))
        }
    }
}
impl CheckoffApp{
    pub fn new(_ctx: &eframe::CreationContext<'_>, checkoffs: Option<Checkoffs>) -> Self {
        match checkoffs {
            Some(c) => CheckoffApp{
                checkoffs: Rc::new(c),
                ..Default::default()
            },
            None => CheckoffApp::default()
        }
    }
}

impl eframe::App for CheckoffApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba{
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {test_label: _, checkoffs:chks } = self;

        ui_util::custom_window_frame(ctx, frame, "Checkoff Generator", |ui|{

            ui.heading("Truck Checkoffs:");
            ui.horizontal(|ui| {
                ui.label("asdf")
            });

            println!("{:?}", chks);
            for ch in chks.checkoffs.iter(){
                match &*ch.borrow() {
                    Some(c) => {
                        ui.label("truck");
                    },
                    None => {ui.label("No Truck Checks. Add one!");}
                }
            }
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