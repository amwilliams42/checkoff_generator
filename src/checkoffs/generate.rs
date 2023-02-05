use std::cell::Ref;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::fs::File;
use std::io::Write;
use chrono::{Duration, NaiveDate};
use serde::Serialize;
use serde_json::value::Map;
use handlebars;
use tectonic;
use tectonic::config::PersistentConfig;
use tectonic::driver::{OutputFormat, ProcessingSessionBuilder};
use tectonic::ErrorKind;
use tectonic::status::{ChatterLevel, StatusBackend};
use tectonic::status::plain::PlainStatusBackend;
use tectonic_bridge_core::{SecuritySettings, SecurityStance};
use rust_embed::RustEmbed;
use crate::checkoffs::TruckCheck;
use crate::ui::CheckoffApp;
use crate::ui::ui::PrintMode;


#[derive(RustEmbed)]
#[folder = "templates"]
struct Asset;

#[derive(Debug, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct IndivCheck {
    name: String,
    date: String,
    level: String,
}

pub struct RenderQueue {
    pub(crate) forms: Rcol,
    pub(crate) ctr: bool,
}

pub type Rcol = BTreeSet<(String, Vec<IndivCheck>)>;

pub fn generate_checkoffs(app: &mut CheckoffApp) {
    let diff = app.end_date.unwrap()
        .signed_duration_since(app.start_date.unwrap()).num_days();

    let mut reports_collection: Rcol = BTreeSet::new();

    match app.print_mode {
        PrintMode::OneByOne => {
            for truck in app.checkoffs.checkoffs.iter() {
                let truck = truck.borrow();
                let name = truck.name.clone();
                let truck_check_vec = gen_single_truck(truck, app.start_date.unwrap(), diff);
                reports_collection.insert((make_safe_string(name), truck_check_vec));
            }
        }
        PrintMode::AllTogether => {
            let mut all_checks: Vec<IndivCheck> = vec![];
            for truck in app.checkoffs.checkoffs.iter() {
                let truck = truck.borrow();
                all_checks.append(&mut gen_single_truck(truck, app.start_date.unwrap(), diff));
            }
            reports_collection.insert(("all".to_string(), all_checks));
        }
    }
    app.rqueue.forms = reports_collection;
    app.prog_w.total = app.rqueue.forms.len() as i32;
    app.rqueue.ctr = true;

}
fn make_safe_string(s: String) -> String {
    //takes a truck name and turns it into a string that can safely be used for filenames
    s.replace(|c: char| !c.is_alphanumeric(), "")
}
fn gen_single_truck(truck: Ref<TruckCheck>, start_date: NaiveDate, diff: i64 ) -> Vec<IndivCheck>{
    let mut check_vec: Vec<IndivCheck> = vec![];

    let mut date = start_date;
    for _ in 0..diff+1 {
        check_vec.push(
            IndivCheck{
                name: truck.name.clone(),
                date: date.format("%m.%d.%Y").to_string(),
                level: truck.level.to_string()
            }
        );
        date = date + Duration::days(1);
    }
    check_vec
}

pub fn create_latex(checks: Vec<IndivCheck>) -> String {
    let mut data = Map::new();
    data.insert("checks".to_string(), handlebars::to_json(&checks));

    let mut handlebars = handlebars::Handlebars::new();
    let template: Vec<u8> = Asset::get("truck_check.hbs").unwrap().data.into();
    let template = std::string::String::from_utf8(template).unwrap();
    handlebars.register_template_string("template", template).unwrap();
    handlebars.render("template", &data).unwrap()
}

pub fn create_pdf(name: String, checks:Vec<IndivCheck>) {

    let latex = create_latex(checks);
    let mut tec_session =
        ProcessingSessionBuilder::new_with_security(SecuritySettings::new(SecurityStance::MaybeAllowInsecures));
    let mut status = Box::new(PlainStatusBackend::new(ChatterLevel::Minimal)) as Box<dyn StatusBackend>;

    let config = PersistentConfig::open(true).unwrap();

    tec_session
        .format_name("latex")
        .keep_logs(false)
        .synctex(false)
        .print_stdout(false)
        .output_dir("./output")
        .output_format(OutputFormat::Pdf)
        .primary_input_buffer(latex.as_ref())
        .bundle(config.default_bundle(false, &mut *status).unwrap())
        .tex_input_name(format!("{}_checkoff.tex", name).as_str());

    fs::create_dir_all("./output").unwrap();

    let mut sess = tec_session.create(&mut *status).unwrap();
    let result = sess.run(&mut *status);

    if let Err(e) = &result {
        if let ErrorKind::EngineError(engine) = e.kind() {
            let output = sess.get_stdout_content();

            if output.is_empty() {
                tectonic::tt_error!(
                    status,
                    "something bad happened inside {}, but no output was logged",
                    engine
                );
            } else {
                tectonic::tt_error!(
                    status,
                    "something bad happened inside {}; its output follows:\n",
                    engine
                );
                status.dump_error_logs(&output);
            }
        }
    }

}