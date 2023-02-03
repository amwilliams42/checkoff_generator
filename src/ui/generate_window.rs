use std::ops::Deref;
use chrono::{Date, Datelike, Local, NaiveDate};
use egui_extras::DatePickerButton;
use crate::checkoffs::generate_checkoffs;
use crate::ui::{CheckoffApp, State};
use crate::ui::ui::PrintMode;

fn date_handler(
    ui: &mut egui::Ui,
    app: &mut CheckoffApp
){
    let loc_time: Date<Local> = Local::today();
    let start_date = app.start_date.get_or_insert_with(||
        NaiveDate::from_ymd(
            loc_time.year(),
            loc_time.month(),
            1
        ));
    let end_date = app.end_date.get_or_insert_with(||
        NaiveDate::from_ymd(
            loc_time.year(),
            loc_time.month(),
            1
        ));
    ui.horizontal(|ui| {
        ui.label("Start Date:");
        ui.add(DatePickerButton::new(start_date).id_source("start date picker"));
    });
    ui.horizontal(|ui| {
        ui.label("End Date:");
        ui.add(DatePickerButton::new(end_date).id_source("end date picker"));
    });

    app.start_date = Some(*start_date);
    app.end_date = Some(*end_date);
}

pub fn generate_window(ui: &mut egui::Ui,app: &mut CheckoffApp) {
    ui.spacing_mut().item_spacing.x = 5.0;
    ui.heading("Checkoff Generation Settings");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("How should the trucks be split? ");
        ui.selectable_value(&mut app.print_mode, PrintMode::AllTogether, "All In One File");
        ui.selectable_value(&mut app.print_mode, PrintMode::OneByOne, "One Unit Per File");
    });


        date_handler(ui, app);
        let mut gen_vis:bool = false;
        match (app.start_date,app.end_date) {
            (Some(start), Some(end)) => {
                let diff = end.signed_duration_since(start);
                ui.label(format!("This will generate {} days of checkoffs per truck", diff.num_days() + 1));
                gen_vis = true;

            }
            _ => {ui.label(format!("Please Select Dates"));}
        }
        ui.add_enabled_ui(gen_vis, |ui| {
            if ui.button("Generate Checkoffs").clicked() {
                generate_checkoffs(app.deref())
            }
        });

}