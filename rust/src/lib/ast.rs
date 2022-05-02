#[derive(Debug)]
pub enum Exp {
    Lambda(Id, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
    Assert(Val, Val),
    Put(Val, Val),
    Get(Val),
    Link(Val),
    Switch(Val, Cases),
    Branches(Branches),
    Project(Box<Exp>, Val),
    Let(Id, Box<Exp>, Box<Exp>),
    LetBx(Id, Box<Exp>, Box<Exp>),
    Ret(Val),

    BinOp(Box<Exp>, BinOp, Box<Exp>),
    Number(i32),
}

#[derive(Debug)]
pub enum Val {
    Bx(Box<Exp>),
    Variant(Box<Val>, Box<Val>),
    Record(Box<RecordVal>),
    RecordExt(Box<Val>, Box<ValField>),
}

pub type Id = String;

pub type RecordVal = Vec<ValField>;

#[derive(Debug)]
pub struct ValField {
    label: Val,
    value: Val,
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
    label: Val,
    pattern: Pat,
}

#[derive(Debug)]
pub struct Branch {
    label: Val,
    body: Box<Exp>,
}

#[derive(Debug)]
pub struct Case {
    label: Val,
    pattern: Pat,
    body: Box<Exp>,
}

#[derive(Debug)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}