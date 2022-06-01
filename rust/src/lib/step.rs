use crate::ast::{Sym, step::{Error, System}};

pub fn step(sys: &mut System) {
    unimplemented!()
}

#[derive(Debug)]
pub enum ProcStatus {
    Stepping,
    Linking,
    Halted,
    Error(Error)
}

pub fn get_status(sym: &Sym, sys: &System) -> ProcStatus {
    unimplemented!()
}
