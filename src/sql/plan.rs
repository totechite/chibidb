use serde::{Serialize, Deserialize};

trait Command {}

struct Plan {
    command: dyn Command
}

pub struct SELECT {
    pub fields: Vec<Field>,
    pub FROM: Option<Vec<Table>>,
    pub WHERE: Option<SearchCondition>,
}

pub struct CREATE {
    pub TABLE: Option<(String, Vec<FieldDefinition>)>
}

pub struct INSERT {
    pub INTO: Option<(String, Option<Vec<String>>)>,
    pub VALUES: Vec<Vec<String>>,
}

pub enum Field {
    All,
    Plain { name: String, table_name: Option<String>, AS: Option<String> },
    Calc { expr: Box<Expression>, name: String, table_name: Option<String>, AS: Option<String> },
}

pub struct Table {
    pub name: String,
    pub scheme: Option<String>,
    pub AS: Option<String>,
}

pub enum Expression {
    //    User defined value.

    Var(String),
    Number(u32),

    //    Operator
    //    +
    Add(Box<Expression>, Box<Expression>),
    //    -
    Sub(Box<Expression>, Box<Expression>),
    //    *
    Mul(Box<Expression>, Box<Expression>),
    //    /
    Div(Box<Expression>, Box<Expression>),
}

pub enum SearchCondition {
    Equal(Value, Value),
    NotEqual(Value, Value),
    GreaterThan(Value, Value),
    LessThan(Value, Value),
    EqualOrGreaterThan(Value, Value),
    EqualOrLessThan(Value, Value),
    AND(Box<SearchCondition>, Box<SearchCondition>),
    OR(Box<SearchCondition>, Box<SearchCondition>),
    NOT(Box<SearchCondition>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(isize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    pub T: Type,
    constraint: Option<Vec<Constraint>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constraint {
    PRIMARY_KEY(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Type {
    integer,
    varchar(u32),
}


