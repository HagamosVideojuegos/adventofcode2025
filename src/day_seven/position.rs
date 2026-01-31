#[derive(Debug)]
pub(crate) struct Position {
    pub char: char
}

impl Position {
    pub(crate) fn new(char: char) -> Position {
        Position { char }
    }

    pub(crate) fn set_laser(&mut self) {
        self.char = '|'
    }

    pub(crate) fn is_splitter(&self) -> bool {
        self.char == '^'
    }

    pub(crate) fn is_empty_space(&self) -> bool {
        self.char == '.'
    }

    pub(crate) fn is_mainfold(&self) -> bool {
        self.char == 'S'
    }

    pub(crate) fn is_beam(&self) -> bool {
        self.char == '|'
    }
}