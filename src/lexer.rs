use std::thread::current;

#[derive(Debug)]
pub enum Token {
    // Brackets
    PARENS_L,
    PARENS_R,

    BRACKET_L,
    BRACKET_R,

    BRACE_L,
    BRACE_R,

    // Separators
    POINT,
    COMMA,
    COLON,
    SEMICOLON,
    ARROW_R, // (->)

    // Arithmetic Operators
    ADD,
    SUB,
    MUL,
    DIV,

    // Relational Operators
    EQ,
    LT,
    GT,
    NEQ, // Not Equal (!=)
    NLT, // Not Less Than (>=)
    NGT, // Not Greater Than (<=)

    // Logical Operators
    NOT,
    AND,
    OR,

    // Assignment
    ASSIGN,

    // Keywords
    FUNC,
    LET,
    IF,
    ELSE,
    WHILE,
    PRINT,

    // Identifiers
    ID { name: String },

    // Basic Types
    TYPE_INT32 { value: i32 },
    TYPE_FLT32 { value: f32 },
    TYPE_CHAR { value: char },

    // Literals
    LIT_INT32 { value: i32 },
    LIT_FLT32 { value: f32 },
    LIT_CHAR { value: char },
    LIT_STRING { value: String },

    // End-of-Input
    EOI,
}

pub enum LexerState {
    Start,
    End,

    State1,

    Equals,
    LessThan,
    GreaterThan,
}

pub struct Lexer {
    input_string: String,
    position: usize,
    state: LexerState,
    pub current_token: Token,
    buffer_string: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input_string: input,
            position: 0,
            state: LexerState::Start,
            current_token: Token::EOI,
            buffer_string: String::new(),
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input_string = input;
        self.position = 0;
        self.state = LexerState::Start;
        self.current_token = Token::EOI;
        self.buffer_string = String::new();
    }

    pub fn advance(&mut self) -> &Token {
        loop {
            if self.position >= self.input_string.len() {
                if !self.buffer_string.is_empty() {
                    self.state = LexerState::Start;
                    self.current_token = Token::ID {
                        name: self.buffer_string.clone(),
                    };
                    self.buffer_string = String::new();
                    break;
                }
                self.state = LexerState::End;
                self.current_token = Token::EOI;
                break;
            }

            let current_char = self.input_string.chars().nth(self.position).unwrap();
            //print!("{}", current_char);

            self.position += 1;

            match self.state {
                LexerState::Start => match current_char {
                    'A'..='Z' | 'a'..='z' | '_' => {
                        self.state = LexerState::State1;
                        self.buffer_string.push(current_char);
                    }
                    '{' => {
                        self.current_token = Token::BRACE_L;
                        break;
                    }
                    '}' => {
                        self.current_token = Token::BRACE_R;
                        break;
                    }
                    '[' => {
                        self.current_token = Token::BRACKET_L;
                        break;
                    }
                    ']' => {
                        self.current_token = Token::BRACKET_R;
                        break;
                    }
                    '(' => {
                        self.current_token = Token::PARENS_L;
                        break;
                    }
                    ')' => {
                        self.current_token = Token::PARENS_R;
                        break;
                    }

                    _ => {}
                },

                LexerState::State1 => match current_char {
                    'A'..'Z' | '_' | 'a'..'z' | '0'..'9' => {
                        self.buffer_string.push(current_char);
                    }

                    _ => {
                        self.state = LexerState::Start;
                        self.current_token = Token::ID {
                            name: self.buffer_string.clone(),
                        };
                        self.buffer_string = String::new();

                        self.position -= 1;
                        break;
                    }
                },

                _ => {}
            }
        }
        self.curr()
    }

    pub fn curr(&self) -> &Token {
        &self.current_token
    }

    pub fn print_tokens(&mut self) {
        loop {
            self.advance();
            println!("{:?}", self.curr());
            if let Token::EOI = self.curr() {
                break;
            }
        }
    }
}
