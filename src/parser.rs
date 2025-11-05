use crate::lexer::Lexer;
use crate::mtree::MTree;
use crate::token::Token;

const INDENT: usize = 2;

pub struct Parser {
    lexer: Lexer,
    pub indent: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer, indent: 0 }
    }

    pub fn analyze(&mut self) -> MTree {
        self.indent = 0;
        self.advance();
        self.parse();
        self.expect(Token::EOI);
        MTree::new(Token::EOI)
    }
}

impl Parser {
    // utility functions for lexer
    pub fn curr(&mut self) -> Token {
        self.lexer.curr()
    }

    pub fn advance(&mut self) {
        self.lexer.advance();
    }

    pub fn peek(&mut self, symbol: Token) -> bool {
        self.lexer.curr() == symbol
    }

    pub fn expect(&mut self, symbol: Token) {
        if self.curr() == symbol {
            self.advance();
            println!("{:<indent$}expect({symbol:?})", "", indent = self.indent);
        } else {
            panic!("Expected '{symbol:?}', currently '{:?}'!", self.curr());
        }
    }

    pub fn expect_type(&mut self) {
        if self.curr().is_type() {
            self.advance();
            println!(
                "{:<indent$}expect({:?})",
                "",
                self.curr(),
                indent = self.indent
            );
        } else {
            panic!("Expected variable type, currently '{:?}'!", self.curr());
        }
    }

    pub fn accept(&mut self, symbol: Token) -> bool {
        if self.curr() == symbol {
            self.advance();
            true
        } else {
            false
        }
    }
}

impl Parser {
    // utility functions for pretty print

    pub fn indent_print(&mut self, msg: &'static str) {
        println!("{:<indent$}{:}", "", msg, indent = self.indent);
    }

    pub fn indent_increment(&mut self) {
        self.indent += INDENT;
    }
    pub fn indent_decrement(&mut self) {
        self.indent -= INDENT;
    }
}

impl Parser {
    // simple recursive descend parser

    pub fn parse(&mut self) {
        self.parse_func();
    }

    pub fn parse_func(&mut self) {
        self.indent_print("parse_func()");
        self.indent_increment();
        {
            self.expect(Token::FUNC);
            self.expect(Token::id());
            self.parse_parameter_list();

            if self.accept(Token::ARROW_R) {
                self.expect_type();
            }

            self.parse_block_nest();
        }
        self.indent_decrement();

        if self.peek(Token::FUNC) {
            self.parse_func();
        }
    }

    pub fn parse_parameter_list(&mut self) {
        self.indent_print("parse_parameter_list()");
        self.indent_increment();
        {
            self.expect(Token::PARENS_L);
            if self.accept(Token::PARENS_R) {
                return;
            }
            self.parse_parameter();
            while self.accept(Token::COMMA) {
                // list -> ( {param{,param}+}? )
                self.parse_parameter(); // param -> id : id
            }
            self.expect(Token::PARENS_R);
        }
        self.indent_decrement();
    }

    pub fn parse_parameter(&mut self) {
        self.indent_print("parse_parameter()");
        self.indent_increment();
        {
            self.expect(Token::id());
            self.expect(Token::COLON);
            self.expect_type();
        }
        self.indent_decrement();
    }

    pub fn parse_block_nest(&mut self) {
        self.indent_print("parse_block_nest()");
        self.indent_increment();
        {
            self.expect(Token::BRACKET_L);
            while !self.peek(Token::BRACKET_R) {
                self.parse_statement();
            }
            self.expect(Token::BRACKET_R);
        }
        self.indent_decrement();
    }

    pub fn parse_block_list(&mut self) {
        self.indent_print("parse_block_list()");
        self.indent_increment();
        {
            self.parse_block_nest();
            while !self.peek(Token::BRACKET_R) {
                self.parse_statement();
            }
        }
        self.indent_decrement();
    }
}

impl Parser {
    // statement/expression parsing functions

    pub fn parse_statement(&mut self) {
        self.indent_print("parse_statement()");
        self.indent_increment();
        {
            match self.curr() {
                Token::LET => self.parse_let(),
                Token::IF => self.parse_if(),
                Token::RETURN => self.parse_return(),
                Token::BRACKET_L => self.parse_block_nest(),
                _ => panic!("Unexpected token '{:?}' in statement!", self.curr()),
            }
        }
        self.indent_decrement();
    }

    pub fn parse_expression(&mut self) {
        self.indent_print("parse_expression()");
        self.indent_increment();

        match self.curr() {
            Token::LIT_INT32 { .. } => {
                self.expect(Token::lit_i32());
            }
            Token::LIT_FLT32 { .. } => {
                self.expect(Token::lit_f32());
            }
            Token::LIT_CHAR { .. } => {
                self.expect(Token::lit_char());
            }
            Token::LIT_STRING { .. } => {
                self.expect(Token::lit_string());
            }
            Token::ID { .. } => {
                self.expect(Token::id());
            }
            _ => {
                panic!("Unexpected token '{:?}' in expression!", self.curr());
            }
        }

        self.indent_decrement();
    }

    pub fn parse_let(&mut self) {
        self.indent_print("parse_let()");
        self.indent_increment();
        {
            self.expect(Token::LET);
            self.expect(Token::id());

            if self.accept(Token::COLON) {
                if self.curr().is_type() {
                    self.advance();
                } else {
                    panic!("Expected type token after ':', got {:?}", self.curr());
                }
            }

            self.expect(Token::ASSIGN);
            self.parse_expression();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }

    pub fn parse_if(&mut self) {
        self.indent_print("parse_if()");
        self.indent_increment();
        {
            self.expect(Token::IF);
            self.parse_expression();
            self.parse_block_nest();
            if self.accept(Token::ELSE) {
                self.parse_block_nest();
            }
        }
        self.indent_decrement();
    }

    pub fn parse_return(&mut self) {
        self.indent_print("parse_return()");
        self.indent_increment();
        {
            self.expect(Token::RETURN);
            self.parse_expression();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }
}
