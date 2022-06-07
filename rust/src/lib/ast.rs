pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone)]
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
    LetBx(Pat, Box<Exp>, Box<Exp>),
    Extract(Val),
    Hole,
}

pub type BxesEnv = std::collections::HashMap<Id, BxVal>;

#[derive(Debug, Clone)]
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
    Record(RecordVal),
    RecordExt(Box<Val>, Box<ValField>),
    Bx(Box<BxVal>),
}

/// "Code box" as in https://arxiv.org/abs/1703.01288
#[derive(Debug, Clone)]
pub struct BxVal {
    pub bxes: BxesEnv,
    pub name: Option<Id>,
    pub code: Exp,
}

pub type Id = String;

pub type RecordVal = Vec<ValField>;

#[derive(Debug, Clone)]
pub struct ValField {
    pub label: Val,
    pub value: Val,
}

#[derive(Debug, Clone)]
pub enum Branches {
    Empty,
    Gather(Box<Branches>, Box<Branches>),
    Branch(Branch),
}

pub type FieldsPat = Vec<FieldPat>;

#[derive(Debug, Clone)]
pub enum Cases {
    Empty,
    Gather(Box<Cases>, Box<Cases>),
    Case(Case),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sym {
    None,
    Num(i32),
    Id(Id),
    Bin(Box<Sym>, Box<Sym>),
    /// Special binary case arising from putting within named nests
    Nest(Box<Sym>, Box<Sym>),
    Tri(Box<Sym>, Box<Sym>, Box<Sym>),
    Dash,
    Under,
    Dot,
    Tick,
}

#[derive(Debug, Clone)]
pub enum Pat {
    Ignore,
    Var(Id),
    Fields(Box<FieldsPat>),
    Case(Box<FieldPat>),
}

#[derive(Debug, Clone)]
pub struct FieldPat {
    pub label: Val,
    pub pattern: Pat,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub label: Val,
    pub body: Box<Exp>,
}

#[derive(Debug, Clone)]
pub struct Case {
    pub label: Val,
    pub pattern: Pat,
    pub body: Box<Exp>,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

/// Syntactic forms for representing the intermediate state of dynamic
/// evaluation.
pub mod step {
    use super::{BxVal, BxesEnv, Exp, Id, Pat, Sym, Val};

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

    /// System representation for stepping repeatedly.
    /// Compared with TraceNet, uses Procs in place of Net.
    #[derive(Debug)]
    pub struct System {
        pub store: Store,
        pub trace: Vec<Trace>,
        pub procs: Procs,
    }

    #[derive(Debug, Clone)]
    pub enum Trace {
        Proc(Sym, Box<Trace>),
        Seq(Vec<Trace>),
        Nest(Sym, Box<Trace>),
        Ret(Val),
        Put(Sym, Val),
        Get(Sym, Val),
        Link(Val, Val),
    }

    pub type Procs = std::collections::HashMap<Sym, Proc>;

    pub type Store = std::collections::HashMap<Sym, Val>;

    pub type Stack = Vec<Frame>;

    pub type ValsEnv = std::collections::HashMap<Id, Val>;

    #[derive(Debug, Clone)]
    pub struct Env {
        pub vals: ValsEnv,
        pub bxes: BxesEnv,
    }

    impl std::convert::From<ValueError> for Error {
        fn from(e: ValueError) -> Self {
            Error::Value(e)
        }
    }

    impl std::convert::From<PatternError> for Error {
        fn from(e: PatternError) -> Self {
            Error::Pattern(e)
        }
    }

    #[derive(Debug, Clone)]
    pub enum Error {
        /// Attempt to step Hole.  Internal logical error.
        Hole,

        /// No processes to consider stepping.
        NoProcs,

        /// Signal (successful) halting state, with value.
        /// Not an error, but not ordinary stepping either.
        SignalHalt(Val),

        /// Logically-impossible error.
        /// (but Rust type system cannot disprove.)
        Impossible,

        /// Pattern matching error.
        Pattern(PatternError),

        /// Value closing error.
        Value(ValueError),

        /// Code extraction error.
        Extract(ExtractError),

        /// No stepping rule applies.
        /// Dynamically-determined type mismatch.
        NoStep,

        /// Value is not a symbol.
        NotASymbol(Val),

        /// Symbol is not defined in the store.
        Undefined(Sym),

        /// Duplicate process name.
        /// It is an error to name a spawned process a non-uniquely.
        Duplicate(Sym),
    }

    #[derive(Debug, Clone)]
    pub enum ValueError {
        CallByValue,
        Undefined(Id),
    }

    #[derive(Debug, Clone)]
    pub enum PatternError {
        NotVariant,
        NotRecord,
    }

    #[derive(Debug, Clone)]
    pub enum ExtractError {
        Undefined(Id),
    }

    #[derive(Debug, Clone)]
    pub enum Proc {
        Spawn(Exp),
        Running(Running),
        Error(Running, Error),
        Halted(Halted),
    }

    #[derive(Debug, Clone)]
    pub struct Running {
        pub env: Env,
        pub stack: Stack,
        pub cont: Exp,
        pub trace: Vec<Trace>,
    }

    #[derive(Debug, Clone)]
    pub struct Halted {
        pub retval: Val,
    }

    #[derive(Debug, Clone)]
    pub struct Frame {
        pub cont: FrameCont,
        pub trace: Vec<Trace>,
    }

    #[derive(Debug, Clone)]
    pub enum FrameCont {
        LetBx(Env, Pat, Exp),
        Let(Env, Pat, Exp),
        App(Val),
        Nest(Sym),
    }
}
