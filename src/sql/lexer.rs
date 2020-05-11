use regex::Regex;
use std::string::ToString;
use std::iter::FromIterator;
use crate::sql::token::{Token, Operator};
use crate::sql::plan::{SELECT, Field, Table, SearchCondition, CREATE, FieldDefinition, Type};
use crate::sql::token::Token::Condition;
use protobuf::well_known_types::Field_Kind::TYPE_BOOL;

pub fn noise_scanner(c: char) -> bool {
    (' ' == c) || ('\n' == c) || (',' == c)
}

#[derive(Debug)]
pub struct Lexer {
    query_string: String
}

#[derive(Debug)]
struct Tokenizer
{
    query_string: String,
    token_list: Vec<String>,
}

impl Lexer {
//    pub fn exec(query_string: impl ToString) -> Vec<Token> {
//        let fst_stage = SyntaxChecker::exec(query_string.to_string());
//        Tokenizer::new(fst_stage).exec()
//    }
}

impl Tokenizer {
    fn new(qs: &str) -> Self {
        let qs = String::from(qs);
        Self {
            query_string: qs.clone(),
            token_list: vec![],
        }
    }

    fn exec(query_string: String) -> Vec<String> {
        let mut char_stack: Option<Vec<char>> = None;
        let mut token_list: Vec<String> = vec![];
        let mut chars = query_string.chars();

        while let Some(char) = chars.next() {
            let mut dump_to_list = || {
                if let Some(chars) = char_stack.take() {
                    let string = String::from_iter(chars);
                    token_list.push(string);
                }
            };

            // consume end
            if ';' == char {
                dump_to_list();
                token_list.push(char.to_string());
                break;
            };
            // consume whitespace
            if ' ' == char {
                dump_to_list();
                continue;
            };
            // consume comma
            if ',' == char {
                dump_to_list();
                token_list.push(char.to_string());
                continue;
            };
            // try operator
            match char {
                '<' | '>' | '=' | '!' => {
                    dump_to_list();
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
                    dump_to_list();
                    token_list.push(char.to_string());
                    continue;
                }
                ')' => {
                    dump_to_list();
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

        return token_list;
    }
}


pub fn select_parse(token: Vec<String>) -> SELECT {
    let token = token.into_iter();
    let mut select = vec![];
    let mut from = vec![];
    let mut where_expr = vec![];
    let mut temp = vec![];
    for t in token {
        match t.as_str() {
            ";" => where_expr.append(&mut temp),
            "WHERE" => from.append(&mut temp),
            "FROM" => select.append(&mut temp),
            "ORDER" => panic!(),
            default => temp.push(t)
        }
    }


    let mut query = SELECT { fields: vec![], FROM: None, WHERE: None };
    for t in select.into_iter() {
        if (t == "SELECT") || (t == ",") {
            continue;
        }
        if t == "*" {
            query.fields.push(Field::All);
            break;
        }
        query.fields.push(Field::Plain { name: t, table_name: None, AS: None });
    }
    if from != Vec::new() as Vec<String> {
        query.FROM = Some(from);
    }
    query
}

pub fn create_parse(token: Vec<String>) -> CREATE {
    let mut query = CREATE { TABLE: None };
    let mut token_iter = token.into_iter();
    token_iter.next().unwrap(); // CREATE
    token_iter.next().unwrap(); // TABLE
    let table_name = token_iter.next().unwrap();
    let mut fields = vec![];
    while let Some(t) = token_iter.next() {
        match t.as_str() {
            "(" => {}
            ")"|";" => { break; }
            "," => {}
            default => {
                let mut field_name = t;
                let mut field_type = Type::text;

                if let Some(n_t) = token_iter.next() {
                    match n_t.as_str() {
                        "int" => {
                            field_type = Type::integer;
                        }
                        "text" => {
                            field_type = Type::text;
                        }
                        default => {
                            panic!()
                        }
                    }
                    fields.push(FieldDefinition { name: field_name, T: field_type })
                }
            }
        }
    }

    query.TABLE = Some((table_name, fields));
    query
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
    use crate::sql::lexer::{Lexer, Tokenizer};
    use crate::sql::exec::Executor;
    use crate::storage::storage::Storage;
    use crate::storage::util::gen_hash;

    #[test]
    fn select_from() {
        let mut exec = Executor { storage: Storage::new() };
        let sql = "SELECT * FROM test_table WHERE age>=20;";
        println!("{:?}", gen_hash(&"test_table".to_string()));
        println!("{:?}", exec.parse(Tokenizer::exec(sql.to_string())));
    }

    #[test]
    fn create_table() {
        let mut exec = Executor { storage: Storage::new() };
        let sql = "CREATE TABLE aaa(id int , item_name text);";
        println!("{:?}", Tokenizer::exec(sql.to_string()));
        exec.parse(Tokenizer::exec(sql.to_string()));
    }

    #[test]
    fn insert() {}
}

