use crate::ast::{
    step::{Proc, Procs, Store, System},
    Exp, Sym,
};
use crate::cbpv::FreeVarsNoNext;

use std::collections::HashMap;

pub struct FreeVars {
    pub base: String,
    pub index: u32,
}

impl Iterator for FreeVars {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let r = Some(format!("{}{}", self.base, self.index));
        self.index += 1;
        r
    }
}

pub fn system_from_exp(e: &Exp) -> Result<System, FreeVarsNoNext> {
    let mut fv = FreeVars {
        base: "_t_".to_string(),
        index: 0,
    };
    let mut procs = HashMap::new();
    let e = crate::cbpv::convert(&mut fv, e)?;
    procs.insert(Sym::None, Proc::Spawn(e));
    Ok(System {
        store: Store(HashMap::new()),
        procs: Procs(procs),
    })
}

pub fn exp(
    input: &str,
    parse_ast: Option<&str>,
    final_system: Option<&str>,
) -> Result<(), FreeVarsNoNext> {
    let expr = crate::parser::ExpParser::new().parse(input).unwrap();
    match parse_ast {
        None => (),
        Some(a) => {
            assert_eq!(&format!("{:?}", expr), a);
        }
    };
    let mut sys = system_from_exp(&expr)?;
    crate::step::fully(&mut sys);
    println!("final system:\n{}", &sys);
    match final_system {
        None => (),
        Some(s) => {
            assert_eq!(&format!("{}", &sys), s);
        }
    };
    Ok(())
}

pub fn parse(input: &str, ast: &str) -> Result<(), FreeVarsNoNext> {
    exp(input, Some(ast), None)
}

pub fn step_fully(input: &str, final_sys: &str) -> Result<(), FreeVarsNoNext> {
    exp(input, None, Some(final_sys))
}
