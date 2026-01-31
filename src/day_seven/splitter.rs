#[derive(Clone)]
pub(crate) struct Splitter {
    row: usize,
    column: usize,
    value: u64
}

impl Splitter {
    pub(crate) fn new(row: usize, column: usize, value: u64) -> Self {
        Splitter {
            row,
            column,
            value
        }
    }

    pub(crate) fn get_row(&self) -> usize {
        self.row
    }

    pub(crate) fn get_column(&self) -> usize {
        self.column
    }


    pub(crate) fn get_value(&self) -> u64 {
        self.value
    }
}