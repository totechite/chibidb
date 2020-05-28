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
    AS,

    //    DATA TYPE
    integer,
    text,

    //    User defined values

    Phrase(String),
    Computation(ArithmeticOperator),
    Condition(LogicalOperator),

    //    meta
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum ValueOfLogicalOperator {
    String(String),
    Number(isize),
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    Equal(ValueOfLogicalOperator, ValueOfLogicalOperator),
    NotEqual(ValueOfLogicalOperator, ValueOfLogicalOperator),
    GreaterThan(ValueOfLogicalOperator, ValueOfLogicalOperator),
    LessThan(ValueOfLogicalOperator, ValueOfLogicalOperator),
    EqualOrGreaterThan(ValueOfLogicalOperator, ValueOfLogicalOperator),
    EqualOrLessThan(ValueOfLogicalOperator, ValueOfLogicalOperator),
    AND(Box<LogicalOperator>, Box<LogicalOperator>),
    OR(Box<LogicalOperator>, Box<LogicalOperator>),
    NOT(Box<LogicalOperator>),
}

#[derive(Debug, PartialEq)]
pub enum ValueOfArithmeticOperator {
    String(String),
    Number(isize),
}

#[derive(Debug, PartialEq)]
pub enum ArithmeticOperator {
    Add(ValueOfArithmeticOperator, ValueOfArithmeticOperator),
    Sub(ValueOfArithmeticOperator, ValueOfArithmeticOperator),
    Div(ValueOfArithmeticOperator, ValueOfArithmeticOperator),
    Mul(ValueOfArithmeticOperator, ValueOfArithmeticOperator),
    Mod(ValueOfArithmeticOperator, ValueOfArithmeticOperator),

}
