use std::iter::FromIterator;

#[derive(Debug)]
pub struct Tokenizer
{
    query_string: String,
    token_list: Vec<String>,
}

impl Tokenizer {
    pub fn new(qs: &str) -> Self {
        let qs = String::from(qs);
        Self {
            query_string: qs.clone(),
            token_list: vec![],
        }
    }

    pub fn exec(query_string: String) -> Vec<String> {
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
                // token_list.push(char.to_string());
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
            // try LogicalOperator & ArithmeticOperator
            match char {
                '<' | '>' | '=' | '!' => {
                    dump_to_list();
                    if let Some(c) = chars.next() {
                        match c {
                            '=' => {
                                let LogicalOperator = format!("{}{}", char, c);
                                token_list.push(LogicalOperator);
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
                '+' | '-' | '*' | '/' | '%' => {
                    dump_to_list();
                    token_list.push(char.to_string());
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


#[cfg(test)]
mod test {
    use crate::sql::tokenizer::Tokenizer;

    #[test]
    fn select_from() {
        let sql = "SELECT a, b, a+b AS total FROM test_table WHERE age>=20;";

        println!("{:#?}", Tokenizer::exec(sql.to_string()));
    }
}