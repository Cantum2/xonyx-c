use std::io;
use std::io::Read;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum Lexeme {
    Word(Vec<char>),
    Number(i64),
    LParen(char),
    RParen(char),
    QUOTE(char),
    SemiColon(char),
    Add(char),
    Subtract(char),
    Divide(char),
    Sqrt(char),
    Multiply(char),
    LCurly(char),
    RCurly(char),
    VarDec(Vec<char>), // change me
    IF,
    RETURN,
    ELSE,
    Unknown,
}

#[derive(Debug)]
pub struct Node {
    lexeme: Lexeme,
    line_number: i32,
    start_col: usize,
    end_col: usize,
}

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub current_char: char,
    pub current_line_no: i32,
}

impl Lexer {
    pub fn new(input: &Vec<char>) -> Lexer {
        Lexer {
            input: input.clone(),
            position: 0,
            read_position: 1,
            current_char: if input.get(0).is_some() {
                *input.get(0).unwrap()
            } else {
                panic!("panic")
            },
            current_line_no: 1,
        }
    }

    fn parse_word(&mut self) -> Node {
        // only check after our current position -- micro-optimization
        let index_of_terminal = &self.input[self.position..].iter().position(|&val| {
            val == ' '
                || val == ';'
                || val == '\r'
                || val == '\n'
                || val == '('
                || val == ')'
                || val == '"'
        });
        if index_of_terminal.is_some() {
            let tok_value: Vec<char> = self.input[self.position..index_of_terminal.unwrap() + self.position].to_vec();
            self.position = index_of_terminal.unwrap() - 1 + self.position;
            let string: String = tok_value.into_iter().collect();
            self.read_position = self.read_position + string.len();
            match string.as_str() {
                "if" => {
                    let n = Node {
                        lexeme: Lexeme::IF,
                        line_number: self.current_line_no,
                        start_col:  self.read_position - string.len(),
                        end_col: self.read_position,
                    };
                    self.read_position = self.read_position + 2;
                    n
                }
                "else" => {
                    let n = Node {
                        lexeme: Lexeme::ELSE,
                        line_number: self.current_line_no,
                        start_col:  self.read_position - string.len(),
                        end_col: self.read_position,
                    };
                    self.read_position = self.read_position + 4;
                    n
            },
                "return" => {
                    let n = Node {
                        lexeme: Lexeme::RETURN,
                        line_number: self.current_line_no,
                        start_col:  self.read_position - string.len(),
                        end_col: self.read_position,
                    };
                    self.read_position = self.read_position + 6;
                    n
            },
                _ => {
                    let node: Node = match string.parse::<i64>() {
                        Ok(number) => {
                            Node {
                            lexeme: Lexeme::Number(number),
                            line_number: self.current_line_no,
                            start_col:  self.read_position - string.len(),
                            end_col: self.read_position,
                        }},
                        Err(e) => {Node {
                            lexeme: Lexeme::Word(string.chars().collect()),
                            line_number: self.current_line_no,
                            start_col:  self.read_position - string.len(),
                            end_col: self.read_position,
                        }},
                    };
                    node
                }
            }
        } else {
            Node {
                lexeme: Lexeme::Unknown,
                line_number: self.current_line_no,
                start_col: self.position,
                end_col: self.position + 1,
            }
        }
    }

    fn advance(&mut self) {}

    pub fn lex(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        while self.position < self.input.len() {
            println!("Current char: {}", &self.current_char);
            match self.current_char {
                '(' => {
                    let node = Node {
                        lexeme: Lexeme::LParen('('),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    println!("Found paren");
                    nodes.push(node);
                }
                ')' => {
                    let node = Node {
                        lexeme: Lexeme::RParen('('),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                '"' => {
                    let node = Node {
                        lexeme: Lexeme::QUOTE('"'),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                '{' => {
                    let node = Node {
                        lexeme: Lexeme::LCurly('{'),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                '}' => {
                    let node = Node {
                        lexeme: Lexeme::RCurly('}'),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                ';' => {
                    let node = Node {
                        lexeme: Lexeme::SemiColon(';'),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                '\n' => {
                    self.current_line_no = self.current_line_no + 1;
                    self.read_position = 0;
                    println!("nodex on new line: {:?}", nodes);
                }
                _ => {
                    if self.current_char.is_alphanumeric() {
                        nodes.push(self.parse_word());
                    }
                }
            }
            self.position = self.position + 1;
            self.read_position = self.read_position + 1;
            self.current_char = if self.input.get(self.position).is_some() {
                *self.input.get(self.position).unwrap()
            } else {
                '0'
            };
        }
        nodes
    }
}
