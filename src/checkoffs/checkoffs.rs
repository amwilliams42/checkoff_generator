use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Checkoffs {
    pub checkoffs: Vec<RefCell<Option<TruckCheck>>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone, Debug)]
pub struct TruckCheck {
    name: String,
    level: String,
}



impl Checkoffs {
    pub fn new(checks: Option<Vec<TruckCheck>>) -> Self {

        let mut v = Vec::new();
        match checks{
            Some(c) => {
                for chk in c {
                    v.push(RefCell::new(Some(chk)))
                }
            },
            _ => {
                v.push(RefCell::new(None))
            }
        };
        Checkoffs {
            checkoffs: v
        }
    }

    pub fn add(&mut self, check: TruckCheck){
        let new_tc: RefCell<Option<TruckCheck>> = RefCell::new(Some(check));
        self.checkoffs.push(new_tc)
    }
}

impl Display for TruckCheck{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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