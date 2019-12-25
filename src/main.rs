mod lexer;
mod token;
mod util;

use crate::token::{Token};
use crate::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("SELECT * FROM table WHERE year==2019;");
    let mut tokenlist = lexer.exec();
    println!("{:#?}", tokenlist);
    println!("{:?}",  tokenlist.pop());
    println!("{:?}",  tokenlist.pop());
}