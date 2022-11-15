use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;
use super::ui_util;
use eframe::egui;
use egui::{Color32, RichText};
use crate::checkoffs::{Checkoffs, TruckCheck};
use crate::checkoffs::checkoffs::TruckLevel;
use crate::ui::ui_util::{draw_truck_line, edit_window};

#[derive(Debug)]
pub struct CheckoffApp{
    checkoffs: Rc<Checkoffs>,
    state: State,
    specify_date: bool,
}

#[derive(Debug, Default)]
pub enum State{
    #[default] Normal,
    Editing(TruckCheck)
}

impl Default for CheckoffApp {
    fn default() -> Self {
        Self {
            checkoffs: Rc::new(Checkoffs::new(None)),
            state: State::Normal,
            specify_date: false,
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {state, checkoffs:chks, specify_date } = self;

        println!("Start of update actual {:?}", state );

        ui_util::custom_window_frame(ctx, frame, "Checkoff Generator", |ui|{

            ui.heading("Truck Checkoffs:");
            ui.horizontal(|ui| {
                ui.label("asdf")
            });

            for ch in chks.checkoffs.iter(){
                match ch.borrow_mut().as_mut() {
                    Some(c) => {
                        draw_truck_line(c, ui);
                        println!("{:?}", c);
                    },
                    None => {}
                }
            }
            if ui.button("Add New").clicked() {
                todo!()
            };
            ui.add(egui::Separator::default());
        });

        match state {
            State::Normal => {},
            State::Editing(t) => {
                edit_window(ctx, frame, t);
            }
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba{
        egui::Rgba::TRANSPARENT
    }
}