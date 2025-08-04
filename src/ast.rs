use std::rc::Rc;

#[derive(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "astpos_bin_encode", derive(bincode::Encode))]
#[cfg_attr(feature = "astpos_bin_decode", derive(bincode::Decode))]
pub struct SourcePos {
    pub source: Rc<str>,
    pub filename: Rc<str>,
    pub start: usize,
    pub end: usize,
    pub line: usize
}

impl std::fmt::Debug for SourcePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstPos").field("filename", &self.filename).field("start", &self.start).field("end", &self.end).field("line", &self.line).finish()
    }
}

impl SourcePos {
    pub fn new(source: Rc<str>, filename: Rc<str>, start: usize, end: usize, line: usize) -> Self {
        SourcePos { source, filename, start, end, line }
    }

    pub fn empty() -> Self {
        SourcePos { source: Rc::from(""), filename: Rc::from(""), start: 0, end: 1, line: 0 }
    }
}

pub trait WithPosition {
    fn get_pos(&self) -> SourcePos;
}