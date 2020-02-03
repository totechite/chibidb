use regex::Regex;
use crate::util::noise_scanner;
use crate::token::{Token, PlainToken, Command as C, Value as V, Operator as O, Type as T, TokenKind, CREATE};

#[derive(Debug)]
pub struct Lexer {
    query_string: String
}

#[derive(Debug)]
struct SyntaxChecker
{
    query_string: String,
}

#[derive(Debug)]
struct Tokenizer {
    plain_token_list: Vec<PlainToken>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            query_string: input.to_String()
        }
    }

    pub fn exec(self) -> Vec<Token> {
        let fst_stage = SyntaxChecker::new(self.query_string.as_str()).exec();
        Tokenizer::new(fst_stage).exec()
    }
}

impl SyntaxChecker {
    fn new(qs: &str) -> Self {
        let qs = String::from(qs);
        Self {
            query_string: qs.clone(),
        }
    }

    fn exec(&mut self) -> Vec<PlainToken> {
        let mut token_str = String::new();
        let mut tokenlist: Vec<PlainToken> = Vec::new();
        let mut chars = self.query_string.chars();
        while let Some(char) = chars.next() {
            if ';' == char {
                tokenlist.push(
                    if let Ok(number) = token_str.parse() {
                        PlainToken { kind: T::Number(number), str: token_str }
                    } else {
                        PlainToken { kind: T::String(token_str.clone()), str: token_str }
                    }
                );
                break;
            }
            if noise_scanner(char) {
                if String::new() == token_str { continue; }
                tokenlist.push(
                    if let Ok(number) = token_str.parse() {
                        PlainToken { kind: T::Number(number), str: token_str }
                    } else {
                        PlainToken { kind: T::String(token_str.clone()), str: token_str }
                    }
                );
                token_str = String::new();
                continue;
            }
            if Regex::new("!|<|>|=").unwrap().is_match(char.to_string().as_str()) {
                tokenlist.push(PlainToken { kind: T::String(token_str.clone()), str: token_str });
                token_str = String::new();
                if let Some(maybe_operator) = chars.next() {
                    let operator_str = format!("{}{}", char, maybe_operator);
                    match char {
                        '=' => {
                            match maybe_operator {
                                '=' => { tokenlist.push(PlainToken { kind: T::String(operator_str.clone()), str: operator_str }) }
                                _ => { panic!("syntax error") }
                            }
                        }
                        '>' => {
                            match maybe_operator {
                                '=' => { tokenlist.push(PlainToken { kind: T::String(operator_str.clone()), str: operator_str }) }
                                _ => { panic!("syntax error") }
                            }
                        }
                        '<' => {
                            match maybe_operator {
                                '=' => { tokenlist.push(PlainToken { kind: T::String(operator_str.clone()), str: operator_str }) }
                                _ => { panic!("syntax error") }
                            }
                        }
                        '!' => {
                            match maybe_operator {
                                '=' => { tokenlist.push(PlainToken { kind: T::String(operator_str.clone()), str: operator_str }) }
                                _ => { panic!("syntax error") }
                            }
                        }
                        _ => {
                            panic!("syntax error")
                        }
                    }
                } else {
                    panic!("syntax error")
                }
                continue;
            };
            token_str += char.to_string().as_str();
        }
        tokenlist.reverse();
        return tokenlist;
    }
}

impl Tokenizer {
    fn new(tl: Vec<PlainToken>) -> Self {
        Tokenizer {
            plain_token_list: tl
        }
    }

    fn exec(&mut self) -> Vec<Token> {
        let mut token_list = Vec::new();
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.str.as_str() {
                "SELECT" => {
                    self.select_tokenize(&mut token_list);
                }
                "INSERT" => {
                    token_list.push(Token::Command(C::INSERT))
//                  self.insert_tokenize();
                }
                "CREATE" => {}
                "UPDATE" => {}
                "DELETE" => {}
                ";" => {
                    token_list.push(Token::EOF);
                    break;
                }
                _ => { panic!("contained undefined keyword"); }
            }
        }
        return token_list;
    }
}

impl Tokenizer {
    //sub tokenizer

    fn create_tokenize(&mut self, token_list: &mut Vec<Token>) {
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.str.as_str() {
                "TABLE" => { token_list.push(Token::Command(C::CREATE(CREATE::TABLE))) }
                "DATABASE" => { token_list.push(Token::Command(C::CREATE(CREATE::TABLE))) }
                _ => {}
            }
        }
    }

    fn select_tokenize(&mut self, token_list: &mut Vec<Token>) {
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.str.as_str() {
                "FROM" => {
                    token_list.push(Token::Command(C::FROM));
                    self.from_tokenize(token_list);
                }
                _ => {
                    token_list.push(Token::Value(V::Column(pt.str)));
                }
            }
        }
    }

    fn from_tokenize(&mut self, token_list: &mut Vec<Token>) {
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.str.as_str() {
                ";" => {
                    token_list.push(Token::EOF);
                    break;
                }
                "WHERE" => {
                    token_list.push(Token::Command(C::WHERE));
                    self.where_tokenize(token_list);
                }
                _ => {
                    token_list.push(Token::Value(V::Table(pt.str)));
                }
            }
        }
    }

    fn where_tokenize(&mut self, token_list: &mut Vec<Token>) {
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.str.as_str() {
                "NOT" => {
                    let expression = {
                        let operator_kind = self.make_operator(Some(pt), None, None);
                        V::Condition(O::NOT(Box::new(operator_kind)))
                    };
                    token_list.push(Token::Value(expression));
                }
                "AND" => {
                    let prev_token = match token_list.pop().unwrap() {
                        Token::Value(V::Condition(operator)) => operator,
                        _ => panic!("error")
                    };
                    token_list.push(Token::Value(V::Condition(O::AND(Box::new(prev_token), Box::new(self.make_operator(None, None, None))))));
                }
                "OR" => {
                    let prev_token = match token_list.pop().unwrap() {
                        Token::Value(V::Condition(operator)) => operator,
                        _ => panic!("error")
                    };
                    token_list.push(Token::Value(V::Condition(O::OR(Box::new(prev_token), Box::new(self.make_operator(None, None, None))))));
                }
                _ => {
                    let expression = {
                        let operator_kind = self.make_operator(Some(pt), None, None);
                        V::Condition(operator_kind)
                    };
                    token_list.push(Token::Value(expression));
                }
            }
        }
    }
}

impl Tokenizer {
    // Util
    fn make_operator(&mut self, left: Option<PlainToken>, operator: Option<PlainToken>, right: Option<PlainToken>) -> O {
        let maybe_left = if let Some(token) = left { token } else { self.plain_token_list.pop().unwrap() };
        let maybe_operator = if let Some(token) = operator { token } else { self.plain_token_list.pop().unwrap() };
        let maybe_right = if let Some(token) = right { token } else { self.plain_token_list.pop().unwrap() };
        let operator_kind = match maybe_operator.str.as_str() {
            "<" => O::LessThan(maybe_left.kind, maybe_right.kind),
            ">" => O::GreaterThan(maybe_left.kind, maybe_right.kind),
            "==" => O::Equal(maybe_left.kind, maybe_right.kind),
            "!=" => O::NotEqual(maybe_left.kind, maybe_right.kind),
            "<=" => O::EqualOrLessThan(maybe_left.kind, maybe_right.kind),
            ">=" => O::EqualOrGreaterThan(maybe_left.kind, maybe_right.kind),
            _ => panic!("syntax error")
        };
        return operator_kind;
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::{Token, Command as C, Value as V, Type as T, Operator as O};

    #[test]
    fn operator() {
        let maybe_output: Vec<Token> = vec![
            Token::Command(
                C::SELECT,
            ),
            Token::Value(
                V::Column(
                    "*".to_string(),
                ),
            ),
            Token::Command(
                C::FROM,
            ),
            Token::Value(
                V::Table(
                    "table".to_string(),
                ),
            ),
            Token::Command(
                C::WHERE,
            ),
            Token::Value(
                V::Condition(
                    O::Equal(
                        T::String(
                            "column".to_string(),
                        ),
                        T::String(
                            "mohumohu".to_string(),
                        ),
                    ),
                ),
            ),
        ];
        let mut lexer = Lexer::new("SELECT * FROM table WHERE column==mohumohu;");
        assert_eq!(maybe_output, lexer.exec());
    }
}