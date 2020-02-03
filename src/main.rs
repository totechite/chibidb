//mod lexer;
//mod token;
//mod util;
mod b_plus_tree;

//use crate::lexer::Lexer;
use crate::b_plus_tree::BPlusTree;
use std::fmt::Debug;

fn main() {
//    let mut lexer = Lexer::new("SELECT * FROM table WHERE birth_year==2019 AND sex=='MAN';");
//    let mut tokenlist = lexer.exec();
//    println!("{:#?}", tokenlist);

    let mut btree: BPlusTree<usize, String> = b_plus_tree::BPlusTree::new();

//    for i in 1..8 {
//        let (s, b) = (40 - i * 4, 40 + i * 2);
//        btree.insert(b, "hoge".to_string());
//        btree.insert(s, "hoge".to_string());
//    }
    btree.insert(10usize, "hoge".to_string());
    btree.insert(3usize, "hoge".to_string());
    btree.insert(12usize, "hoge".to_string());
    btree.insert(13usize, "hoge".to_string());
    btree.insert(2usize, "hoge".to_string());
    btree.insert(1usize, "hoge".to_string());
    btree.insert(6usize, "hoge".to_string());
    btree.insert(15usize, "hoge".to_string());
    btree.insert(8usize, "hoge".to_string());
    btree.insert(20usize, "hoge".to_string());

//    btree.delete(1);
//    btree.delete(2);
//    btree.delete(3);
//    btree.delete(6);
//    btree.delete(15);
//    btree.delete(20);
//    btree.delete(13);

//    println!("{:#?}", btree);
    btree.update(6, "hage".to_string());

    println!("{:?}", btree.print());

}