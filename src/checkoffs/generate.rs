use std::cell::Ref;
use std::fs::File;
use chrono::{Duration, NaiveDate};
use serde::Serialize;
use serde_json::value::Map;
use handlebars;
use crate::checkoffs::TruckCheck;
use crate::ui::CheckoffApp;

#[derive(Debug, Serialize)]
struct IndivCheck {
    name: String,
    date: String,
    level: String,
}

pub fn generate_checkoffs(app: &CheckoffApp) {
    let mut check_vec: Vec<IndivCheck> = vec![];

    for truck in app.checkoffs.checkoffs.iter() {
        let truck = truck.borrow();

        let diff = app.end_date.unwrap()
            .signed_duration_since(app.start_date.unwrap()).num_days();

        let mut truck_check_vec = gen_single_truck(truck, app.start_date.unwrap(), diff);

        println!("{:?}", truck_check_vec);
        check_vec.append(&mut truck_check_vec);
    }

    create_input_file(check_vec)
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

fn create_input_file(checks: Vec<IndivCheck>) {
    let mut data = Map::new();
    data.insert("checks".to_string(), handlebars::to_json(&checks));

    let mut handlebars = handlebars::Handlebars::new();

    handlebars.register_template_file("template", "templates/input.hbs").unwrap();

    let mut file = File::create("./templates/input.tex").unwrap();
    handlebars.render_to_write("template", &data, &mut file).unwrap();
}