#[derive(Debug)]
pub struct PlainToken {
    pub kind: Type,
    pub str: String,
}

pub enum PTKind {
    String,
    Number,
}

#[derive(Debug)]
pub enum Token {
    HeadOfQuery,
    Command(Command),
    Value(Value),
    EOF,
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum Type {
    Number(isize),
    String(String),
}

#[derive(Debug)]
pub enum Value {
    Column(String),
    Table(String),
    Condition(Operator),
}

#[derive(Debug)]
pub enum Operator {
    Equal(Type, Type),
    NotEqual(Type, Type),
    GreaterThan(Type, Type),
    LessThan(Type, Type),
    EqualOrGreaterThan(Type, Type),
    EqualOrLessThan(Type, Type),

}
