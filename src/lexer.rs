#[derive(Debug)]
pub enum Keyword {
    IF,
    RETURN,
    ELSE,
    CLASS,
    LET,
    PRINT,
    SECTION,
}

#[derive(Debug)]
pub enum Symbol {
    LParen,
    RParen,
    Quote,
    BinOp(char), // +, -, *, /
    Sqrt,  // ^
    LCurly,
    RCurly,
    Comma,
    Colon,
    SemiColon,
    Assignment,
    Comparison,
    ReturnType // ~
}

// TODO create generic lexeme for types
#[derive(Debug)]
pub enum Lexeme {
    Word(Vec<char>),
    Number(i64),
    Boolean(bool),
    Function,
    Identifier(Vec<char>),
    Keyword(Keyword),
    Symbol(Symbol),
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

    fn peek_next_char(&self) -> char {
        let next_char = self.input.iter().nth(self.position + 1);
        if let Some(inside) = next_char {
            *inside
        } else {
            panic!("Next char is unknown");
        }
    }

    fn set_file_navigators(&mut self) {
        // while self.current_char != ' ' {

        // }
        self.position = self.position + 1;
        self.read_position = self.read_position + 1;
        self.current_char = if self.input.get(self.position).is_some() {
            *self.input.get(self.position).unwrap()
        } else {
            '0'
        };
    }
    
    fn go_to_next_line(&mut self) {
        while self.current_char != '\n' {
            println!("{}", self.current_char);
            self.set_file_navigators();
        }
        self.current_line_no += 1;
        self.read_position = 0;
        self.set_file_navigators();
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
                || val == ':'
                || val == '~'
        });
        if index_of_terminal.is_some() {
            let tok_value: Vec<char> =
                self.input[self.position..index_of_terminal.unwrap() + self.position].to_vec();
            self.position = index_of_terminal.unwrap() - 1 + self.position;
            let string: String = tok_value.into_iter().collect();
            self.read_position = self.read_position + string.len();
            match string.as_str() {
                "if" => {
                    Node {
                        lexeme: Lexeme::Keyword(Keyword::IF),
                        line_number: self.current_line_no,
                        start_col: self.read_position - string.len(),
                        end_col: self.read_position,
                    }
                }
                "else" => {
                    Node {
                        lexeme: Lexeme::Keyword(Keyword::ELSE),
                        line_number: self.current_line_no,
                        start_col: self.read_position - string.len(),
                        end_col: self.read_position,
                    }
                },
                "class" =>  {
                    Node {
                        lexeme: Lexeme::Keyword(Keyword::CLASS),
                        line_number: self.current_line_no,
                        start_col: self.read_position - string.len(),
                        end_col: self.read_position,
                    }
                },
                "let"  => {
                    Node {
                        lexeme: Lexeme::Keyword(Keyword::LET),
                        line_number: self.current_line_no,
                        start_col: self.read_position - string.len(),
                        end_col: self.read_position,
                    }
                },
                "print" =>  {
                    Node {
                        lexeme: Lexeme::Keyword(Keyword::PRINT),
                        line_number: self.current_line_no,
                        start_col: self.read_position - string.len(),
                        end_col: self.read_position,
                    }
                },
                "section" =>  {
                    Node {
                        lexeme: Lexeme::Keyword(Keyword::SECTION),
                        line_number: self.current_line_no,
                        start_col: self.read_position - string.len(),
                        end_col: self.read_position,
                    }
                },
                "return" => {
                    Node {
                        lexeme: Lexeme::Keyword(Keyword::RETURN),
                        line_number: self.current_line_no,
                        start_col: self.read_position - string.len(),
                        end_col: self.read_position,
                    }
                }
                _ => {
                    // TODO handle when its a type dec vs a string
                    let node: Node = match string.parse::<i64>() {
                        Ok(number) => Node {
                            lexeme: Lexeme::Number(number),
                            line_number: self.current_line_no,
                            start_col: self.read_position - string.len(),
                            end_col: self.read_position,
                        },
                        Err(e) => Node {
                            lexeme: Lexeme::Identifier(string.chars().collect()),
                            line_number: self.current_line_no,
                            start_col: self.read_position - string.len(),
                            end_col: self.read_position,
                        },
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


    pub fn lex(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        while self.position < self.input.len() {
            match self.current_char {
                '(' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::LParen),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                ')' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::RParen),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                '"' => {
                    // TODO handle strings
                    let node = self.parse_word();
                    nodes.push(node);
                }
                '{' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::LCurly),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                '}' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::RCurly),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                },
                ':' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::Colon),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                },
                ',' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::Comma),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                },
                '~' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::ReturnType),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                }
                '/' => {
                    let next_char = self.peek_next_char();
                    if next_char == '/' {
                        self.go_to_next_line();
                    }  else {
                        let node = Node {
                            lexeme: Lexeme::Symbol(Symbol::BinOp(self.current_char)),
                            line_number: self.current_line_no,
                            start_col: self.read_position,
                            end_col: self.read_position + 1,
                        };
                        nodes.push(node);
                    }
                },
                '+' | '-' | '*' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::BinOp(self.current_char)),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                },
                '=' => {
                    // TODO look ahead to next char to see if it is = as well. If so then symbol is Comparison
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::Assignment),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                },
                ';' => {
                    let node = Node {
                        lexeme: Lexeme::Symbol(Symbol::SemiColon),
                        line_number: self.current_line_no,
                        start_col: self.read_position,
                        end_col: self.read_position + 1,
                    };
                    nodes.push(node);
                },
                '\n' => {
                    self.current_line_no = self.current_line_no + 1;
                    self.read_position = 0;
                    println!("Hitting new line");
                }
                _ => {
                    if self.current_char.is_alphanumeric() {
                        nodes.push(self.parse_word());
                    }
                }
            }

            self.set_file_navigators();
        }
        nodes
    }
}
