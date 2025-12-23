use crate::bst::Tree;
pub mod ast;
pub mod bst;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[allow(dead_code)]
enum Token<'a> {
    // Keywords
    #[token("SELECT")]
    SELECT,
    #[token("WHERE")]
    WHERE,
    #[token("LIMIT")]
    LIMIT,
    // Operators
    #[token("==")]
    EQUALS,
    #[token(">=")]
    GREATEROREQ,
    #[token("<=")]
    LESSOREQ,
    #[token(">")]
    GREATERTHAN,
    #[token("<")]
    LESSTHAN,
    // Punctuation
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    // Identifier
    #[regex("[a-zA-Z0-9_]+")]
    Identifier(&'a str),
}

fn main() {
    let tree = Tree::new();
    println!("{:?}", tree);

    let example_query = "SELECT City, State, Population WHERE Population > 50000";

    let lex = Token::lexer(example_query);

    for token in lex {
        match token {
            Ok(tok) => println!("{:?}", tok),
            _ => (),
        };
    }
}
