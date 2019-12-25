use regex::Regex;
use crate::util::{noise_scanner, is_match_command};
use crate::token::{Token, PlainToken, Command as C, Value as V, Operator as O, Type as T};

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
            query_string: input.to_string()
        }
    }

    pub fn exec(self) -> Vec<Token> {
        let fst_stage = SyntaxChecker::new(self.query_string.as_str()).exec();
        println!("{:?}", fst_stage);
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
                if let Some(maybe_operater) = chars.next() {
                    let operator_str = format!("{}{}", char, maybe_operater);
                    match char {
                        '=' => {
                            match maybe_operater {
                                '=' => { tokenlist.push(PlainToken { kind: T::String(operator_str.clone()), str: operator_str }) }
                                _ => { panic!("syntax error") }
                            }
                        }
                        '>' => {
                            match maybe_operater {
                                '=' => { tokenlist.push(PlainToken { kind: T::String(operator_str.clone()), str: operator_str }) }
                                _ => { panic!("syntax error") }
                            }
                        }
                        '<' => {
                            match maybe_operater {
                                '=' => { tokenlist.push(PlainToken { kind: T::String(operator_str.clone()), str: operator_str }) }
                                _ => { panic!("syntax error") }
                            }
                        }
                        '!' => {
                            match maybe_operater {
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
                    token_list.push(Token::Command(C::SELECT));
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

    fn select_tokenize(&mut self, token_list: &mut Vec<Token>) {
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.str.as_str() {
                "FROM" => {
                    token_list.push(Token::Command(C::FROM));
                    self.from_tokenize(token_list);
                }
                "WHERE" => {
                    token_list.push(Token::Command(C::WHERE))
                }
                ";" => {
                    token_list.push(Token::EOF);
                    break;
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
                "BETWEEN" => {
                    token_list.push(Token::Command(C::BETWEEN));
                }
                "LIKE" => { token_list.push(Token::Command(C::LIKE)); }
                "AND" => { token_list.push(Token::Command(C::AND)); }
                _ => {
                    let expression = {
                        let maybe_left = pt;
                        let maybe_operater = self.plain_token_list.pop().unwrap();
                        let maybe_right = self.plain_token_list.pop().unwrap();
                        let operator_kind = match maybe_operater.str.as_str() {
                            "<" => O::LessThan(maybe_left.kind, maybe_right.kind),
                            ">" => O::GreaterThan(maybe_left.kind, maybe_right.kind),
                            "==" => O::Equal(maybe_left.kind, maybe_right.kind),
                            "!=" => O::NotEqual(maybe_left.kind, maybe_right.kind),
                            "<=" => O::EqualOrLessThan(maybe_left.kind, maybe_right.kind),
                            ">=" => O::EqualOrGreaterThan(maybe_left.kind, maybe_right.kind),
                            _ => panic!("syntax error")
                        };
                        V::Condition(operator_kind)
                    };
                    token_list.push(Token::Value(expression));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Tokenize, Lexer};

    #[test]
    fn test1() {
        let mut lexer = Lexer::new("SELECT * FROM table_name;");
        lexer.exec();
    }
}