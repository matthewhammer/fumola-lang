#[derive(Debug)]
pub enum Exp {
    Nest(Val, Box<Exp>),
    Put(Val, Val),
    Get(Val),
    Link(Val),
    Assert(Val, Val),
    Lambda(Id, Box<Exp>),
    App(Box<Exp>, Val),
    Let(Pat, Box<Exp>, Box<Exp>),
    Ret(Val),
    Switch(Val, Cases),
    Branches(Branches),
    Project(Box<Exp>, Val),
    LetBx(Id, Box<Exp>, Box<Exp>),
    Extract(Val),
    BinOp(Box<Exp>, BinOp, Box<Exp>),
    Number(i32),
}

#[derive(Debug)]
pub enum Val {
    Num(i32),
    Variant(Box<Val>, Box<Val>),
    Record(Box<RecordVal>),
    RecordExt(Box<Val>, Box<ValField>),
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
pub enum Pat {
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
