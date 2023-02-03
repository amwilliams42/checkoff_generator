use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
use std::io::Error;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use super::truck_window;
use eframe::{egui, Storage};
use egui::{Color32, RichText};
use crate::checkoffs::{CheckoffForm, Checkoffs, TruckCheck};
use crate::checkoffs::checkoffs::TruckLevel;
use crate::ui::edit_window::edit_window;
use crate::ui::generate_window::generate_window;
use crate::ui::truck_window::{draw_truck_line, home_window};

#[derive(Debug)]
pub struct CheckoffApp{
    pub(crate) checkoffs: Checkoffs,
    pub state: State,
    pub form: CheckoffForm,
    pub print_mode: PrintMode,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>
}

#[derive(Debug, Default, PartialEq)]
pub enum State{
    #[default] Normal,
    Editing,
    Generating,
}

#[derive(Debug, Default, PartialEq)]
pub enum PrintMode{
    #[default] OneByOne,
    AllTogether,
}

impl Default for CheckoffApp {
    fn default() -> Self {
        Self {
            checkoffs: Checkoffs::new(None),
            state: State::Normal,
            form: CheckoffForm::new(),
            print_mode: Default::default(),
            start_date: None,
            end_date: None
        }
    }
}
impl CheckoffApp{
    pub fn new(_ctx: &eframe::CreationContext<'_>) -> Self {
        let form = match CheckoffForm::load() {
            Ok(f) => f,
            Err(_) => {CheckoffForm::new()}
        };
        let checks = match Checkoffs::load() {
            Ok(c) => c,
            Err(_) => Checkoffs::new(None)
        };
        CheckoffApp{
            checkoffs: checks,
            form,
            ..Default::default()
        }
    }
}

impl eframe::App for CheckoffApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        use egui::*;


        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let (mut trucks, mut edit, mut generate) = (true, true, true);

                    match self.state {
                        State::Normal => {
                            trucks = false;
                        }
                        State::Editing => {
                            edit = false;
                        }
                        State::Generating => {
                            generate = false;
                        }
                    }
                    ui.add_enabled_ui(trucks, |ui| {
                        if ui.button("Trucks").clicked() {
                            self.state = State::Normal
                        }
                    });
                    ui.add_enabled_ui(edit, |ui| {
                        if ui.button("Edit Forms").clicked() {
                            self.state = State::Editing
                        }
                    });
                    ui.add_enabled_ui(generate, |ui| {
                        if ui.button("Generate Forms").clicked() {
                            self.state = State::Generating
                        }
                    });
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match self.state {
                State::Normal => {
                    home_window(ui, self);
                },
                State::Editing => {
                    edit_window(ui, self);
                },
                State::Generating => {
                    generate_window(ui, self);
                }
            }
        });

    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        self.checkoffs.save().unwrap();
        self.form.save().unwrap();
    }
}