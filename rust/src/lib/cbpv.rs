use crate::ast::{Exp, Pat, Val};

pub struct Binding {
    pub var: String,
    pub def: Exp,
}

pub type Bindings = Vec<Binding>;

pub fn value<I: Iterator<Item = String>>(
    free_vars: &mut I,
    bindings: &mut Bindings,
    v: &Val,
) -> Result<Val, ()> {
    use Exp::*;
    use Val::*;
    match v {
        CallByValue(e) => {
            let x = free_vars.next().ok_or(())?;
            bindings.push(Binding {
                var: x.clone(),
                def: *e.clone(),
            });
            Ok(Var(x))
        }
        Record(r) => unimplemented!(),
        RecordExt(v1, v2) => unimplemented!(),
        Variant(v1, v2) => unimplemented!(),
        Sym(_) | Ptr(_) | Proc(_) | Num(_) | Var(_) | Bx(_) => Ok(v.clone()),
    }
}

pub fn expression_<I: Iterator<Item = String>>(free_vars: &mut I, e: &Exp) -> Result<Box<Exp>, ()> {
    Ok(Box::new(expression(free_vars, e)?))
}

pub fn expression<I: Iterator<Item = String>>(free_vars: &mut I, e: &Exp) -> Result<Exp, ()> {
    use Exp::*;
    use Val::*;
    let mut bindings = vec![];
    fn wrap(mut bs: Bindings, e: Exp) -> Exp {
        let top = bs.pop();
        match top {
            None => e,
            Some(Binding { var, def }) => Let(Pat::Var(var), Box::new(def), Box::new(wrap(bs, e))),
        }
    }
    match e {
        Extract(v) => {
            let v = value(free_vars, &mut bindings, v)?;
            Ok(wrap(bindings, Extract(v)))
        }
        Nest(v, e) => {
            let v = value(free_vars, &mut bindings, v)?;
            Ok(wrap(bindings, Nest(v, expression_(free_vars, e)?)))
        }
        App(e, v) => {
            let v = value(free_vars, &mut bindings, v)?;
            Ok(wrap(bindings, App(expression_(free_vars, e)?, v)))
        }
        Let(pat, e1, e2) => Ok(Let(
            pat.clone(),
            expression_(free_vars, e1)?,
            expression_(free_vars, e2)?,
        )),
        LetBx(pat, e1, e2) => Ok(LetBx(
            pat.clone(),
            expression_(free_vars, e1)?,
            expression_(free_vars, e2)?,
        )),
        Switch(v, cs) => {
            unimplemented!()
        }
        Branches(bs) => {
            unimplemented!()
        }
        Project(e, v) => {
            let v = value(free_vars, &mut bindings, v)?;
            Ok(wrap(bindings, Project(expression_(free_vars, e)?, v)))
        }
        AssertEq(v1, b, v2) => {
            let v1 = value(free_vars, &mut bindings, v1)?;
            let v2 = value(free_vars, &mut bindings, v2)?;
            Ok(wrap(bindings, AssertEq(v1, *b, v2)))
        }
        Put(v1, v2) => {
            let v1 = value(free_vars, &mut bindings, v1)?;
            let v2 = value(free_vars, &mut bindings, v2)?;
            Ok(wrap(bindings, Put(v1, v2)))
        }
        Link(v) => {
            let v = value(free_vars, &mut bindings, v)?;
            Ok(wrap(bindings, Link(v)))
        }
        Get(v) => {
            let v = value(free_vars, &mut bindings, v)?;
            Ok(wrap(bindings, Get(v)))
        }
    }
}
