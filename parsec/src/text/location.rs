#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Location {
    byte_offset: usize,
    column: usize,
    row: usize,
}

impl Location {
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
}

impl Default for Location {
    fn default() -> Self {
        Self {
            byte_offset: 0,
            column: 1,
            row: 1
        }
    }
}