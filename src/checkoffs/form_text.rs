use std::cell::RefCell;
use egui::TextBuffer;
use crate::checkoffs::checkoffs::TruckLevel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CheckoffForm {
    pub cabinets: Vec<RefCell<Cabinet>>,
    pub categories: Vec<RefCell<Category>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cabinet {
    pub number: String,
    pub contents: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub name: String,
    pub items: Vec<String>,
    pub level: TruckLevel,
}

impl CheckoffForm {
    pub fn load() -> Result<CheckoffForm, std::io::Error> {
        let checkoff_form_json = std::fs::read_to_string("form.json");

        match checkoff_form_json {
            Ok(c) => Ok(serde_json::from_str::<CheckoffForm>(&c).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let out = serde_json::to_string(&self).unwrap();
        std::fs::write("form.json", out)
    }

    pub(crate) fn new() -> CheckoffForm {
        let mut forms: Vec<RefCell<CheckoffForm>> = Vec::new();
        CheckoffForm{
            cabinets: vec![],
            categories: vec![]
        }
    }
    pub fn add_empty_cabinet(&mut self) {
        self.cabinets.push(RefCell::from(
            Cabinet {
                number: "Name".to_string(),
                contents: "Contents".to_string()
            }))
    }
    pub fn add_empty_category(&mut self) {
        self.categories.push(RefCell::from(
            Category{
                name: "".to_string(),
                items: vec![],
                level: TruckLevel::BLS
            }
        ))
    }
}

impl Category {
    pub fn add_item_row(&mut self) {
        // Due to how formatting is done in the document, items need to be added two at a time
        if self.items.len() % 2 != 0 {
            self.items.push("".to_string());
        } else {
            self.items.push("".to_string());
            self.items.push("".to_string());
        }
    }
    pub fn delete_empties(&mut self) {
        self.items.retain(|x| !x.is_empty());
        if self.items.len() % 2 != 0 {
            self.items.push("".to_string());
        }
    }
}