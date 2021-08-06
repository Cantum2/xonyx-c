use super::lexer;
use super::lexer::Keyword;
use super::lexer::Lexeme;
use super::lexer::Symbol;

#[derive(Debug, PartialEq)]
pub enum Production {
    ProgStart,
    ClassDec,
    BlockBody,
    Vardec,
    TypeDec,
    FunctionDec,
    Expression,
    Ident,
}

#[derive(Debug, PartialEq)]
pub struct ASTNode {
    production: Option<Production>,
    children: Vec<ASTNode>,
    value: Option<Vec<char>>,
    operator: Option<Vec<char>>,
}

#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<lexer::Node>,
    root: ASTNode,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Node>) -> Parser {
        let root = ASTNode {
            production: Some(Production::ProgStart),
            children: vec![],
            value: Some(vec![]),
            operator: Some(vec![]),
        };
        Parser { tokens, root }
    }

    pub fn parse_assignment(&mut self) -> ASTNode {
        let current_node = self.tokens.pop().unwrap();
        let next_node = self.tokens.pop().unwrap();
        match next_node.lexeme {
            Lexeme::Symbol(value) => match value {
                Symbol::Assignment => ASTNode {
                    children: vec![],
                    operator: None,
                    value: match next_node.lexeme {
                        Lexeme::Number(value) => Some(value), // fix me: need to make it to where value is vec<char> or num
                        _ => panic!("Expected Variable dec found {:?}", current_node.lexeme),
                    },
                    production: Some(Production::Vardec),
                },
                _ => panic!("Expected ':' found {:?}", value),
            },
            _ => panic!("Expected Symbol found {:?}", next_node.lexeme),
        }
    }

    pub fn parse_type(&mut self) -> ASTNode {
        let current_node = self.tokens.pop().unwrap();
        ASTNode {
            children: vec![self.parse_assignment()], // probably not always the case
            operator: None,
            value: match current_node.lexeme {
                Lexeme::Identifier(value) => match value[..] {
                    ['N', 'u', 'm', 'b', 'e', 'r'] => Some(value),
                    ['S', 't', 'r', 'i', 'n', 'g'] => Some(value),
                    _ => panic!("Expected String or Number found {:?}", value),
                },
                _ => panic!("Expected Identifier found {:?}", current_node.lexeme),
            },
            production: Some(Production::TypeDec),
        }
    }

    pub fn parse_vardec(&mut self) -> ASTNode {
        let current_node = self.tokens.pop().unwrap();
        let next_node = self.tokens.pop().unwrap();
        match next_node.lexeme {
            Lexeme::Symbol(value) => match value {
                Symbol::Colon => ASTNode {
                    // [0] is var name [1] is type (Number, String, Function etc.)
                    children: vec![self.parse_ident(), self.parse_type()],
                    operator: None,
                    value: match current_node.lexeme {
                        Lexeme::Keyword(value) => Some("let".chars().collect()),
                        _ => panic!("Expected Variable dec found {:?}", current_node.lexeme),
                    },
                    production: Some(Production::Vardec),
                },
                _ => panic!("Expected ':' found {:?}", value),
            },
            _ => panic!("Expected Symbol found {:?}", next_node.lexeme),
        }
    }

    pub fn parse_ident(&mut self) -> ASTNode {
        let current_node = self.tokens.pop().unwrap();
        ASTNode {
            children: vec![], // probably not always the case
            operator: None,
            value: match current_node.lexeme {
                Lexeme::Identifier(value) => Some(value),
                _ => panic!("Expected Identifier found {:?}", current_node.lexeme),
            },
            production: None,
        }
    }

    // pub fn verify_next_symbol(&mut self, symbol_to_check: Symbol) -> Result<Ok{
    //     let current_token = self.tokens.pop().unwrap();
    //     match current_token.lexeme {
    //         Lexeme::Symbol(value) => match value {
    //             symbol_to_check => Ok,
    //             _ => panic!("Expected {:?} found {:?} ", symbol_to_check, value),
    //         },
    //         _ => panic!(
    //             "Expected Symbol: {:?} found {:?} ",
    //             symbol_to_check, current_token.lexeme
    //         ),
    //     }
    // }

    pub fn throw_error(&mut self) {}

    pub fn parse(&mut self) -> ASTNode {
        // check if first one is class
        println!("tokens: {:#?}", self.tokens);
        let mut ast_node = ASTNode {
            production: None,
            children: vec![],
            value: Some(vec![]),
            operator: Some(vec![]),
        };
        loop {
            let current_token = self.tokens.pop();
            match current_token {
                Some(node) => {
                    match node.lexeme {
                        Lexeme::Keyword(value) => {
                            match value {
                                Keyword::IF => {}
                                Keyword::RETURN => {}
                                Keyword::ELSE => {}
                                Keyword::CLASS => {
                                    // verify next symbol is curly
                                    let n = ASTNode {
                                        children: vec![self.parse_ident(), self.parse()],
                                        operator: Some(vec![]),
                                        value: Some(vec![]),
                                        production: Some(Production::ClassDec),
                                    };
                                    self.root.children.push(n);
                                }
                                Keyword::LET => {
                                    ast_node = ASTNode {
                                        children: vec![self.parse_vardec()],
                                        operator: Some(vec![]),
                                        value: Some(vec![]),
                                        production: Some(Production::ClassDec),
                                    };
                                }
                                Keyword::PRINT => {}
                                Keyword::SECTION => {}
                                Keyword::SNIPPET => {}
                            }
                        }
                        Lexeme::Identifier(value) => {}
                        Lexeme::Symbol(value) => match value {
                            _ => {}
                        },
                        _ => {}
                    }
                }
                None => break,
            }
        }
        ast_node
    }
}
