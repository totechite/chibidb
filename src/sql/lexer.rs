use regex::Regex;
use std::string::ToString;
use std::iter::FromIterator;
use crate::sql::token::{Token, Operator};

pub fn noise_scanner(c: char) -> bool {
    (' ' == c) || ('\n' == c) || (',' == c)
}

#[derive(Debug)]
pub struct Lexer {
    query_string: String
}

#[derive(Debug)]
struct SyntaxChecker
{
    query_string: String,
    token_list: Vec<String>,
}

#[derive(Debug)]
struct Tokenizer {
    plain_token_list: Vec<String>
}

impl Lexer {
//    pub fn exec(query_string: impl ToString) -> Vec<Token> {
//        let fst_stage = SyntaxChecker::exec(query_string.to_string());
//        Tokenizer::new(fst_stage).exec()
//    }
}

impl SyntaxChecker {
    fn new(qs: &str) -> Self {
        let qs = String::from(qs);
        Self {
            query_string: qs.clone(),
            token_list: vec![],
        }
    }

    fn exec(query_string: String) -> Vec<String> {
        let mut char_stack: Option<Vec<char>> = None;
        let mut op_stack: Option<Vec<char>> = None;
        let mut token_list: Vec<String> = vec![];
        let mut chars = query_string.chars();

        while let Some(char) = chars.next() {
            // consume end
            if ';' == char {
                if let Some(chars) = char_stack.take() {
                    let string = String::from_iter(chars);
                    token_list.push(string);
                }
                break;
            };
            // consume whitespace
            if ' ' == char {
                if let Some(chars) = char_stack.take() {
                    let string = String::from_iter(chars);
                    token_list.push(string);
                }
                continue;
            };
            // consume comma
            if ',' == char {
                if let Some(chars) = char_stack.take() {
                    let string = String::from_iter(chars);
                    token_list.push(string);
                }
                token_list.push(char.to_string());
                continue;
            };
            // try operator
            match char {
                '<' | '>' | '=' | '!' => {
                    if let Some(chars) = char_stack.take() {
                        let string = String::from_iter(chars);
                        token_list.push(string);
                    }
                    if let Some(c) = chars.next() {
                        match c {
                            '=' => {
                                let operator = format!("{}{}", char, c);
                                token_list.push(operator);
                            }
                            _ => {
                                token_list.push(char.to_string());
                                char_stack = if let Some(mut str) = char_stack {
                                    str.push(c);
                                    Some(str)
                                } else { Some(vec![c]) };
                            }
                        }
                    }
                    continue;
                }
                _ => {}
            }
            // try paren
            match char {
                '(' => {
                    if let Some(str) = char_stack.take() {
                        let str = String::from_iter(str);
                        token_list.push(str);
                    };
                    token_list.push(char.to_string());
                    continue;
                }
                ')' => {
                    if let Some(str) = char_stack.take() {
                        let str = String::from_iter(str);
                        token_list.push(str);
                    };
                    token_list.push(char.to_string());
                    continue;
                }
                _ => {}
            };

            // For making token
            char_stack = if let Some(mut str) = char_stack {
                str.push(char);
                Some(str)
            } else { Some(vec![char]) };
        }
        token_list.reverse();
        return token_list;
    }
}

//impl Tokenizer {
//    fn new(tl: Vec<String>) -> Self {
//        Tokenizer {
//            plain_token_list: tl
//        }
//    }
//
//    fn exec(&mut self) -> Vec<Token> {
//        let mut token_list = vec![];
//        while let Some(pt) = self.plain_token_list.pop() {
//            match pt.as_str() {
//                "SELECT" => {
//                    token_list.push(Token::SELECT);
//                    self.select_tokenize(&mut token_list);
//                }
//                "INSERT" => {
//                    token_list.push(Token::INSERT);
////                  self.insert_tokenize();
//                }
//                "CREATE" => {
//                    self.create_tokenize(&mut token_list);
//                }
//                "UPDATE" => {}
//                "DELETE" => {}
//                _ => { panic!("contained undefined keyword"); }
//            }
//        }
//
//        return token_list;
//    }
//}
//
//impl Tokenizer {
////sub tokenizer
//
//    fn create_tokenize(&mut self, token_list: &mut Vec<Token>) {
//        while let Some(pt) = self.plain_token_list.pop() {
//            match pt.as_str() {
//                "TABLE" => {
//                    token_list.push(Token::CREATE);
//                    self.create_table_tokenize(token_list);
//                }
//                "DATABASE" => { token_list.push(Token::CREATE) }
//                _ => {}
//            }
//        }
//    }
//
//    fn create_table_tokenize(&mut self, token_list: &mut Vec<Token>) {
//        token_list.push(Token::TableName(self.plain_token_list.pop().unwrap()));
//        while let Some(pt) = self.plain_token_list.pop() {
//            match pt.as_str() {
//                _ => {}
//            }
//        }
//    }
//
//    fn select_tokenize(&mut self, token_list: &mut Vec<Token>) {
//        while let Some(pt) = self.plain_token_list.pop() {
//            match pt.as_str() {
//                "FROM" => {
//                    token_list.push(Token::FROM);
//                    self.from_tokenize(token_list);
//                }
//                _ => {
//                    token_list.push(Token::ColumnName(pt));
//                }
//            }
//        }
//    }
//
//    fn from_tokenize(&mut self, token_list: &mut Vec<Token>) {
//        while let Some(pt) = self.plain_token_list.pop() {
//            match pt.as_str() {
//                ";" => {
//                    token_list.push(Token::EOF);
//                    break;
//                }
//                "WHERE" => {
//                    token_list.push(Token::WHERE);
//                    self.where_tokenize(token_list);
//                }
//                _ => {
//                    token_list.push(Token::TableName(pt));
//                }
//            }
//        }
//    }
//
//    fn where_tokenize(&mut self, token_list: &mut Vec<Token>) {
//        while let Some(pt) = self.plain_token_list.pop() {
//            match pt.as_str() {
//                "NOT" => {
//                    let expression = {
//                        let operator_kind = self.make_operator(Some(pt), None, None);
//                        Token::Condition(Operator::NOT(Box::new(operator_kind)))
//                    };
//                    token_list.push(expression);
//                }
//                "AND" => {
//                    let prev_token = match token_list.pop().unwrap() {
//                        Token::Condition(operator) => operator,
//                        _ => panic!("error")
//                    };
//                    token_list.push(Token::Condition(Operator::AND(Box::new(prev_token), Box::new(self.make_operator(None, None, None)))));
//                }
//                "OR" => {
//                    let prev_token = match token_list.pop().unwrap() {
//                        Token::Condition(operator) => operator,
//                        _ => panic!("error")
//                    };
//                    token_list.push(Token::Condition(Operator::OR(Box::new(prev_token), Box::new(self.make_operator(None, None, None)))));
//                }
//                _ => {
//                    let expression = {
//                        let operator_kind = self.make_operator(Some(pt), None, None);
//                        Token::Condition(operator_kind)
//                    };
//                    token_list.push(expression);
//                }
//            }
//        }
//    }
//}
//
//impl Tokenizer {
//    // Util
//    fn make_operator(&mut self, left: Option<String>, operator: Option<String>, right: Option<String>) -> Operator {
//        let maybe_left = if let Some(token) = left { token } else { self.plain_token_list.pop().unwrap() };
//        let maybe_operator = if let Some(token) = operator { token } else { self.plain_token_list.pop().unwrap() };
//        let maybe_right = if let Some(token) = right { token } else { self.plain_token_list.pop().unwrap() };
//        let operator_kind = match maybe_operator.as_str() {
//            "<" => Operator::LessThan(maybe_left.kind, maybe_right.kind),
//            ">" => Operator::GreaterThan(maybe_left.kind, maybe_right.kind),
//            "==" => Operator::Equal(maybe_left.kind, maybe_right.kind),
//            "!=" => Operator::NotEqual(maybe_left.kind, maybe_right.kind),
//            "<=" => Operator::EqualOrLessThan(maybe_left.kind, maybe_right.kind),
//            ">=" => Operator::EqualOrGreaterThan(maybe_left.kind, maybe_right.kind),
//            _ => panic!("syntax error")
//        };
//        return operator_kind;
//    }
//}

#[cfg(test)]
mod test {
    use crate::sql::lexer::{Lexer, SyntaxChecker};

    #[test]
    fn select_from() {}

    #[test]
    fn create_table() {}

    #[test]
    fn insert() {}
}

