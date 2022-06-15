pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    Nest(Val, Box<Exp>),
    Spawn(Val, Box<Exp>),
    Put(Val, Val),
    Get(Val),
    Link(Val),
    AssertEq(Val, bool, Val),
    Lambda(Pat, Box<Exp>),
    App(Box<Exp>, Val),
    Let(Pat, Box<Exp>, Box<Exp>),
    Ret(Val),
    /// Ret_ is like Ret, but without surface syntax, and only used internally.
    Ret_(Val),
    Switch(Val, Cases),
    Branches(Branches),
    Project(Box<Exp>, Val),
    LetBx(Pat, Box<Exp>, Box<Exp>),
    Extract(Val),
    Hole,
}

pub type BxesEnv = std::collections::HashMap<Id, BxVal>;

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BxVal {
    pub bxes: BxesEnv,
    pub name: Option<Id>,
    pub code: Exp,
}

pub type Id = String;

pub type RecordVal = Vec<ValField>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValField {
    pub label: Val,
    pub value: Val,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branches {
    Empty,
    Gather(Box<Branches>, Box<Branches>),
    Branch(Branch),
}

pub type FieldsPat = Vec<FieldPat>;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Nest: Special binary case arising from putting within named nests.
    Nest(Box<Sym>, Box<Sym>),
    Tri(Box<Sym>, Box<Sym>, Box<Sym>),
    Dash,
    Under,
    Dot,
    Tick,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pat {
    Ignore,
    Var(Id),
    Fields(FieldsPat),
    Case(Box<FieldPat>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPat {
    pub label: Val,
    pub pattern: Pat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    pub label: Val,
    pub body: Box<Exp>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub label: Val,
    pub pattern: Pat,
    pub body: Box<Exp>,
}

/// Syntactic forms for representing the intermediate state of dynamic
/// evaluation.
pub mod step {
    use super::{BxesEnv, Exp, Id, Pat, Sym, Val};

    /// Net surface syntax produces an ast-like structure
    /// to represent an initial net.
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
        Seq(Vec<Trace>),
        Nest(Sym, Vec<Trace>),
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

    /// Signal.
    /// Not an error, but not ordinary stepping either.
    #[derive(Debug, Clone)]
    pub enum Signal {
        /// Process has successfully produced a final return value.
        Halt(Val),
        /// Process is waiting to link to a symbol not yet in the store,
        LinkWaitPtr(Sym),
        /// Process is waiting to link to another process to halt.
        LinkWaitHalt(Sym),
        /// Process is spawning another process with given name, env and body.
        Spawn(Sym, Env, Exp),
    }

    /// Fumola implementation errors.
    #[derive(Debug, Clone)]
    pub enum InternalError {
        /// Logically-impossible error.
        /// (but Rust type system cannot disprove.)
        Impossible,

        /// Attempt to step Hole.  Internal logical error.
        /// Special kind of Impossible Error.
        Hole,
    }

    #[derive(Debug, Clone)]
    pub enum Error {
        /// Signal.
        /// Not an error, but not ordinary stepping either.
        Signal(Signal),

        /// Implementation-caused errors.
        Internal(InternalError),

        //
        // # Fumola program / proogrammer errors,
        //
        /// No processes to consider stepping.
        NoProcs,

        /// Pattern matching error.
        Pattern(PatternError),

        /// Value closing error.
        Value(ValueError),

        /// Code extraction error.
        Extract(ExtractError),

        /// Switch stepping error.
        Switch(SwitchError),

        /// Switch stepping error.
        Project(ProjectError),

        /// No stepping rule applies.
        /// Dynamically-determined type mismatch.
        NoStep,

        /// Value is not a symbol (invalid put, nest, spawn).
        NotASymbol(Val),

        /// Value is not a pointer (invalid get).
        NotAPointer(Val),

        /// Invalid process symbol. Not yet in store.
        InvalidProc(Sym),

        /// Value is not a link target (not a symbol, not a process identifer).
        NotLinkTarget(Val),

        /// Symbol is not defined in the store.
        Undefined(Sym),

        /// Duplicate process name.
        /// It is an error to name a spawned process a non-uniquely.
        Duplicate(Sym),

        /// Assertion that v1 and v2 are equal (or not) equal failed.
        AssertionFailure(Val, bool, Val),
    }

    #[derive(Debug, Clone)]
    pub enum SwitchError {
        NotVariant(Val),
        MissingCase(Sym),
    }

    #[derive(Debug, Clone)]
    pub enum ProjectError {
        MissingBranch(Sym),
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
        FieldNotFound(Val),
    }

    #[derive(Debug, Clone)]
    pub enum ExtractError {
        Undefined(Id),
    }

    #[derive(Debug, Clone)]
    pub enum Proc {
        Spawn(Exp),
        Running(Running),
        WaitingForPtr(Running, Sym),
        WaitingForHalt(Running, Sym),
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
        pub trace: Vec<Trace>,
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
        Project(Val),
        Nest(Sym),
    }
}
