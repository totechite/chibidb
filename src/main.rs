mod lexer;
mod token;

use crate::token::{Token};
use crate::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("SELECT a, b FROM table ;");
    let mut tokenlist = lexer.exec();
    println!("{:#?}", tokenlist);
    println!("{:?}",  tokenlist.pop());
    println!("{:?}",  tokenlist.pop());

}