use std::env;

pub fn main() {
    let input = match env::var("DAY_FOUR_INPUT") {
        Ok(seq) => seq.split(' ').map(String::from).collect::<Vec<String>>(),
        Err(_) => {
            eprintln!("DAY_FOUR_INPUT environment variable not found, using default sequence");
            vec![
                "..@@.@@@@.".to_string(),
                "@@@.@.@.@@".to_string(),
                "@@@@@.@.@@".to_string(),
                "@.@@@@..@.".to_string(),
                "@@.@@@@.@@".to_string(),
                ".@@@@@@@.@".to_string(),
                ".@.@.@.@@@".to_string(),
                "@.@@@.@@@@".to_string(),
                ".@@@@@@@@.".to_string(),
                "@.@.@@@.@.".to_string()
            ]
        }
    };

    let mut matrix = Matrix::new(input.iter().max().unwrap().len() as u32, input.len() as u32);
    for x in 0..input.len() {
        for (y, c) in input[x].chars().enumerate() {
            matrix.set(x as u32, y  as u32, c)
        }
    }

    let first = part_one(&matrix);
    println!("Secret code one: {}", first);
    let second = part_two(&matrix);
    println!("Secret code two: {}", second);
}

fn part_one(matrix: &Matrix) -> u32 {
    let current_matrix: Matrix = matrix.clone();
    count(&current_matrix).code
}

fn part_two(matrix: &Matrix) -> u32 {
    let mut code: u32 = 0;
    let mut current_matrix = matrix.clone();
    loop {
        let current = count_updating(&current_matrix);
        code += current.code;
        current_matrix = current.matrix;
        if current.code == 0 {
            break;
        }
    }
    code
}

fn count(matrix: &Matrix) -> MatrixCode {
    let mut code: u32 = 0;
    for x in 0..matrix.rows {
        for y in 0..matrix.cols {
            if check_cell(&matrix, x, y) {
                code += 1
            }
        }
    }
    MatrixCode::new(matrix.clone(), code)
}

fn count_updating(matrix: &Matrix) -> MatrixCode {
    let mut code: u32 = 0;
    let mut matrix = matrix.clone();
    for x in 0..matrix.rows {
        for y in 0..matrix.cols {
            if check_cell(&matrix, x, y) {
                matrix.set(x, y, 'X');
                code += 1
            }
        }
    }
    MatrixCode::new(matrix.clone(), code)
}

fn check_cell(current_matrix: &Matrix, x: u32, y: u32) -> bool {
    if current_matrix.get(x, y) != '@' { return false; }
    let mut count_around: i8 = 0;
    if x > 0 {
        if current_matrix.get(x - 1, y) == '@' { // left
            count_around += 1;
        }
        if y > 0 {
            if current_matrix.get(x - 1, y - 1) == '@' { // top left
                count_around += 1;
            }
        }
        if y < current_matrix.rows - 1 {
            if current_matrix.get(x - 1, y + 1) == '@' { // bottom left
                count_around += 1;
            }
        }
    }
    if y > 0 {
        if current_matrix.get(x, y - 1) == '@' { // top
            count_around += 1;
        }
        if x < current_matrix.cols - 1 {
            if current_matrix.get(x + 1, y - 1) == '@' { // top right
                count_around += 1;
            }
        }
    }
    if y < current_matrix.rows - 1 {
        if current_matrix.get(x, y + 1) == '@' { // bottom
            count_around += 1;
        }
        if x < current_matrix.cols - 1 {
            if current_matrix.get(x + 1, y + 1) == '@' { // bottom right
                count_around += 1;
            }
        }
    }
    if x < current_matrix.cols - 1 {
        if current_matrix.get(x + 1, y) == '@' { // right
            count_around += 1;
        }
    }
    count_around < 4
}

#[derive(Clone)]
struct Matrix {
    cols: u32,
    rows: u32,
    data: Vec<char>
}

impl Matrix {
    fn new(cols: u32, rows: u32) -> Matrix {
        Matrix {
            cols,
            rows,
            data: vec!['.'; usize::try_from(cols * rows).unwrap()]
        }
    }

    fn get(&self, x: u32, y: u32) -> char {
        self.data[usize::try_from((y * self.cols) + x).unwrap()]
    }

    fn set(&mut self, x: u32, y: u32, value: char) {
        self.data[usize::try_from((y * self.cols) + x).unwrap()] = value;
    }
}

struct MatrixCode {
    matrix: Matrix,
    code: u32,
}

impl MatrixCode {
    fn new(matrix: Matrix, code: u32) -> MatrixCode {
        MatrixCode { matrix, code }
    }
}
