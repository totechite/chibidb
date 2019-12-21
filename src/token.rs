#[derive(Debug)]
pub struct PlainToken(pub String);

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
    UPDATE,
    DELETE,
    INSERT,
}

#[derive(Debug)]
pub enum Type {
    Number,
    String,
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
    NotEqual,
    GreaterThan,
    LessThan,
    BETWEEN,
    LIKE,
    IN,
}
