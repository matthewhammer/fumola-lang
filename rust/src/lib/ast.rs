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
