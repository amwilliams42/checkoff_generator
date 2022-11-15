use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Checkoffs {
    pub checkoffs: Vec<RefCell<Option<TruckCheck>>>,
}

#[derive(Clone, Debug)]
pub struct TruckCheck {
    pub name: String,
    pub level: TruckLevel,
    pub print: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TruckLevel {
    ALS,
    BLS,
    Vent
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
    pub fn new(name: String, level:TruckLevel) -> Self{
        TruckCheck{
            name,
            level,
            print: false,
        }
    }
}