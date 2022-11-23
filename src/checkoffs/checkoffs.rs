use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Debug)]
pub struct Checkoffs {
    pub checkoffs: Vec<RefCell<TruckCheck>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TruckCheck {
    pub name: String,
    pub level: TruckLevel,
    pub print: bool,
    pub id: usize,
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
                    v.push(RefCell::new(chk))
                }
            },
            _ => {
                v.push(RefCell::new(TruckCheck::default()))
            }
        };
        Checkoffs {
            checkoffs: v
        }
    }

    pub fn add(&mut self, check: TruckCheck){
        let new_tc: RefCell<TruckCheck> = RefCell::new(check);
        self.checkoffs.push(new_tc)
    }
    pub fn remove(&mut self, check: &mut TruckCheck) {
        println!("Before: {:?}", self.checkoffs );
        self.checkoffs.retain(|ch| ch.borrow().to_owned() != check.to_owned());
        println!("After: {:?}", self.checkoffs );

    }
}

impl Display for TruckCheck{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for TruckCheck {
    fn default() -> Self {
        TruckCheck::new("Input Name Here".to_string(), TruckLevel::ALS)
    }
}

impl TruckCheck{
    pub fn new(name: String, level:TruckLevel) -> Self{
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        TruckCheck{
            name,
            level,
            print: false,
            id: COUNTER.fetch_add(1, Ordering::Release),
        }
    }
}