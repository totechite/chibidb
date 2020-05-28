use crate::sql::plan::Expression;
use crate::sql::token::{ArithmeticOperator};

pub trait Command_Declare {}

#[derive(Debug, Default)]
pub struct SELECT_FROM_Declare {
    pub text: String,
    pub select_term: SELECT_Declare,
    pub from_term: FROM_Declare,
    pub join_term: Option<JOIN_Declare>,
    pub where_term: Option<WHERE_Declare>,
    pub order_by: Option<ORDER_BY_Declare>,
    pub group_by: Option<GROUP_BY_Declare>,
}

impl SELECT_FROM_Declare {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct SELECT_Declare{
    pub text: String,
    pub contents: Vec<Column_Term>,
}

#[derive(Debug)]
pub struct Column_Term {
    pub text: String,
    pub context: Column_Context,
    pub alias: Option<String>,
}

#[derive(Debug)]
pub enum Column_Context {
    Literal { column_name: String, attribute: Option<String> },
    SubQuery(Box<SELECT_FROM_Declare>),
    Computation(ComputationTerm),
}

#[derive(Debug, Default)]
pub struct FROM_Declare {
    pub text: String,
    pub contents: Vec<Table_Term>,
}

#[derive(Debug)]
pub struct Table_Term {
    pub text: String,
    pub context: Table_Context,
    pub alias: Option<String>,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(usize),
}

#[derive(Debug, Clone)]
pub enum OperatorTerm<O> where O: Clone {
    String(String),
    Number(usize),
    Boolean(bool),
    Operator(O),
}

#[derive(Debug)]
pub enum Table_Context {
    Literal(String),
    SubQuery(Box<SELECT_FROM_Declare>),
}

#[derive(Debug, )]
pub struct WHERE_Declare {
    pub text: String,
    pub content: Box<ConditionTerm>,
}

#[derive(Debug, Default)]
pub struct JOIN_Declare {
    pub text: String,
    pub table: String,

}

#[derive(Debug, Clone)]
pub struct ConditionTerm {
    pub text: String,
    pub operator: LogicalOperator,
    pub left_v: OperatorTerm<Box<ConditionTerm>>,
    pub right_v: OperatorTerm<Box<ConditionTerm>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    EqualOrGreaterThan,
    EqualOrLessThan,
    AND,
    OR,
    NOT,
}

#[derive(Debug)]
pub struct ComputationTerm {
    text: String,
    operator: ArithmeticOperator,
    left_v: OperatorTerm<Box<ConditionTerm>>,
    right_v: OperatorTerm<Box<ConditionTerm>>,
}

#[derive(Debug, Default)]
pub struct ORDER_BY_Declare {}

#[derive(Debug, Default)]
pub struct GROUP_BY_Declare {}
