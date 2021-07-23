use std::fs::File;
use std::env;
use std::io;
use std::io::Read;
mod lexer;

fn main() {
    let file = env::args().nth(1).expect("Supply a file name");
    let text = file_as_text(&file).expect("Bad file");
    println!("{}", &text);
    let mut lexer = lexer::Lexer::new(&text.chars().collect());
    assert_eq!(lexer.current_char, 'T');
    assert_eq!(lexer.input.len(), text.len());
    let toks = lexer.lex();
    // println!("toks: {:?}", toks);
}

fn file_as_text(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

