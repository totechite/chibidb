use regex::Regex;
use std::string::ToString;
use std::iter::{FromIterator, Enumerate};
use crate::sql::token::{Token};
use crate::sql::plan::{SELECT, Field, Table, SearchCondition, CREATE, FieldDefinition, Type};
use crate::sql::token::Token::Condition;
use protobuf::well_known_types::Field_Kind::TYPE_BOOL;
use std::collections::HashMap;
use crate::sql::node::{SELECT_FROM_Declare, Column_Term, Column_Context, Table_Term, Table_Context, FROM_Declare, SELECT_Declare, JOIN_Declare, WHERE_Declare, ConditionTerm, OperatorTerm, LogicalOperator};
use crate::sql::node::Table_Context::Literal;
use std::thread;
use std::sync::mpsc;
use std::slice::Iter;
use std::cell::RefCell;

pub fn noise_scanner(c: char) -> bool {
    (' ' == c) || ('\n' == c) || (',' == c)
}

#[derive(Debug, Default)]
pub struct Lexer {
    query_string: String,
    tokens: Vec<String>,
}


impl Lexer {
    pub fn new(tokens: Vec<String>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    pub fn exec(&self) -> Vec<Token> {
        let mut left_paren_counter = 0;
        let mut right_paren_counter = 0;
        for (idx, token) in self.tokens.iter().enumerate() {
            match token.as_str() {
                "(" => left_paren_counter += 1,
                ")" => right_paren_counter += 1,
                ";" => {
                    if left_paren_counter != right_paren_counter {
                        panic!("don't have much paren")
                    }
                }
                "SELECT" => { self.select_from(); }
                "CREATE" => {}
                "TABLE" => {}
                "INSERT" => {}
                "DELETE" => {}
                _ => {}
            }
        }

        return vec![];
    }

    fn select_from(&self) {
        let mut declare = SELECT_FROM_Declare::new();
        declare.text = self.tokens.join(" ");


        let (select_term, from_term,
            join_term, where_term,
            orderby_term, groupby_term) = self.split_to_terms();
        println!("{:?}", select_term);
        println!("{:?}", from_term);
        println!("{:?}", where_term);

        // parse SELECT
        let select_declare = thread::spawn(move || Lexer::parse_select(select_term.as_slice()));

        // parse FROM
        let from_declare = thread::spawn(move || Lexer::parse_from(from_term.as_slice()));

        // parse WHERE
        let where_declare = if let Some(where_term) = where_term {
            Some(Lexer::parse_where(where_term.as_slice()))
        } else { None };

        declare.select_term = select_declare.join().unwrap();
        declare.from_term = from_declare.join().unwrap();
        declare.where_term = where_declare;


        println!("{:#?}", declare);
    }

    fn parse_select(select_term: &[String]) -> SELECT_Declare {
        let mut select_declare = SELECT_Declare { text: select_term.join(" "), contents: vec![] };
        let mut token_with_index = select_term.iter().enumerate();
        while let Some((idx, token)) = token_with_index.next() {
            match token.as_str() {
                "SELECT" => continue,
                _ => {
                    let mut text = vec![token.as_str()];
                    let column_context = {
                        let mut sepalete = token.clone().split(".").map(|s| s.to_string()).collect::<Vec<String>>();
                        sepalete.reverse();
                        let column_name = sepalete.get(0).unwrap().clone();
                        let attribute = sepalete.get(1).cloned();
                        Column_Context::Literal { column_name, attribute }
                    };
                    let mut alias = None;
                    if let Some((idx, expect_AS)) = token_with_index.next() {
                        match expect_AS.as_str() {
                            "AS" => {
                                alias = Some(token_with_index.next().unwrap().1);
                                text.append(&mut vec!["AS", &alias.unwrap()]);
                            }
                            "," => {}
                            _ => panic!()
                        }
                    }
                    let column_term = Column_Term {
                        text: text.join(" "),
                        context: column_context,
                        alias: alias.cloned(),
                    };
                    select_declare.contents.push(column_term);
                }
            }
        }
        select_declare
    }

    fn parse_from(from_term: &[String]) -> FROM_Declare {
        let mut from_declare = FROM_Declare { text: from_term.join(" "), contents: vec![] };
        let mut token_with_index = from_term.iter().enumerate();
        while let Some((idx, token)) = token_with_index.next() {
            match token.as_str() {
                "FROM" => continue,
                _ => {
                    let mut text = vec![token.as_str()];
                    let table_name = token.as_str();
                    let mut alias = None;
                    if let Some((idx, expected_alias)) = token_with_index.next() {
                        match expected_alias.as_str() {
                            "," => {} // do nothing
                            _ => {
                                text.push(expected_alias.as_str());
                                alias = Some(expected_alias.clone());
                            }
                        }
                    }
                    let table_term = Table_Term { text: text.join(" ").to_string(), context: Table_Context::Literal(table_name.to_string()), alias };
                    from_declare.contents.push(table_term);
                }
            }
        }
        from_declare
    }

    fn parse_where(where_term: &[String]) -> WHERE_Declare {
        let mut token_with_index = where_term.iter().enumerate();
        token_with_index.next().unwrap();
        let content = Self::aux_parse_paren(&mut token_with_index);

        let where_declare = WHERE_Declare { text: where_term.join(" "), content };
        where_declare
    }

    fn aux_parse_paren(token_with_index: &mut Enumerate<Iter<String>>) -> Box<ConditionTerm> {
        match Self::parse_paren(token_with_index).0 {
            OperatorTerm::Operator(condition_term) => condition_term,
            _ => panic!()
        }
    }

    fn parse_paren(token_with_index: &mut Enumerate<Iter<String>>) -> (OperatorTerm<Box<ConditionTerm>>, String) {
        let mut tmp_formula = None;
        let mut text: RefCell<Vec<String>> = RefCell::new(vec![]);

        fn v_parse(value: String) -> OperatorTerm<Box<ConditionTerm>> {
            match value.parse::<usize>() {
                Ok(num) => OperatorTerm::Number(num),
                Err(_) => OperatorTerm::String(value),
            }
        }

        let gen_ConditionTerm = |operator: LogicalOperator, left_v: OperatorTerm<Box<ConditionTerm>>, right_v: OperatorTerm<Box<ConditionTerm>>| -> ConditionTerm {
            return ConditionTerm { text: text.borrow_mut().join(" "), operator, left_v, right_v };
        };

        while let Some((idx, token)) = token_with_index.next() {
            if token == "(" {
                return Self::parse_paren(token_with_index);
            }

            if token == ")" {
                return (tmp_formula.unwrap(), text.borrow_mut().join(" "));
            }

            match token.as_str() {
                "AND" => {
                    let operator = LogicalOperator::AND;
                    let right_v = Self::parse_paren(token_with_index);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                "OR" => {
                    let operator = LogicalOperator::OR;
                    let right_v = Self::parse_paren(token_with_index);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                "NOT" => {}
                "=" => {
                    let operator = LogicalOperator::Equal;
                    let right_v = Self::parse_paren(token_with_index);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                "<>" => {
                    let operator = LogicalOperator::NotEqual;
                    let right_v = Self::parse_paren(token_with_index);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                "<" => {
                    let operator = LogicalOperator::LessThan;
                    let right_v = Self::parse_paren(token_with_index);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                "<=" => {
                    let operator = LogicalOperator::EqualOrLessThan;
                    let right_v = Self::parse_paren(token_with_index);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                ">" => {
                    let operator = LogicalOperator::GreaterThan;
                    let right_v = Self::parse_paren(token_with_index);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                ">=" => {
                    let operator = LogicalOperator::EqualOrGreaterThan;
                    let right_v = Self::parse_paren(token_with_index);
                    println!("{:?}", right_v);
                    text.borrow_mut().push(token.clone());
                    text.borrow_mut().push(right_v.1.clone());
                    let condition_term = gen_ConditionTerm(operator, tmp_formula.clone().unwrap(), right_v.0);
                    tmp_formula = Some(OperatorTerm::Operator(Box::new(condition_term)));
                }
                _ => {
                    text.borrow_mut().push(token.clone());
                    tmp_formula = Some(v_parse(token.clone()));
                    println!("{:?}", tmp_formula);
                    println!("{:?}", token_with_index);


                    if let Some(expect_zero) = token_with_index.clone().position(|(_, str)| vec!["AND","OR","NOT"].contains(&str.as_str()) ) {
                        println!("{:?}", token_with_index);

                        if expect_zero==0{
                            break
                        }
                    }

                }
            }
        }
        // println!("{:?}", tmp_formula);
        return (tmp_formula.unwrap(), text.borrow_mut().join(" "));
    }


    fn split_to_terms(&self) -> (Vec<String>, Vec<String>, Option<Vec<String>>, Option<Vec<String>>, Option<Vec<String>>, Option<Vec<String>>) {
        // SELECT FROM
        let from_position = self.tokens.iter().position(|t| t == "FROM").unwrap();
        let (select_term, mut from_term) = {
            let (select_term, mut from_term) = self.tokens.split_at(from_position).clone();
            (select_term.iter().map(String::from).collect::<Vec<String>>().to_vec(), from_term.into_iter().map(String::from).collect::<Vec<String>>().to_vec())
        };

        let join_position = self.tokens.iter().position(|t| t == "JOIN");
        let where_position = self.tokens.iter().position(|t| t == "WHERE");
        let orderby_position = self.tokens.iter().position(|t| t == "ORDER");
        let groupby_position = self.tokens.iter().position(|t| t == "GROUP");
        for position in vec![join_position, where_position, orderby_position, groupby_position] {
            if let Some(position) = position {
                let (_, term) = self.tokens.split_at(position).0.split_at(from_position);
                from_term = term.into_iter().map(String::from).collect::<Vec<String>>().to_vec();
                break;
            }
        }

        // JOIN
        let mut join_term = None;
        if let Some(target_position) = join_position {
            let mut term = self.tokens.split_at(target_position).1;
            for position in vec![where_position, orderby_position, groupby_position] {
                if let Some(position) = position {
                    term = term.split_at(position).0;
                    break;
                }
            }
            join_term = Some(term.into_iter().map(String::from).collect::<Vec<String>>());
        }

        // WHERE
        let mut where_term = None;
        if let Some(target_position) = where_position {
            let mut term = self.tokens.split_at(target_position).1;
            println!("{:?}", term);
            for position in vec![orderby_position, groupby_position] {
                if let Some(position) = position {
                    term = term.split_at(position).0;
                    break;
                }
            }
            where_term = Some(term.into_iter().map(String::from).collect::<Vec<String>>());
        }

        // ORDER BY
        let mut orderby_term = None;
        if let Some(target_position) = orderby_position {
            let mut term = self.tokens.split_at(target_position).1;
            for position in vec![groupby_position] {
                if let Some(position) = position {
                    term = term.split_at(position).0;
                }
            }
            orderby_term = Some(term.into_iter().map(String::from).collect::<Vec<String>>());
        }

        // GROUP BY
        let mut groupby_term = None;
        if let Some(target_position) = groupby_position {
            let mut term = self.tokens.split_at(target_position).1;
            for position in vec![self.tokens.iter().position(|t| t == ";")] {
                if let Some(position) = position {
                    term = term.split_at(position).0;
                    break;
                }
            }
            groupby_term = Some(term.into_iter().map(String::from).collect::<Vec<String>>());
        }

        (select_term, from_term, join_term, where_term, orderby_term, groupby_term)
    }
}


#[cfg(test)]
mod lexer {
    use crate::sql::tokenizer::Tokenizer;
    use crate::sql::lexer::Lexer;

    #[test]
    fn select_from() {
        let sql = "SELECT test_table.a, b AS total FROM test_table WHERE age>=20 AND (name='Smith' OR name = 'John');";
        let tokens = Tokenizer::exec(sql.to_string());
        println!("{:?}", tokens);
        Lexer::new(tokens).exec();
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
            ")" | ";" => { break; }
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


#[cfg(test)]
mod test {
    use crate::sql::lexer::{Lexer};
    use crate::sql::exec::Executor;
    use crate::storage::storage::Storage;
    use crate::storage::util::gen_hash;
    use crate::sql::tokenizer::Tokenizer;

    #[test]
    fn select_from() {
        let mut exec = Executor { storage: Storage::new() };
        let sql = "SELECT test_table.a, b, a + b AS total FROM test_table WHERE age>=20;";

        println!("{:?}", Tokenizer::exec(sql.to_string()));
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

