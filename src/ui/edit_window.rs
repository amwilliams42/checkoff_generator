use std::borrow::BorrowMut;
use eframe::Frame;
use crate::checkoffs::CheckoffForm;
use crate::checkoffs::checkoffs::TruckLevel;
use crate::ui::{CheckoffApp, State, truck_window};
use itertools::Itertools;

pub(crate) fn edit_window(
    ui: &mut egui::Ui,
    app: &mut CheckoffApp,
) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    ui.heading("Cabinets");
                    if ui.button("Add New").clicked() {
                        app.form.add_empty_cabinet()
                    };
                });
                for cabinet in app.form.cabinets.iter() {
                    ui.horizontal(|ui| {
                        let mut cab = cabinet.borrow_mut();
                        ui.add(egui::TextEdit::singleline(&mut cab.number).desired_width(60 as f32));
                        ui.add(
                            egui::TextEdit::multiline(&mut cab.contents)
                        )
                    });
                }
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    ui.heading("Categories");
                    if ui.button("Add New").clicked() {
                        app.form.add_empty_category()
                    };
                });
                for (i, category) in app.form.categories.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let mut cat = category.borrow_mut();
                        ui.add(egui::TextEdit::singleline(&mut cat.name).desired_width(150 as f32));
                        ui.radio_value(&mut cat.level, TruckLevel::BLS, "BLS");
                        ui.radio_value(&mut cat.level, TruckLevel::ALS, "ALS");
                        ui.radio_value(&mut cat.level, TruckLevel::Vent, "VENT");
                    });

                    let cat = category.clone();
                    let items = cat.into_inner().items;
                    let (items, add_row, del_row) = category_table(ui,items, i);
                    category.borrow_mut().items = items;
                    if add_row {category.borrow_mut().add_item_row()}
                    if del_row {category.borrow_mut().delete_empties()}
                }
            });
        });
        ui.horizontal(|ui| {
            if ui.button("Save and Return").clicked() {
                app.state = State::Normal
            }
        });
}

fn category_table(
    ui: &mut egui::Ui,
    items: Vec<String>,
    table_id: usize
) -> (Vec<String>, bool, bool) {
    //Displays the table and returns the table with changes if any
    let mut temp_items = items.clone();
    let mut add_row: bool = false;
    let mut del_row: bool = false;
    ui.push_id(format!("{}_table_values", table_id), |ui| {
        use egui_extras::{TableBuilder, Column};
        TableBuilder::new(ui)
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .body(|mut body| {
                for (a, b) in temp_items.iter_mut().tuples() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.add(egui::TextEdit::singleline(a).desired_width(150 as f32));
                        });
                        row.col(|ui| {
                            ui.add(egui::TextEdit::singleline(b).desired_width(150 as f32));
                        });
                    });
                }
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        if ui.button("Add Row").clicked() {
                            add_row = true
                        };
                    });
                    row.col(|ui| {
                        if ui.button("Delete Empties").clicked() {
                            del_row = true
                        };
                    });
                })
            });
    });
    (temp_items, add_row, del_row)
}
