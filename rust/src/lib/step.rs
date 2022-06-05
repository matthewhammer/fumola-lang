use crate::ast::{
    step::{
        Env, Error, ExtractError, Frame, FrameCont, PatternError, Proc, Running, System, Trace,
        ValueError,
    },
    BxVal, Exp, Pat, Sym, Val, ValField,
};

use std::collections::HashMap;

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
        Var(x) => match env.vals.get(x) {
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

/// Try to match closed value against pattern.
/// Updates the environment for each pattern-identifier match, even if pattern error.
pub fn pattern(p: &Pat, v: Val, env: &mut Env) -> Result<(), PatternError> {
    unimplemented!()
}

/// step a running process.
/// returns None if already Blocked.
pub fn running(r: &mut Running) -> Result<(), Error> {
    // for each Exp form, step it, possibly to an Error.
    use std::mem::replace;
    use Exp::*;
    use Val::*;
    let mut cont = replace(&mut r.cont, Hole);
    match cont {
        Hole => Err(Error::NoStep),
        Ret(v) => {
            let v = value(&r.env, &v)?;
            if r.stack.len() == 0 {
                Err(Error::SignalHalt(v))
            } else {
                let fr = r.stack.pop().ok_or(Error::Impossible)?;
                match fr.cont {
                    FrameCont::App(v) => Err(Error::NoStep),
                    FrameCont::Nest(s) => {
                        let tr = replace(&mut r.trace, fr.trace);
                        r.trace.push(Trace::Nest(s, Box::new(Trace::Seq(tr))));
                        Ok(())
                    }
                    FrameCont::Let(mut env0, pat, e1) => {
                        pattern(&pat, v, &mut env0)?;
                        let _ = replace(&mut r.env, env0);
                        let _ = replace(&mut r.cont, e1);
                        let mut tr = replace(&mut r.trace, fr.trace);
                        r.trace.append(&mut tr);
                        Ok(())
                    }
                    FrameCont::LetBx(env0, Pat::Id(x), e1) => {
                        if let Bx(bv) = v {
                            let _ = replace(&mut r.env, env0);
                            let _ = replace(&mut r.cont, e1);
                            let mut tr = replace(&mut r.trace, fr.trace);
                            r.trace.append(&mut tr);
                            r.env.bxes.insert(x, *bv);
                            Ok(())
                        } else {
                            Err(Error::NoStep)
                        }
                    }
                    FrameCont::LetBx(_, _, _) => unimplemented!(),
                }
            }
        }
        Nest(v, e) => match value(&r.env, &v)? {
            Sym(s) => {
                let trace = replace(&mut r.trace, vec![]);
                r.stack.push(Frame {
                    cont: FrameCont::Nest(s),
                    trace,
                });
                r.cont = *e;
                Ok(())
            }
            _ => Err(Error::NoStep),
        },
        Let(pat, e1, e2) => {
            let trace = replace(&mut r.trace, vec![]);
            r.stack.push(Frame {
                cont: FrameCont::Let(r.env.clone(), pat, *e2),
                trace,
            });
            r.cont = *e1;
            Ok(())
        }
        LetBx(Pat::Id(x), e1, e2) => {
            let trace = replace(&mut r.trace, vec![]);
            r.stack.push(Frame {
                cont: FrameCont::LetBx(r.env.clone(), Pat::Id(x), *e2),
                trace,
            });
            r.cont = *e1;
            Ok(())
        }
        LetBx(_, e1, e2) => unimplemented!(),
        Extract(Var(x)) => {
            let bx = r
                .env
                .bxes
                .get(&x)
                .ok_or(Error::Extract(ExtractError::Undefined(x)))?
                .clone();
            r.env.vals = HashMap::new();
            if let Some(name) = bx.name.clone() {
                drop(r.env.vals.insert(name, Bx(Box::new(bx.clone()))))
            }
            r.env.bxes = bx.bxes;
            r.cont = bx.code;
            Ok(())
        }
        Extract(_) => unimplemented!(),

        // To do
        // ------

        // Lambda(Pat, Box<Exp>),
        // App(Box<Exp>, Val),

        // Project(Box<Exp>, Val),
        // Branches(Branches),

        // Put(Val, Val),
        // Get(Val),
        // Link(Val),
        // AssertEq(Val, bool, Val),

        // Switch(Val, Cases),
        _ => unimplemented!(),
    }
}

pub fn system(sys: &mut System) {
    unimplemented!()
    // loop:
    // each round: for each proc, attempt to step it once.
    // if any proc steps, continue for another round; otherwise, end.
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
