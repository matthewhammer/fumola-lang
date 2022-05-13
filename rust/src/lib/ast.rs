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

pub type Branches = Vec<Branch>;

pub type FieldsPat = Vec<FieldPat>;

pub type Cases = Vec<Case>;

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
    use super::{Val, Exp, Sym, Pat};

    pub enum Trace {
        Seq(Vec<Trace>),
        Nest(Sym, Box<Trace>),
        Ret(Val),
        Put(Sym, Val),
        Get(Sym, Val),
        Link(Val, Sym),
    }

    pub enum FrameCont {
        Let(Pat, Exp),
        App(Val),
        Nest(Sym),
    }

    pub struct Frame {
        pub cont: FrameCont,
        pub trace: Trace,
    }

    pub struct Running {
        pub stack: Vec<Frame>,
        pub cont: Exp,
    }

    pub struct Halted {
        pub retval: Val
    }

    pub enum ProcState {
        Running(Running),
        Halted(Halted),
    }

    pub struct Process {
        pub trace: Trace,
        pub state: ProcState,
    }

    /// Net surface syntax produces an ast-like structure
    /// to represent an initial net.
    ///
    /// (to do -- surface syntax for all of the above net-internal structure, and then we can write these more general net forms too, e.g., to represent a net that we see live somewhere)
    pub enum Net {
        Running(Sym, Exp),
        Halted(Sym, Val),
        Gather(Box<Net>, Box<Net>),
    }

    /// Normalized net representation for stepping repeatedly.
    pub struct NetNorm {
        // lookup for a symbol involves finding the right trace
        // fragment, for the right process.  The stack is consulted
        // first, from top to bottom, then the non-stack trace here.
        // This is not efficient; it is a Fumola "reference semantics".
        // The purpose is to match the formal semantics as closely
        // as we reasonably can in Rust, and provide some visuals later to
        // permit non-formalists to gain a visual intuition for the formalism.
        pub trace: std::collections::HashMap<Sym, Trace>,
        pub proc: std::collections::HashMap<Sym, Process>,
    }
}
