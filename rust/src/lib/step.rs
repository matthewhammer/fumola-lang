use crate::ast::{
    step::{Env, Error, Frame, FrameCont, PatternError, Proc, Running, System, Trace, ValueError},
    Exp, Pat, Sym, Val, ValField,
};

/// step a process.
/// returns None for processes that are blocked, Error, or Halted.
pub fn proc(proc: &mut Proc) -> Result<(), ()> {
    use Proc::*;
    match proc {
        Error(_, _) => Err(()),
        Halted(_) => Err(()),
        Running(r) => match running(r) {
            Ok(()) => Ok(()),
            Err(err) => {
                *proc = Error(r.clone(), err);
                Ok(())
            }
        },
    }
}

pub fn value_field(env: &Env, value_field: &ValField) -> Result<ValField, ValueError> {
    unimplemented!()
}

pub fn value(env: &Env, v: &Val) -> Result<Val, ValueError> {
    use Val::*;
    match v {
        Sym(_) => Ok(v.clone()),
        Ptr(_) => Ok(v.clone()),
        Proc(_) => Ok(v.clone()),
        Num(_) => Ok(v.clone()),
        Variant(v1, v2) => Ok(Variant(
            Box::new(value(env, v1)?),
            Box::new(value(env, v2)?),
        )),
        Var(x) => match env.get(x) {
            Some(v) => Ok(v.clone()),
            None => Err(ValueError::Undefined(x.clone())),
        },
        Bx(e) => Ok(Bx(e.clone())),
        RecordExt(v1, vf) => Ok(RecordExt(
            Box::new(value(env, v1)?),
            Box::new(value_field(env, vf)?),
        )),
        Record(fs) => {
            let mut v = vec![];
            for r in fs
                .iter()
                .map(|vf: &ValField| -> Result<ValField, ValueError> { value_field(env, vf) })
            {
                v.push(r?)
            }
            Ok(Record(v))
        }
        CallByValue(_) => Err(ValueError::CallByValue),
    }
}

pub fn pattern(env: &mut Env, p: &Pat, v: Val) -> Result<(), PatternError> {
    unimplemented!()
}

/// step a running process.
/// returns None if already Blocked.
pub fn running(r: &mut Running) -> Result<(), Error> {
    // for each Exp form, step it, possibly to an Error.
    use Exp::*;
    use Val::*;
    match &r.cont {
        Ret(v) => {
            let v = value(&r.env, &v)?;
            if r.stack.len() == 0 {
                Err(Error::SignalHalt(v))
            } else {
                let fr = r.stack.pop().ok_or(Error::Impossible)?;
                match fr.cont {
                    FrameCont::Nest(s) => {
                        let tr = std::mem::replace(&mut r.trace, fr.trace);
                        r.trace.push(Trace::Nest(s, Box::new(Trace::Seq(tr))));
                        Ok(())
                    }
                    FrameCont::App(v) => Err(Error::NoStep),
                    FrameCont::Let(p, e) => {
                        pattern(&mut r.env, &p, v)?;
                        let _ = std::mem::replace(&mut r.cont, e);
                        let mut tr = std::mem::replace(&mut r.trace, fr.trace);
                        r.trace.append(&mut tr);
                        Ok(())
                    }
                    _ => unimplemented!(),
                }
            }
        }
        Nest(v, e) => match value(&r.env, &v)? {
            Sym(s) => {
                let trace = std::mem::replace(&mut r.trace, vec![]);
                r.stack.push(Frame {
                    cont: FrameCont::Nest(s),
                    trace,
                });
                r.cont = *e.clone();
                Ok(())
            }
            _ => Err(Error::NoStep),
        },
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
