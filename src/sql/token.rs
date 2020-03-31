use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Token {
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
    CREATE,
    TABLE,
    DATABASE,
    PRIMARY,
    KEY,

    //    DATA TYPE
    integer,
    varchar,

    //    User defined values
    ColumnName(String),
    TableName(String),
    Condition(Operator),

    //    meta
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(isize),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal(Value, Value),
    NotEqual(Value, Value),
    GreaterThan(Value, Value),
    LessThan(Value, Value),
    EqualOrGreaterThan(Value, Value),
    EqualOrLessThan(Value, Value),
    AND(Box<Operator>, Box<Operator>),
    OR(Box<Operator>, Box<Operator>),
    NOT(Box<Operator>),
}
