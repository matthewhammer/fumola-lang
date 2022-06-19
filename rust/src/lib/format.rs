#![allow(unused_imports)]
use crate::ast::{
    step::{
        Env, Error, ExtractError, Frame, FrameCont, Halted, InternalError, PatternError, Proc,
        Procs, ProjectError, Running, Signal, Stack, Store, SwitchError, System, Trace, Traces,
        ValsEnv, ValueError,
    },
    Branch, Branches, BxVal, BxesEnv, Case, Cases, Exp, FieldPat, FieldsPat, Pat, RecordVal, Sym,
    Val, ValField,
};

use std::fmt;

use std::collections::HashMap;

impl fmt::Display for FieldsPat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = self.0.iter().peekable();
        while let Some(fld) = i.next() {
            let l = &fld.label;
            let p = &fld.pattern;
            write!(f, "{} => {}", l, p)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Pat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Pat::*;
        match self {
            Ignore => write!(f, "_"),
            Var(x) => write!(f, "{}", x),
            Fields(fs) => write!(f, "[{}]", fs),
            Case(c) => write!(f, "{}({})", c.label, c.pattern),
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Exp::*;
        match self {
            Nest(v, e) => write!(f, "#{} {{ {} }}", v, e),
            Spawn(v, e) => write!(f, "~{} {{ {} }}", v, e),
            Put(v1, v2) => write!(f, "{} := {}", v1, v2),
            Get(v) => write!(f, "@{}", v),
            Link(v) => write!(f, "&{}", v),
            AssertEq(v1, true, v2) => write!(f, "{} == {}", v1, v2),
            AssertEq(v1, false, v2) => write!(f, "{} != {}", v1, v2),
            Lambda(p, e) => write!(f, "\\{} => {}", p, e),
            App(e, v) => write!(f, "{} {}", e, v),
            Let(p, e1, e2) => write!(f, "let {} = {}; {}", p, e1, e2),
            LetBx(p, e1, e2) => write!(f, "let box {} = {}; {}", p, e1, e2),
            Ret(v) => write!(f, "ret {}", v),
            Ret_(v) => write!(f, "ret_ {}", v),
            Switch(v, cases) => write!(f, "switch {} {{ {} }}", v, cases),
            Branches(bs) => write!(f, "{{ {} }}", bs),
            Project(e, v) => write!(f, "{} => {}", e, v),
            Extract(v) => write!(f, "{}", v),
            Hole => write!(f, "__"),
        }
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}({}) => {}", self.label, self.pattern, self.body)
    }
}

impl fmt::Display for Cases {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Cases::*;
        match self {
            Empty => write!(f, ""),
            Gather(b1, b2) => write!(f, "{}; {}", b1, b2),
            Case(c) => write!(f, "{}", c),
        }
    }
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.label, self.body)
    }
}

impl fmt::Display for ValField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.label, self.value)
    }
}

impl fmt::Display for RecordVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = self.0.iter().peekable();
        while let Some(fld) = i.next() {
            let l = &fld.label;
            let v = &fld.value;
            write!(f, "{} => {}", l, v)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Branches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Branches::*;
        match self {
            Empty => write!(f, ""),
            Gather(b1, b2) => write!(f, "{}; {}", b1, b2),
            Branch(b) => write!(f, "{}", b),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Val::*;
        match self {
            CallByValue(e) => write!(f, "`({})", e),
            Sym(s) => write!(f, "${}", s),
            Ptr(s) => write!(f, "!{}", s),
            Proc(s) => write!(f, "~{}", s),
            Var(i) => write!(f, "{}", i),
            Num(n) => write!(f, "{}", n),
            Variant(v1, v2) => write!(f, "#{}({})", v1, v2),
            Record(r) => write!(f, "[{}]", r),
            RecordExt(v, fld) => write!(f, "{}, {} => {}", v, fld.label, fld.value),
            Bx(bx) => write!(f, "{}", bx),
        }
    }
}

impl fmt::Display for BxVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.name {
            None => write!(f, "{{{} |- {}}}", self.bxes, self.code),
            Some(n) => write!(f, "rec {} {{{} |- {}}}", n, self.bxes, self.code),
        }
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fumola [\n  store = {};\n  procs = {}\n]\n",
            self.store, self.procs
        )
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Sym::*;
        match self {
            None => write!(f, "%"),
            Num(n) => write!(f, "{}", n),
            Id(i) => write!(f, "{}", i),
            Bin(s1, s2) => write!(f, "{}{}", s1, s2),
            Nest(s1, s2) => write!(f, "{}/{}", s1, s2),
            Tri(s1, s2, s3) => write!(f, "{}{}{}", s1, s2, s3),
            Dash => write!(f, "-"),
            Under => write!(f, "_"),
            Dot => write!(f, "."),
            Tick => write!(f, "'"),
        }
    }
}

impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Trace::*;
        match self {
            Seq(ts) => {
                let mut i = ts.iter().peekable();
                while let Some(tr) = i.next() {
                    write!(f, "{}", tr)?;
                    if i.peek().is_some() {
                        write!(f, "; ")?;
                    }
                }
                Ok(())
            }
            Nest(s, ts) => {
                write!(f, "#{} {{", s)?;
                let mut i = ts.iter().peekable();
                while let Some(tr) = i.next() {
                    write!(f, "{}", tr)?;
                    if i.peek().is_some() {
                        write!(f, "; ")?;
                    }
                }
                write!(f, "}}")?;
                Ok(())
            }
            Ret(v) => write!(f, "ret {}", v),
            Put(s, v) => write!(f, "put {} <= {}", s, v),
            Get(s, v) => write!(f, "get {} => {}", s, v),
            Link(v1, v2) => write!(f, "link {} => {}", v1, v2),
        }
    }
}

impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Proc::Spawn(e) => write!(f, "spawn({})", e),
            Proc::Running(r) => write!(f, "running({})", r),
            Proc::WaitingForPtr(r, s) => write!(f, "waitingForPtr({}, {})", r, s),
            Proc::WaitingForHalt(r, s) => write!(f, "waitingForHalt({}, {})", r, s),
            Proc::Error(r, e) => write!(f, "error({}, {})", e, r),
            Proc::Halted(h) => write!(f, "halted({})", h.trace),
        }
    }
}
impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Signal::*;
        match self {
            Halt(v) => write!(f, "halt({})", v),
            LinkWaitPtr(s) => write!(f, "linkWaitPtr({})", s),
            LinkWaitHalt(s) => write!(f, "linkWaitHalt({})", s),
            Spawn(s, env, e) => write!(
                f,
                "spawn([name = {}; bxes = {}; vals = {}; code = {}])",
                s, env.bxes, env.vals, e
            ),
        }
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InternalError::*;
        match self {
            Impossible => write!(f, "impossible"),
            Hole => write!(f, "hole"),
        }
    }
}

impl fmt::Display for SwitchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SwitchError::*;
        match self {
            NotVariant(v) => write!(f, "notVariant({})", v),
            MissingCase(s) => write!(f, "missingCase({})", s),
        }
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValueError::*;
        match self {
            CallByValue => write!(f, "callByValue"),
            Undefined(i) => write!(f, "undefined({})", i),
        }
    }
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PatternError::*;
        match self {
            NotVariant => write!(f, "notVariant"),
            NotRecord => write!(f, "notRecord"),
            FieldNotFound(v) => write!(f, "fieldNotFound({})", v),
        }
    }
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ExtractError::*;
        match self {
            Undefined(i) => write!(f, "undefined({})", i),
        }
    }
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ProjectError::*;
        match self {
            MissingBranch(s) => write!(f, "missingBranch({})", s),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Signal(s) => write!(f, "signal({})", s),
            Internal(i) => write!(f, "internal({})", i),
            NoProcs => write!(f, "noProcs"),
            Pattern(p) => write!(f, "pattern({})", p),
            Value(v) => write!(f, "value({})", v),
            Extract(e) => write!(f, "extract({})", e),
            Switch(s) => write!(f, "switch({})", s),
            Project(p) => write!(f, "project({})", p),
            NoStep => write!(f, "noStep"),
            NotASymbol(s) => write!(f, "notASymbol({})", s),
            NotAPointer(s) => write!(f, "notAPointer({})", s),
            InvalidProc(s) => write!(f, "invalidProc({})", s),
            NotLinkTarget(v) => write!(f, "notLinkTarget({})", v),
            Undefined(s) => write!(f, "undefined({})", s),
            Duplicate(s) => write!(f, "duplicate({})", s),
            AssertionFailure(v1, true, v2) => write!(f, "assertionFailure({} == {})", v1, v2),
            AssertionFailure(v1, false, v2) => write!(f, "assertionFailure({} != {})", v1, v2),
        }
    }
}

impl fmt::Display for Running {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[trace = {}; stack = {}; bxes = {}; vals = {}; cont = {}]",
            self.trace, self.stack, self.env.bxes, self.env.vals, self.cont,
        )
    }
}

impl fmt::Display for Traces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut i = self.0.iter().peekable();
        while let Some(tr) = i.next() {
            write!(f, "{}", tr)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for Procs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut ps: Vec<_> = self.0.keys().collect();
        ps.sort();
        let mut i = ps.iter().peekable();
        while let Some(p) = i.next() {
            let proc = self.0.get(*p).ok_or(fmt::Error)?;
            write!(f, "{} => {}", p, proc)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = self.0.iter().peekable();
        write!(f, "[")?;
        while let Some(fr) = i.next() {
            write!(f, "[trace = {}, cont = {}]", fr.trace, fr.cont)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for FrameCont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FrameCont::*;
        match self {
            LetBx(env, p, e) => write!(
                f,
                "{} ;; {} |- let box {} = __; {}",
                env.bxes, env.vals, p, e
            ),
            Let(env, p, e) => write!(f, "{} ;; {} |- let {} = __; {}", env.bxes, env.vals, p, e),
            App(v) => write!(f, "__ {}", v),
            Project(v) => write!(f, "__ <= {}", v),
            Nest(s) => write!(f, "#{} {{ __ }}", s),
        }
    }
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut xs: Vec<_> = self.0.keys().collect();
        xs.sort();
        let mut i = xs.iter().peekable();
        while let Some(x) = i.next() {
            let v = self.0.get(*x).ok_or(fmt::Error)?;
            write!(f, "{} => {}", x, v)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for ValsEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut xs: Vec<_> = self.0.keys().collect();
        xs.sort();
        let mut i = xs.iter().peekable();
        while let Some(x) = i.next() {
            let v = self.0.get(*x).ok_or(fmt::Error)?;
            write!(f, "{} => {}", x, v)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for BxesEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut xs: Vec<_> = self.0.keys().collect();
        xs.sort();
        let mut i = xs.iter().peekable();
        while let Some(x) = i.next() {
            let v = self.0.get(*x).ok_or(fmt::Error)?;
            write!(f, "{} => {}", x, v)?;
            if i.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")
    }
}
