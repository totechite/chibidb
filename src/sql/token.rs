use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct PlainToken {
    pub kind: Type,
    pub str: String,
}

pub enum PTKind {
    String,
    Number,
}


///////////DON'T DELETE//////////

pub trait TokenKind {
    fn to_string(self: Box<Self>) -> String;
}

impl TokenKind for Command {
    fn to_string(self: Box<Self>) -> String {
        format!("{:?}", self)
    }
}

impl TokenKind for SELECT {
    fn to_string(self: Box<Self>) -> String {
        format!("{:?}", self)
    }
}

impl TokenKind for CREATE {
    fn to_string(self: Box<Self>) -> String {
        format!("{:?}", self)
    }
}

impl TokenKind for Condition {
    fn to_string(self: Box<Self>) -> String {
        format!("{:?}", self)
    }
}

impl TokenKind for Operator {
    fn to_string(self: Box<Self>) -> String {
        format!("{:?}", self)
    }
}
/////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum Token {
    HeadOfQuery,
    Command(Command),
    Value(Value),
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    SELECT,
    FROM,
    WHERE,
    AND,
    BETWEEN,
    LIKE,
    IN,
    UPDATE,
    DELETE,
    INSERT,
    CREATE(CREATE),

}

#[derive(Debug, PartialEq)]
pub enum SELECT {
    FROM(Option<Condition>),
}

pub enum INSERT {}

pub enum DELETE {}

pub enum UPDATE {}

#[derive(Debug, PartialEq)]
pub enum CREATE {
    DATABASE,
    TABLE,
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    WHERE(Box<Operator>),
    BETWEEN,
    AND,
    OR,
    NOT,
}


#[derive(Debug, PartialEq)]
pub enum Type {
    Number(isize),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Column(String),
    Table(String),
    Condition(Operator),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal(Type, Type),
    NotEqual(Type, Type),
    GreaterThan(Type, Type),
    LessThan(Type, Type),
    EqualOrGreaterThan(Type, Type),
    EqualOrLessThan(Type, Type),
    AND(Box<Operator>, Box<Operator>),
    OR(Box<Operator>, Box<Operator>),
    NOT(Box<Operator>),
}
