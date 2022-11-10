use std::borrow::{Borrow, BorrowMut};
use std::cell::Cell;

pub struct Checkoffs {
    checkoffs: Cell<Vec<TruckCheck>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct TruckCheck {
    name: String,
    level: String,
}



impl Checkoffs {
    pub fn new(checks: Option<Vec<TruckCheck>>) -> Self {

        let new_checks = match checks{
            Some(c) => c,
            _ => Vec::new()
        };
        Checkoffs {
            checkoffs: Cell::new(new_checks)
        }
    }

    pub fn add(&mut self, check: TruckCheck){
        self.checkoffs.get_mut().push(check);
    }
    pub fn remove(&mut self, check: TruckCheck){
        let checkoffs = self.checkoffs.get_mut();
        checkoffs.retain(|t| *t != check);
    }
}

impl TruckCheck{
    pub fn new(name: String, level:String) -> Self{
        TruckCheck{
            name,
            level,
        }
    }

    pub fn update(&mut self, name: Option<String>, level: Option<String>) {
        match (name,level) {
            (Some(n), Some(l)) => {self.name = n; self.level=l},
            (Some(n), None) => {self.name = n;},
            (None, Some(l)) => {self.level=l},
            (None, None) => {}
        }
    }
}