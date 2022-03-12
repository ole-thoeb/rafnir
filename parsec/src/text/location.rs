use std::cmp::Ordering;
use std::ops::{Range};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Location {
    byte_offset: usize,
    column: usize,
    row: usize,
}

impl Location {
    pub fn new(byte_offset: usize, column: usize, row: usize) -> Self {
        Location {
            byte_offset,
            column,
            row
        }
    }

    pub fn new_line(&self, offset_increment: usize) -> Self {
        Self {
            byte_offset: self.byte_offset + offset_increment,
            column: 1,
            row: self.row + 1,
        }
    }

    pub fn increment(&self, offset_increment: usize) -> Self {
        Self {
            byte_offset: self.byte_offset + offset_increment,
            column: self.column + 1,
            row: self.row,
        }
    }

    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn locate<T>(self, end: Location, target: T) -> Located<T> {
        Located {
            source_range: self..end,
            target
        }
    }

    pub fn start() -> Self {
        Self {
            byte_offset: 0,
            column: 1,
            row: 1
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::start()
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.byte_offset.partial_cmp(&other.byte_offset)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.byte_offset.cmp(&other.byte_offset)
    }
}

pub type SourceRange = Range<Location>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Located<T> {
    source_range: SourceRange,
    target: T
}

impl<T> Located<T> {
    fn source_range(&self) -> &SourceRange {
        &self.source_range
    }

    fn target(&self) -> &T {
        &self.target
    }
}