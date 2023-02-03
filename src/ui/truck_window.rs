use std::ops::DerefMut;
use chrono::{Date, Datelike, Local, NaiveDate};
use eframe::epaint::Color32;
use eframe::Frame;
use egui::{RichText, SidePanel};
use crate::checkoffs::checkoffs::TruckLevel;
use crate::checkoffs::{Checkoffs, TruckCheck};
use crate::ui::{CheckoffApp, State};
use crate::ui::ui::PrintMode;



pub(crate) fn home_window(
    ui: &mut egui::Ui,
    app: &mut CheckoffApp
) {
        ui.heading("Truck Checkoffs:");
        let mut chks = app.checkoffs.clone();

        let mut checks_temp = chks.checkoffs.clone();
        let mut to_delete: Vec<bool> = Vec::new();

        // Iterating over the temporary list causes a problem with deleting an entry as it is unsafe
        // in general to remove an item from a list you are iterating over. So we create a vec
        // of boolean values to_delete of the one(s) we mark for deletion. This also allows us
        // to delete multiple at once, though as of yet the UI doesn't allow for that.
        for ch in checks_temp.iter() {
            match draw_truck_line(ch.borrow_mut().deref_mut(), ui) {
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
        app.checkoffs.checkoffs = checks_temp;

        ui.horizontal(|ui| {
            if ui.button("Add New").clicked() {
                app.checkoffs.add(TruckCheck::default());
            };
            if ui.button("Edit Checkoffs").clicked() {
                app.state = State::Editing;
            }
        });
}

pub(crate) fn draw_truck_line(
    c: &mut TruckCheck,
    ui: &mut egui::Ui,
) -> bool {
    let mut deleted: bool = false;
    ui.push_id(format!("{:?}", c), |ui|{
        ui.horizontal(|ui|{
            ui.add(egui::TextEdit::singleline(&mut c.name));
            egui::ComboBox::from_id_source(&c.name)
                .selected_text(format!("{:?}", c.level))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut c.level, TruckLevel::ALS, "ALS");
                    ui.selectable_value(&mut c.level, TruckLevel::BLS, "BLS");
                    ui.selectable_value(&mut c.level, TruckLevel::Vent, "Vent");
                });
            ui.add(egui::Checkbox::new(&mut c.print, "Print"));
            if ui.button(RichText::new("Delete").color(Color32::RED)).clicked() {
                deleted = true
            };
        });
    });
    ui.end_row();
    deleted
}

