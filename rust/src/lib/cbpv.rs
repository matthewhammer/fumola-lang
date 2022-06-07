use crate::ast::{BxVal, Exp, Pat, Val};

use std::collections::HashMap;

pub struct Binding {
    pub var: String,
    pub def: Exp,
}

pub type Bindings = Vec<Binding>;

fn convert_<I: Iterator<Item = String>>(free_vars: &mut I, e: &Exp) -> Result<Box<Exp>, ()> {
    Ok(Box::new(convert(free_vars, e)?))
}

pub fn convert<I: Iterator<Item = String>>(free_vars: &mut I, e: &Exp) -> Result<Exp, ()> {
    let mut bindings = vec![];
    let e = expression(free_vars, &mut bindings, e)?;
    Ok(wrap(bindings, e))
}

fn wrap(mut bs: Bindings, e: Exp) -> Exp {
    let top = bs.pop();
    match top {
        None => e,
        Some(Binding { var, def }) => Exp::Let(Pat::Var(var), Box::new(def), Box::new(wrap(bs, e))),
    }
}

fn value<I: Iterator<Item = String>>(
    free_vars: &mut I,
    bindings: &mut Bindings,
    v: &Val,
) -> Result<Val, ()> {
    use Exp::*;
    use Val::*;
    match v {
        CallByValue(e) => {
            let def = expression(free_vars, bindings, e)?;
            let var = free_vars.next().ok_or(())?;
            let res = Ok(Var(var.clone()));
            bindings.push(Binding { var, def });
            res
        }
        Bx(bx) => Ok(Bx(Box::new(BxVal {
            bxes: HashMap::new(),
            name: bx.name.clone(),
            code: convert(free_vars, &bx.code)?,
        }))),
        Record(r) => unimplemented!(),
        RecordExt(v1, v2) => unimplemented!(),
        Variant(v1, v2) => unimplemented!(),
        Sym(_) | Ptr(_) | Proc(_) | Num(_) | Var(_) => Ok(v.clone()),
    }
}

fn expression_<I: Iterator<Item = String>>(
    free_vars: &mut I,
    bindings: &mut Bindings,
    e: &Exp,
) -> Result<Box<Exp>, ()> {
    Ok(Box::new(expression(free_vars, bindings, e)?))
}

fn expression<I: Iterator<Item = String>>(
    free_vars: &mut I,
    bindings: &mut Bindings,
    e: &Exp,
) -> Result<Exp, ()> {
    use Exp::*;
    use Val::*;
    match e {
        Hole => Ok(Hole),
        Extract(v) => {
            let v = value(free_vars, bindings, v)?;
            Ok(Extract(v))
        }
        Ret(v) => {
            let v = value(free_vars, bindings, v)?;
            Ok(Ret(v))
        }
        Nest(v, e) => {
            let v = value(free_vars, bindings, v)?;
            Ok(Nest(v, expression_(free_vars, bindings, e)?))
        }
        App(e, v) => {
            let v = value(free_vars, bindings, v)?;
            Ok(App(expression_(free_vars, bindings, e)?, v))
        }
        Let(pat, e1, e2) => Ok(Let(
            pat.clone(),
            convert_(free_vars, e1)?,
            convert_(free_vars, e2)?,
        )),
        LetBx(pat, e1, e2) => Ok(LetBx(
            pat.clone(),
            convert_(free_vars, e1)?,
            convert_(free_vars, e2)?,
        )),
        Switch(v, cs) => {
            unimplemented!()
        }
        Branches(bs) => {
            unimplemented!()
        }
        Lambda(pat, e) => Ok(Lambda(pat.clone(), expression_(free_vars, bindings, e)?)),
        Project(e, v) => {
            let v = value(free_vars, bindings, v)?;
            Ok(Project(expression_(free_vars, bindings, e)?, v))
        }
        AssertEq(v1, b, v2) => {
            let v1 = value(free_vars, bindings, v1)?;
            let v2 = value(free_vars, bindings, v2)?;
            Ok(AssertEq(v1, *b, v2))
        }
        Put(v1, v2) => {
            let v1 = value(free_vars, bindings, v1)?;
            let v2 = value(free_vars, bindings, v2)?;
            Ok(Put(v1, v2))
        }
        Link(v) => {
            let v = value(free_vars, bindings, v)?;
            Ok(Link(v))
        }
        Get(v) => {
            let v = value(free_vars, bindings, v)?;
            Ok(Get(v))
        }
    }
}