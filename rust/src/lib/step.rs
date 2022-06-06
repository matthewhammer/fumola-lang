use crate::ast::{
    step::{
        Env, Error, ExtractError, Frame, FrameCont, Halted, PatternError, Proc, Running, System,
        Trace, ValueError,
    },
    BxVal, Exp, Pat, Sym, Val, ValField,
};

use std::collections::HashMap;

/// step a process.
/// returns None for processes that are blocked, Error, or Halted.
pub fn proc(proc: &mut Proc) -> Result<(), ()> {
    let pr = std::mem::replace(proc, Proc::Spawn(Exp::Hole));
    match pr {
        Proc::Error(_, _) => {
            *proc = pr;
            Err(())
        }
        Proc::Halted(_) => {
            *proc = pr;
            Err(())
        }
        Proc::Spawn(mut e) => {
            *proc = Proc::Running(Running {
                env: Env {
                    vals: HashMap::new(),
                    bxes: HashMap::new(),
                },
                stack: vec![],
                cont: e,
                trace: vec![],
            });
            Ok(())
        }
        Proc::Running(mut r) => match running(&mut r) {
            Ok(()) => {
                *proc = Proc::Running(r);
                Ok(())
            }
            Err(Error::SignalHalt(v)) => {
                *proc = Proc::Halted(Halted { retval: v });
                Ok(())
            }
            Err(err) => {
                *proc = Proc::Error(r.clone(), err);
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
    match p {
        Pat::Var(x) => {
            env.vals.insert(x.clone(), v);
            Ok(())
        }
        _ => unimplemented!(),
    }
}

/// step a running process.
/// returns None if already Blocked.
pub fn running(r: &mut Running) -> Result<(), Error> {
    // for each Exp form, step it, possibly to an Error.
    use std::mem::replace;
    use Exp::*;
    use Val::*;
    let mut cont = replace(&mut r.cont, Hole);
    //println!("running({{cont = {:?}, ...}})", cont);
    match cont {
        Hole => Err(Error::Hole),
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
                    FrameCont::LetBx(env0, Pat::Var(x), e1) => {
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
        LetBx(Pat::Var(x), e1, e2) => {
            let trace = replace(&mut r.trace, vec![]);
            r.stack.push(Frame {
                cont: FrameCont::LetBx(r.env.clone(), Pat::Var(x), *e2),
                trace,
            });
            r.cont = *e1;
            Ok(())
        }
        LetBx(_, e1, e2) => unimplemented!(),
        Extract(Var(x)) => {
            let bxo = r.env.bxes.get(&x);
            let bx = bxo
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

pub fn system(sys: &mut System) -> Result<(), Error> {
    if sys.procs.len() == 0 {
        return Err(Error::NoProcs);
    }
    let mut stepped = false;
    for (_, p) in sys.procs.iter_mut() {
        match proc(p) {
            Ok(()) => stepped = true,
            Err(()) => (),
        }
    }
    if stepped {
        return Ok(());
    } else {
        return Err(Error::NoStep);
    }
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
