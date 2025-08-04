use std::rc::Rc;

use crate::ast::{SourcePos, WithPosition};

#[derive(Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Token<TokenType: std::fmt::Debug + Default> {
    pub source: Rc<str>,
    pub filename: Rc<str>,
    pub type_: TokenType,
    pub lexeme: Rc<str>,
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
    pub fn new(source: Rc<str>, filename: Rc<str>, type_: TokenType, lexeme: Rc<str>, pos: usize, end: usize, line: usize) -> Self {
        Token { source, filename, type_, lexeme, pos, end, line }
    }

    pub fn dummy(lexeme: Rc<str>) -> Self {
        Token { source: Rc::from(""), filename: Rc::from(""), type_: TokenType::default(), lexeme, pos: 0, end: 1, line: 0 }
    }

    pub fn empty() -> Self {
        Token::dummy(Rc::from(""))
    }

    pub fn from_type(type_: TokenType) -> Self {
        Token { source: Rc::from(""), filename: Rc::from(""), type_, lexeme: Rc::from(""), pos: 0, end: 1, line: 0 }
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
            Rc::clone(&self.source), 
            Rc::clone(&self.filename), 
            self.pos, self.end, self.line
        )
    }
}