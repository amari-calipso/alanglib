use std::rc::Rc;

use crate::{ast::{SourcePos, WithPosition}, SharedStr};

#[derive(Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Token<TokenType: std::fmt::Debug + Default> {
    pub source: SharedStr,
    pub filename: SharedStr,
    pub type_: TokenType,
    pub lexeme: SharedStr,
    pub pos: usize,
    pub end: usize,
    pub line: usize
}

impl<TokenType: std::fmt::Debug + Default> std::fmt::Debug for Token<TokenType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token").field("type_", &self.type_).field("lexeme", &self.lexeme).field("pos", &self.pos).field("end", &self.end).field("line", &self.line).finish()
    }
}

impl<TokenType: std::fmt::Debug + Default> Token<TokenType> {
    pub fn new(source: SharedStr, filename: SharedStr, type_: TokenType, lexeme: SharedStr, pos: usize, end: usize, line: usize) -> Self {
        Token { source, filename, type_, lexeme, pos, end, line }
    }

    pub fn dummy(lexeme: SharedStr) -> Self {
        Token { 
            source: SharedStr::from(""), 
            filename: SharedStr::from(""), 
            type_: TokenType::default(), 
            lexeme, pos: 0, end: 1, line: 0 
        }
    }

    pub fn empty() -> Self {
        Token::dummy(SharedStr::from(""))
    }

    pub fn from_type(type_: TokenType) -> Self {
        Token { 
            source: SharedStr::from(""), 
            filename: SharedStr::from(""), 
            type_, 
            lexeme: SharedStr::from(""), 
            pos: 0, end: 1, line: 0 
        }
    }

    pub fn set_type(&mut self, type_: TokenType) {
        self.type_ = type_;
    }

    pub fn set_lexeme(&mut self, lexeme: &str) {
        self.lexeme = Rc::from(lexeme);
    }
}

impl<TokenType: std::fmt::Debug + Default> WithPosition for Token<TokenType> {
    fn get_pos(&self) -> SourcePos {
        SourcePos::new(
            SharedStr::clone(&self.source), 
            SharedStr::clone(&self.filename), 
            self.pos, self.end, self.line
        )
    }
}