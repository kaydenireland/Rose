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
            self.expect(Token::ARROW_R);
            self.expect(Token::id());
            self.parse_block_nest();
        }
        self.indent_decrement();
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
            self.expect(Token::id());
        }
        self.indent_decrement();
    }

    pub fn parse_block_nest(&mut self) {
        self.indent_print("parse_block_nest()");
        self.indent_increment();
        {
            self.expect(Token::BRACKET_L);
            if self.peek(Token::BRACKET_L) {
                self.parse_block_list();
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
            if self.peek(Token::BRACKET_L) {
                self.parse_block_list()
            }
        }
        self.indent_decrement();
    }
}
