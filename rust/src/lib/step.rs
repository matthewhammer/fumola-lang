use crate::ast::{
    step::{
        Env, Error, ExtractError, Frame, FrameCont, Halted, InternalError, PatternError, Proc,
        ProjectError, Running, Signal, Stack, Store, SwitchError, System, Trace, ValueError,
    },
    Branch, Branches, Case, Cases, Exp, Pat, Sym, Val, ValField,
};

use std::collections::HashMap;

/// step a process.
/// returns None for processes that are blocked, Error, or Halted.
pub fn proc(store: &mut Store, proc: &mut Proc) -> Result<(), ()> {
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
        Proc::Spawn(e) => {
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
        Proc::Running(mut r) => match running(store, &mut r) {
            Ok(()) => {
                *proc = Proc::Running(r);
                Ok(())
            }
            Err(Error::Signal(Signal::Halt(v))) => {
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

pub fn value_field(_env: &Env, _value_field: &ValField) -> Result<ValField, ValueError> {
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
        Pat::Ignore => Ok(()),
        Pat::Var(x) => {
            env.vals.insert(x.clone(), v);
            Ok(())
        }
        _ => unimplemented!(),
    }
}

/// Shallow copy of expression head, using holes for subexpressions.
///
/// For debugging purposes, when we "take" the continuation from the
/// system state to step it, we replace it with a (shallow) copy of
/// its head expression, where each subexpression is a hole.  That
/// way, the person debugging the stuck program can have a local copy
/// of where it got stuck, but we avoid a full deep copy for each step.
pub fn head(e: &Exp) -> Exp {
    use Exp::*;
    fn hole() -> Box<Exp> {
        Box::new(Hole)
    }
    match e {
        Nest(v, _) => Nest(v.clone(), hole()),
        Lambda(pat, _) => Lambda(pat.clone(), hole()),
        Put(v1, v2) => Put(v1.clone(), v2.clone()),
        Get(v) => Get(v.clone()),
        Link(v) => Link(v.clone()),
        Ret(v) => Ret(v.clone()),
        Switch(v, c) => Switch(v.clone(), head_cases(c)),
        Let(pat, _e1, _e2) => Let(pat.clone(), hole(), hole()),
        LetBx(pat, _e1, _e2) => LetBx(pat.clone(), hole(), hole()),
        Extract(v) => Extract(v.clone()),
        Hole => Hole,
        App(_e1, v) => App(hole(), v.clone()),
        Project(_e, v) => Project(hole(), v.clone()),
        Branches(b) => Branches(head_branches(b)),
        AssertEq(v1, b, v2) => AssertEq(v1.clone(), *b, v2.clone()),
    }
}

pub fn head_branches(_b: &Branches) -> Branches {
    // to do -- for each branch case, keep pattern and hole the body.
    Branches::Empty
}

pub fn head_cases(_c: &Cases) -> Cases {
    // to do -- for each branch case, keep pattern and hole the body.
    Cases::Empty
}

pub fn into_symbol(v: Val) -> Result<Sym, Error> {
    match v {
        Val::Sym(s) => Ok(s),
        _ => Err(Error::NotASymbol(v)),
    }
}

pub fn put_symbol(stack: &Stack, s: Sym) -> Sym {
    let mut r = s;
    for fr in stack.iter().rev() {
        match &fr.cont {
            FrameCont::Nest(ns) => r = Sym::Nest(Box::new(ns.clone()), Box::new(r)),
            _ => (),
        }
    }
    r
}

/// step a running process.
/// returns None if already Blocked.
pub fn running(store: &mut Store, r: &mut Running) -> Result<(), Error> {
    // for each Exp form, step it, possibly to an Error.
    use std::mem::replace;
    use Exp::*;
    use Val::*;
    let h = head(&r.cont);
    println!("running({{cont = {:?}, ...}})", h);
    let cont = replace(&mut r.cont, h);
    match cont {
        Hole => Err(Error::Internal(InternalError::Hole)),
        Ret(v) => {
            let v = value(&r.env, &v)?;
            if r.stack.len() == 0 {
                Err(Error::Signal(Signal::Halt(v)))
            } else {
                let fr = r
                    .stack
                    .last()
                    .ok_or(Error::Internal(InternalError::Impossible))?
                    .clone();
                match fr.cont {
                    FrameCont::App(_) | FrameCont::Project(_) => Err(Error::NoStep),
                    FrameCont::Nest(s) => {
                        r.stack
                            .pop()
                            .ok_or(Error::Internal(InternalError::Impossible))?;
                        let tr = replace(&mut r.trace, fr.trace);
                        r.trace.push(Trace::Nest(s, Box::new(Trace::Seq(tr))));
                        Ok(())
                    }
                    FrameCont::Let(mut env0, pat, e1) => {
                        r.stack
                            .pop()
                            .ok_or(Error::Internal(InternalError::Impossible))?;
                        pattern(&pat, v, &mut env0)?;
                        let _ = replace(&mut r.env, env0);
                        let _ = replace(&mut r.cont, e1);
                        let mut tr = replace(&mut r.trace, fr.trace);
                        r.trace.append(&mut tr);
                        Ok(())
                    }
                    FrameCont::LetBx(env0, Pat::Var(x), e1) => {
                        if let Bx(bv) = v {
                            r.stack
                                .pop()
                                .ok_or(Error::Internal(InternalError::Impossible))?;
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
        LetBx(_, _e1, _e2) => unimplemented!(),
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
        Lambda(pat, e1) => {
            if r.stack.len() == 0 {
                Err(Error::NoStep)
            } else {
                let fr = r
                    .stack
                    .pop()
                    .ok_or(Error::Internal(InternalError::Impossible))?;
                match fr.cont {
                    FrameCont::App(v) => {
                        pattern(&pat, v, &mut r.env)?;
                        let mut tr = replace(&mut r.trace, fr.trace);
                        r.trace.append(&mut tr);
                        let _ = replace(&mut r.cont, *e1);
                        Ok(())
                    }
                    _ => Err(Error::NoStep),
                }
            }
        }
        Branches(branches) => {
            if r.stack.len() == 0 {
                Err(Error::NoStep)
            } else {
                let fr = r
                    .stack
                    .pop()
                    .ok_or(Error::Internal(InternalError::Impossible))?;
                match fr.cont {
                    FrameCont::Project(v) => {
                        let sym = into_symbol(v)?;
                        let br = project_branch(&r.env, &sym, branches)?;
                        let mut tr = replace(&mut r.trace, fr.trace);
                        r.trace.append(&mut tr);
                        let _ = replace(&mut r.cont, *br.body);
                        Ok(())
                    }
                    _ => Err(Error::NoStep),
                }
            }
        }
        App(e1, v) => {
            let v = value(&r.env, &v)?;
            let trace = replace(&mut r.trace, vec![]);
            r.stack.push(Frame {
                cont: FrameCont::App(v),
                trace,
            });
            r.cont = *e1;
            Ok(())
        }
        Project(e1, v) => {
            let v = value(&r.env, &v)?;
            let trace = replace(&mut r.trace, vec![]);
            r.stack.push(Frame {
                cont: FrameCont::Project(v),
                trace,
            });
            r.cont = *e1;
            Ok(())
        }
        Put(v1, v2) => {
            let v1 = value(&r.env, &v1)?;
            let sym = into_symbol(v1)?;
            let v2 = value(&r.env, &v2)?;
            let sym = put_symbol(&r.stack, sym);
            r.trace.push(Trace::Put(sym.clone(), v2.clone()));
            store.insert(sym.clone(), v2);
            r.cont = Ret(Sym(sym));
            Ok(())
        }
        Get(v) => {
            let v1 = value(&r.env, &v)?;
            let sym = into_symbol(v1)?;
            let v2 = match store.get(&sym) {
                None => return Err(Error::Undefined(sym)),
                Some(v2) => v2.clone(),
            };
            r.trace.push(Trace::Get(sym.clone(), v2.clone()));
            r.cont = Ret(v2);
            Ok(())
        }
        Switch(v, cases) => {
            let v = value(&r.env, &v)?;
            match v {
                Val::Variant(v1, v2) => {
                    let sym = into_symbol(*v1)?;
                    let case = switch_case(&r.env, &sym, cases)?;
                    pattern(&case.pattern, *v2, &mut r.env)?;
                    r.cont = *case.body;
                    Ok(())
                }
                v => Err(Error::Switch(SwitchError::NotVariant(v))),
            }
        }
        // To do
        // ------

        // Link(Val),
        // AssertEq(Val, bool, Val),
        _ => unimplemented!(),
    }
}

pub fn project_branch(env: &Env, sym: &Sym, bs: Branches) -> Result<Branch, Error> {
    match bs {
        Branches::Empty => Err(Error::Project(ProjectError::MissingBranch(sym.clone()))),
        Branches::Gather(b1, b2) => match project_branch(env, sym, *b1) {
            Ok(e) => Ok(e),
            Err(Error::Project(ProjectError::MissingBranch(_))) => project_branch(env, sym, *b2),
            Err(e) => Err(e),
        },
        Branches::Branch(branch) => {
            let label = value(env, &branch.label)?;
            let label_sym = into_symbol(label)?;
            if &label_sym == sym {
                Ok(branch)
            } else {
                Err(Error::Project(ProjectError::MissingBranch(sym.clone())))
            }
        }
    }
}

pub fn switch_case(env: &Env, sym: &Sym, cases: Cases) -> Result<Case, Error> {
    match cases {
        Cases::Empty => Err(Error::Switch(SwitchError::MissingCase(sym.clone()))),
        Cases::Gather(cases1, cases2) => match switch_case(env, sym, *cases1) {
            Ok(e) => Ok(e),
            Err(Error::Switch(SwitchError::MissingCase(_))) => switch_case(env, sym, *cases2),
            Err(e) => Err(e),
        },
        Cases::Case(case) => {
            let label = value(env, &case.label)?;
            let label_sym = into_symbol(label)?;
            if &label_sym == sym {
                Ok(case)
            } else {
                Err(Error::Switch(SwitchError::MissingCase(sym.clone())))
            }
        }
    }
}

pub fn system(sys: &mut System) -> Result<(), Error> {
    if sys.procs.len() == 0 {
        return Err(Error::NoProcs);
    }
    let mut stepped = false;
    for (_, p) in sys.procs.iter_mut() {
        match proc(&mut sys.store, p) {
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
