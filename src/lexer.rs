use core::borrow::BorrowMut;
use regex::Regex;
use crate::token::{Token, PlainToken, Command as C, Value as V, Operator as O};

#[derive(Debug)]
pub struct Lexer {
    query_string: String
}

#[derive(Debug)]
struct FstTokenizer
{
    query_string: String,
    chars: Vec<char>,
}

#[derive(Debug)]
struct ScdTokenizer {
    plain_token_list: Vec<PlainToken>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            query_string: input.to_string()
        }
    }

    pub fn exec(self) -> Vec<Token> {
        let fst_stage = FstTokenizer::new(self.query_string.as_str()).exec();
        ScdTokenizer::new(fst_stage).exec()
    }
}

impl FstTokenizer {
    fn new(qs: &str) -> Self {
        let qs = String::from(qs);
        Self {
            query_string: qs.clone(),
            chars: qs.chars().collect(),
        }
    }

    fn exec(&mut self) -> Vec<PlainToken> {
        let mut token_str = String::new();
        let mut tokenlist: Vec<PlainToken> = Vec::new();
        let mut index = 0;
        while let Some(char) = self.chars.get(index) {
            index += 1;
            if &';' == char {
                tokenlist.push(PlainToken(char.to_string()));
                break;
            }
            if Self::noise_scanner(char) {
                if String::new() == token_str { continue; }
                tokenlist.push(PlainToken(token_str));
                token_str = String::new();
                continue;
            };
            token_str += char.to_string().as_str();
        }
        tokenlist.reverse();
        return tokenlist;
    }

    fn noise_scanner(c: &char) -> bool {
        (&' ' == c) || (&'\n' == c) || (&',' == c)
    }
}

impl ScdTokenizer {
    fn new(tl: Vec<PlainToken>) -> Self {
        ScdTokenizer {
            plain_token_list: tl
        }
    }

    fn exec(&mut self) -> Vec<Token> {
        let mut token_list = Vec::new();
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.0.as_str() {
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
                _ => {}
            }
        }
        return token_list;
    }

    fn select_tokenize(&mut self, token_list: &mut Vec<Token>) {
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.0.as_str() {
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
                    token_list.push(Token::Value(V::Column(pt.0)));
                }
            }
        }
    }

    fn from_tokenize(&mut self, token_list: &mut Vec<Token>) {
        while let Some(pt) = self.plain_token_list.pop() {
            match pt.0.as_str() {
                ";" => {
                    token_list.push(Token::EOF);
                    break;
                }
                _ => {
                    token_list.push(Token::Value(V::Table(pt.0)));
                }
            }
        }
    }

//    fn where_tokenize(&mut self, token_list: &mut Vec<Token>) {
//        while let Some(pt) = self.plain_token_list.pop() {
//            match pt.0.as_str() {
//                _ => {
//                    token_list.push(Token::Value(V::Condition(_)));
//                }
//            }
//        }
//    }
}


#[cfg(test)]
mod test {
    use crate::lexer::Tokenize;

    #[test]
    fn test1() {
        let mut tokenizer = Tokenize::new("SELECT * FROM table_name;");
        tokenizer.exec();
    }
}