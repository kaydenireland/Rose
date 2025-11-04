#![allow(warnings)]

use std::clone;

use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
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
    TYPE_INT32,
    TYPE_FLT32,
    TYPE_CHAR,

    // Literals
    LIT_INT32 { value: i32 },
    LIT_FLT32 { value: f32 },
    LIT_CHAR { value: char },
    LIT_STRING { value: String },

    // End-of-Input
    EOI,
}
