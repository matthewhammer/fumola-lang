use crate::ast::{Sym, step::{Proc, Running, Error, System}};

/// step a process.
/// returns None for blocked, Error, Halted processes.
pub fn proc(proc: &Proc) -> Option<Proc> {
    use Proc::*;
    match proc {
        Error(_, _) => None,
        Halted(_) => None,
        Running(r) => running(r)
    }
}

/// step a running process.
/// returns None if already Blocked.
pub fn running(running: &Running) -> Option<Proc> {
    // for each Exp form, step it, possibly to an Error.
    match running.cont {
        _ => unimplemented!()
    }
}

pub fn system(sys: &mut System) {
    unimplemented!()
    // loop over procs
    // for each proc, step it once.
    // if it steps, update the system.
    // if not, do not update the system for the step.        
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
