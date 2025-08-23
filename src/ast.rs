use std::{rc::Rc, sync::Arc};

use crate::SharedStr;

#[derive(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sourcepos_bin_encode", derive(bincode::Encode))]
#[cfg_attr(feature = "sourcepos_bin_decode", derive(bincode::Decode))]
pub struct SourcePos {
    pub source: SharedStr,
    pub filename: SharedStr,
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
    pub fn new(source: SharedStr, filename: SharedStr, start: usize, end: usize, line: usize) -> Self {
        SourcePos { source, filename, start, end, line }
    }

    pub fn empty() -> Self {
        SourcePos { source: Rc::from(""), filename: Rc::from(""), start: 0, end: 1, line: 0 }
    }
}

pub trait WithPosition {
    fn get_pos(&self) -> SourcePos;
}

impl<T: WithPosition> WithPosition for &T {
    fn get_pos(&self) -> SourcePos {
        (*self).get_pos()
    }
}

impl<T: WithPosition> WithPosition for &mut T {
    fn get_pos(&self) -> SourcePos {
        (**self).get_pos()
    }
}

impl<T: WithPosition> WithPosition for Box<T> {
    fn get_pos(&self) -> SourcePos {
        (**self).get_pos()
    }
}

impl<T: WithPosition> WithPosition for Rc<T> {
    fn get_pos(&self) -> SourcePos {
        (**self).get_pos()
    }
}

impl<T: WithPosition> WithPosition for Arc<T> {
    fn get_pos(&self) -> SourcePos {
        (**self).get_pos()
    }
}