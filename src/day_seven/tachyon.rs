use std::cell::RefCell;
use std::rc::Rc;
use crate::day_seven::position::Position;
use crate::day_seven::splitter::Splitter;

pub(crate) struct Tachyon {
    structure: Vec<Vec<Position>>
}

impl Tachyon {
    pub(crate) fn new(new_structure: Vec<String>) -> Tachyon {
        Tachyon {
            structure: new_structure.iter()
                .map(|row| row.chars().map(Position::new).collect())
                .collect()
        }
    }

    pub(crate) fn shoot_beam(&mut self) {
        for row in 0..self.structure.len() {
            for column in 0..self.structure[row].len() {
                if self.is_laser(row, column) {
                    continue
                } else if self.is_mainfold(row, column) {
                    if row < self.structure.len() - 1 {
                        if self.is_empty_space(row + 1, column) {
                            self.set_laser(row + 1, column);
                        } else if self.is_splitter(row + 1, column) {
                            self.split(row + 1, column)
                        }
                    }
                } else if self.is_empty_space(row, column) {
                    if self.is_on_bean(row, column) {
                        self.set_laser(row, column);
                    }
                } else if self.is_splitter(row, column) {
                    if self.is_on_bean(row, column) {
                        self.split(row, column);
                    }
                } else {
                    eprintln!("Unknown character encountered: {:?}", self.structure[row][column]);
                }
            }
        }
    }

    pub(crate) fn set_laser(&mut self, row: usize, column: usize) {
        self.structure[row][column].set_laser()
    }

    pub(crate) fn is_splitter(&self, row: usize, column: usize) -> bool {
        self.is_in_range(row, column) && self.structure[row][column].is_splitter()
    }

    pub(crate) fn is_empty_space(&self, row: usize, column: usize) -> bool {
        self.is_in_range(row, column) && self.structure[row][column].is_empty_space()
    }

    pub(crate) fn is_mainfold(&self, row: usize, column: usize) -> bool {
        self.is_in_range(row, column) && self.structure[row][column].is_mainfold()
    }

    pub(crate) fn is_laser(&self, row: usize, column: usize) -> bool {
        self.is_in_range(row, column) && self.structure[row][column].is_beam()
    }

    pub fn is_final(&self, row: usize, column: usize) -> bool {
        for current_row in (row + 1)..self.structure.len() {
            if self.structure[current_row][column].is_splitter() {
                return false;
            }
        }
        true
    }

    pub fn is_on_splitter(&self, row: usize, column: usize) -> Option<usize> {
        for current_row in (row + 1)..self.structure.len() {
            if self.structure[current_row][column].is_splitter() {
                return Some(current_row);
            }
        }
        None
    }

    fn is_in_range(&self, row: usize, column: usize) -> bool {
        column < self.structure[row].len()
    }

    fn split(&mut self, row: usize, column: usize) {
        self.split_left(row, column);
        self.split_right(row, column);
    }

    fn split_left(&mut self, row: usize, column: usize) {
        if column > 0 {
            self.set_laser(row, column - 1);
        }
    }

    fn split_right(&mut self, row: usize, column: usize) {
        if column < self.structure[row].len() - 1 {
            self.set_laser(row, column + 1);
        }
    }

    pub(crate) fn is_on_bean(&self, row: usize, column: usize) -> bool {
        row > 0 && self.is_laser(row - 1, column)
    }

    pub(crate) fn count_splits(&self) -> u64 {
        let mut counter = 0;
        for row in 0..self.structure.len() {
            for column in 0..self.structure[row].len() {
                if self.is_splitter(row, column) && self.is_on_bean(row, column) {
                    counter += 1;
                }
            }
        }
        counter
    }

    pub(crate) fn count_quantum_splits(&mut self) -> u64 {
        let mut splitters: Vec<Rc<RefCell<Splitter>>> = Vec::new();
        for row in (0..self.structure.len() - 1).rev().step_by(2) {
            for column in 0..self.structure[row].len() {
                if self.is_splitter(row, column) && self.is_on_bean(row, column) {
                    let mut value = 0;
                    if self.is_final(row + 1, column - 1) {
                        value += 1;
                    } else {
                        let left_child = self.is_on_splitter(row + 1, column - 1);
                        if left_child.is_some() {
                            value += splitters.iter().find(|x| {
                                let splitter = x.borrow();
                                splitter.get_row() == left_child.unwrap() && splitter.get_column() == column - 1
                            }).map(|x| x.borrow().get_value()).unwrap_or(0)
                        }
                    }
                    if self.is_final(row + 1, column + 1) {
                        value += 1;
                    } else {
                        let right_child = self.is_on_splitter(row + 1, column + 1);
                        if right_child.is_some() {
                            value += splitters.iter().find(|x| {
                                let splitter = x.borrow();
                                splitter.get_row() == right_child.unwrap() && splitter.get_column() == column + 1
                            }).map(|x| x.borrow().get_value()).unwrap_or(0)
                        }
                    }
                    splitters.push(Rc::new(RefCell::new(Splitter::new(row, column, value))));
                }
            }
        }
        
        splitters.iter().find(|x| x.borrow().get_row() == 2).map(|x| x.borrow().get_value()).unwrap_or(0)
    }
}