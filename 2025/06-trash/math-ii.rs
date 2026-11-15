use std::io;

type Matrix<T> = Vec<Vec<T>>;

fn calculate(matrix: &Matrix<u64>, ops: &[char]) -> u64 {
    matrix
        .iter()
        .zip(ops.iter())
        .map(|(nums, op)| match op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => 0,
        })
        .sum()
}

fn transpose<T: Copy>(matrix: &Matrix<T>) -> Matrix<T> {
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    (0..num_cols)
        .map(|y| (0..num_rows).map(|x| matrix[x][y]).collect())
        .collect()
}

fn main() {
    let mut matrix = vec![];
    let mut ops = vec![];

    let input: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let mut numbers: Vec<u64> = vec![];

    for mut line in transpose(&input) {
        let last = line.pop().unwrap();
        if !last.is_whitespace() {
            ops.push(last);
        }

        let line = line.iter().copied().collect::<String>();
        let line = line.trim();
        if line.is_empty() {
            matrix.push(numbers);
            numbers = vec![];
        } else {
            numbers.push(line.parse().unwrap());
        }
    }
    matrix.push(numbers);

    println!("{}", calculate(&matrix, &ops));
}
