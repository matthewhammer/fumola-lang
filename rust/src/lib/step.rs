use crate::ast::{
    step::{Env, Error, Proc, Running, System, ValueError},
    Exp, Sym, Val, ValField,
};

/// step a process.
/// returns None for processes that are blocked, Error, or Halted.
pub fn proc(proc: &Proc) -> Option<Proc> {
    use Proc::*;
    match proc {
        Error(_, _) => None,
        Halted(_) => None,
        Running(r) => match running(r) {
            Ok(proc) => Some(proc),
            Err(err) => Some(Error(r.clone(), err)),
        },
    }
}

pub fn value_field(env: &Env, value_field: &ValField) -> Result<ValField, ValueError> {
    unimplemented!()
}

pub fn value(env: &Env, v: &Val) -> Result<Val, ValueError> {
    use Val::*;
    match v {
        Var(x) => match env.get(x) {
            Some(v) => Ok(v.clone()),
            None => Err(ValueError::Undefined(x.clone())),
        },
        Bx(e) => Ok(Bx(e.clone())),
        RecordExt(v1, vf) => Ok(RecordExt(
            Box::new(value(env, v1)?),
            Box::new(value_field(env, vf)?),
        )),
        CallByValue(_) => Err(ValueError::CallByValue),
        _ => unimplemented!(),
    }
}

/// step a running process.
/// returns None if already Blocked.
pub fn running(r: &Running) -> Result<Proc, Error> {
    // for each Exp form, step it, possibly to an Error.
    use Exp::*;
    use Val::*;
    match &r.cont {
        Nest(v, e) => {
            match value(&r.env, &v)? {
                Sym(s) => {
                    unimplemented!()
                    // push stack with `nest s`
                    // continue with body of nest.
                }
                _ => Err(Error::NoStep),
            }
        }
        _ => unimplemented!(),
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
    Error(Error),
}

pub fn get_status(sym: &Sym, sys: &System) -> ProcStatus {
    unimplemented!()
}
