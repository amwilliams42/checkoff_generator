use egui::ProgressBar;

#[derive(Debug)]
pub struct ProgressWindow {
    pub(crate) enabled: bool,
    pub(crate) current: i32,
    pub(crate) total: i32,
    pub(crate) in_progress: bool,
    pub(crate) initialized: bool,
}

impl ProgressWindow{
    pub fn name(&self) -> &'static str {
        "Generating Checkoffs"
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Window::new(self.name())
            .fixed_pos(&[50.,50.])
            .resizable(false)
            .title_bar(false)
            .fixed_size(&[300.,50.])
            .show(ui.ctx(), |ui|
                self.ui(ui));
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let prog_string = format!("Generating Checkoff {} of {}...", self.current, self.total);
        let prog_perc:f32 = (self.current as f32/ self.total as f32) as f32;
        let mut but: bool = false;

            ui.vertical_centered(|ui|{
                match self.in_progress {
                    true => {
                        ui.label(prog_string);
                    }
                    false => {
                        ui.label("Completed!");
                        but = true;
                    }
                }
                ui.add(ProgressBar::new(prog_perc).animate(true).show_percentage());
                ui.add_enabled_ui(but, |ui|{
                    if ui.button("Ok").clicked() {
                        self.enabled = false;
                    }
                });
            });
    }


    pub fn inc_counter(&mut self) {
        if self.current < self.total {
            self.current += 1;
        }
    }
}