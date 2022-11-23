use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use eframe::epaint::Color32;
use eframe::Frame;
use egui::RichText;
use crate::checkoffs::checkoffs::TruckLevel;
use crate::checkoffs::{Checkoffs, TruckCheck};
use crate::ui::{CheckoffApp, ui_util, State};

pub(crate) fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    // Height of the title bar
    let height = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(
                rect.shrink(1.0),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            // Paint the title:
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height * 0.8),
                text_color,
            );

            // Paint the line under the title:
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );

            // Add the close button:
            let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {
                frame.close();
            }

            // Interact with the title bar (drag to move window):
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            // Add the contents:
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
                .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}

pub(crate) fn draw_truck_line(
    checks: &mut Checkoffs,
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


pub(crate) fn edit_window(
    ctx: &egui::Context,
    frame: &mut Frame,
    tcheck: &mut TruckCheck,
) {
    let title = stringify!("Editing {}", &tcheck);
    ui_util::custom_window_frame(ctx, frame, title, |ui|{
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