use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use super::ui_util;
use eframe::egui;
use egui::{Color32, RichText};
use crate::checkoffs::{Checkoffs, TruckCheck};
use crate::checkoffs::checkoffs::TruckLevel;
use crate::ui::ui_util::{draw_truck_line, edit_window};

#[derive(Debug)]
pub struct CheckoffApp{
    checkoffs: Checkoffs,
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
            checkoffs: Checkoffs::new(None),
            state: State::Normal,
            specify_date: false,
        }
    }
}
impl CheckoffApp{
    pub fn new(_ctx: &eframe::CreationContext<'_>, checkoffs: Option<Checkoffs>) -> Self {
        match checkoffs {
            Some(c) => CheckoffApp{
                checkoffs: c,
                ..Default::default()
            },
            None => CheckoffApp::default()
        }
    }
}

impl eframe::App for CheckoffApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {state, checkoffs:chks, specify_date } = self;

        ui_util::custom_window_frame(ctx, frame, "Checkoff Generator", |ui|{

            ui.heading("Truck Checkoffs:");
            ui.horizontal(|ui| {
                ui.label("asdf")
            });

            let mut checks_temp = chks.checkoffs.clone();
            let mut to_delete: Vec<bool> = Vec::new();

            for ch in checks_temp.iter() {
                match draw_truck_line(chks, ch.borrow_mut().deref_mut(), ui) {
                    true => {
                      to_delete.push(true)
                    },
                    false => {to_delete.push(false)}
                };
            }

            // Retain from the boolean mask from earlier, onto the temp, then finally update the
            // actual checkoffs with the temp ones.
            let mut iter = to_delete.into_iter();
            checks_temp.retain(|_| !iter.next().unwrap());
            chks.checkoffs = checks_temp;

            if ui.button("Add New").clicked() {
                chks.add(TruckCheck::default());
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