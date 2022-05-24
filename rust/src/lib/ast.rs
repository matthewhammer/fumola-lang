pub type Span = std::ops::Range<usize>;

#[derive(Debug)]
pub enum Exp {
    Nest(Val, Box<Exp>),
    Put(Val, Val),
    Get(Val),
    Link(Val),
    AssertEq(Val, bool, Val),
    Lambda(Pat, Box<Exp>),
    App(Box<Exp>, Val),
    Let(Pat, Box<Exp>, Box<Exp>),
    Ret(Val),
    Switch(Val, Cases),
    Branches(Branches),
    Project(Box<Exp>, Val),
    /// "Let box", as in https://arxiv.org/abs/1703.01288
    LetBx(Pat, Val, Box<Exp>),
    /// explicit "extract" rather than implicit as in https://arxiv.org/abs/1703.01288
    Extract(Val),
    BinOp(Box<Exp>, BinOp, Box<Exp>),
    Var(Id),
}

#[derive(Debug)]
pub enum Val {
    /// The special CBV value form permits us to inject expression syntax into
    /// value syntax, deviating from CBPV. We restore CBPV before
    /// evaluation via a simple transformation that introduces new let-var forms.
    CallByValue(Box<Exp>),
    Sym(Sym),
    Ptr(Sym),
    Proc(Sym),
    Var(Id),
    Num(i32),
    Variant(Box<Val>, Box<Val>),
    Record(Box<RecordVal>),
    RecordExt(Box<Val>, Box<ValField>),
    /// "Code box" as in https://arxiv.org/abs/1703.01288
    Bx(Box<Exp>),
}

pub type Id = String;

pub type RecordVal = Vec<ValField>;

#[derive(Debug)]
pub struct ValField {
    pub label: Val,
    pub value: Val,
}

#[derive(Debug)]
pub enum Branches {
    Empty,
    Gather(Box<Branches>, Box<Branches>),
    Branch(Branch),
}

pub type FieldsPat = Vec<FieldPat>;

#[derive(Debug)]
pub enum Cases {
    Empty,
    Gather(Box<Cases>, Box<Cases>),
    Case(Case),
}

#[derive(Debug)]
pub enum Sym {
    Num(i32),
    Id(Id),
    Bin(Box<Sym>, Box<Sym>),
    Tri(Box<Sym>, Box<Sym>, Box<Sym>),
    Dash,
    Under,
    Dot,
    Tick,
}

#[derive(Debug)]
pub enum Pat {
    Ignore,
    Id(Id),
    Fields(Box<FieldsPat>),
    Case(Box<FieldPat>),
}

#[derive(Debug)]
pub struct FieldPat {
    pub label: Val,
    pub pattern: Pat,
}

#[derive(Debug)]
pub struct Branch {
    pub label: Val,
    pub body: Box<Exp>,
}

#[derive(Debug)]
pub struct Case {
    pub label: Val,
    pub pattern: Pat,
    pub body: Box<Exp>,
}

#[derive(Debug)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

/// Syntactic forms for representing the intermediate state of dynamic
/// evaluation.
pub mod step {
    use super::{Exp, Pat, Sym, Val};

    /// Net surface syntax produces an ast-like structure
    /// to represent an initial net.
    ///
    /// (to do -- surface syntax for all of the above net-internal
    /// structure, and then we can write these more general net forms
    /// too, e.g., to represent a net that we see live somewhere, or
    /// the final form of a net, for behavioral tests)
    #[derive(Debug)]
    pub enum Net {
        Running(Sym, Exp),
        Halted(Sym, Val),
        Gather(Box<Net>, Box<Net>),
    }

    /// Trace-Net pair.  The pair is well-formed when there exists some
    /// initial net N0, without any trace, such that N0 steps to this TraceNet.
    #[derive(Debug)]
    pub struct TraceNet {
        pub trace: Trace,
        pub net: Net,
    }

    /// Trace-Net representation for stepping repeatedly.
    /// Compared with TraceNet, uses Procs in place of Net.
    pub struct TraceNetStep {
        pub trace: Trace,
        pub proc: Procs,
    }

    #[derive(Debug)]
    pub enum Trace {
        Proc(Sym, Box<Trace>),
        Seq(Vec<Trace>),
        Nest(Sym, Box<Trace>),
        Ret(Val),
        Put(Sym, Val),
        Get(Sym, Val),
        Link(Val, Val),
    }

    pub type Procs = std::collections::HashMap<Sym, Process>;

    pub struct Process {
        pub store: Store,
        pub trace: Trace,
        pub state: ProcState,
    }

    pub type Store = std::collections::HashMap<Sym, Val>;

    pub enum ProcState {
        Running(Running),
        Halted(Halted),
    }

    pub struct Running {
        pub stack: Vec<Frame>,
        pub cont: Exp,
    }

    pub struct Halted {
        pub retval: Val,
    }

    pub struct Frame {
        pub cont: FrameCont,
        pub trace: Trace,
    }

    pub enum FrameCont {
        Let(Pat, Exp),
        App(Val),
        Nest(Sym),
    }
}
