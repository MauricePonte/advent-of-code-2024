use ndarray::{s, Array2, ArrayView2};

pub fn solution_day_four() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let pattern = ['X', 'M', 'A', 'S'];
    let array = create_2darray(input).unwrap();
    let padded_arr = pad_array(&array, 4, 'f');
    let mut answer = check_array(&padded_arr, pattern);
    let rotated_array = rotate2(&padded_arr);
    answer.extend(&check_array(&rotated_array, pattern));
    println!("Part one: {}", answer.iter().filter(|&&a| a).count());
}

fn part_two(input: &str) {
    let pattern = ['M', 'A', 'S'];
    let array = create_2darray(input).unwrap();
    let padded_arr = pad_array(&array, 4, 'f');
    let answer = check_array_two(&padded_arr, pattern);
    println!("Part two: {}", answer.iter().filter(|&&a| a).count());
}

fn check_array(array: &Array2<char>, pattern: [char; 4]) -> Vec<bool> {
    let mut answer = Vec::new();
    array
        .windows_with_stride((4, 4), (1, 1))
        .into_iter()
        .for_each(|window| {
            answer.push(horizontal_match(&window, &pattern));
            answer.push(diagonal_match(&window, &pattern));
        });
    answer
}

fn check_array_two(array: &Array2<char>, pattern: [char; 3]) -> Vec<bool> {
    let mut answer = Vec::new();
    array
        .windows_with_stride((3, 3), (1, 1))
        .into_iter()
        .for_each(|window| {
            let a = diagonal_match(&window, &pattern);
            let r_a = rotate2(&window.to_owned());
            let b = diagonal_match(&r_a.view(), &pattern);
            answer.push(a && b);
        });
    answer
}

fn create_2darray(input: &str) -> Result<Array2<char>, anyhow::Error> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = data.len();
    let cols = data[0].len();

    let flat_data = data.into_iter().flatten().collect();
    let array: Array2<char> = Array2::from_shape_vec((rows, cols), flat_data)?;
    Ok(array)
}

fn horizontal_match(a: &ArrayView2<char>, m: &[char]) -> bool {
    let row = a.row(0).to_vec();
    match_slice(&row, m)
}

fn diagonal_match(a: &ArrayView2<char>, m: &[char]) -> bool {
    let diag = a.diag().to_vec();
    match_slice(&diag, m)
}

fn match_slice(row: &[char], m: &[char]) -> bool {
    let mut rev = row.to_owned();
    rev.reverse();
    row == m || rev == m
}

fn rotate2(grid: &Array2<char>) -> Array2<char> {
    let transposed = grid.t();

    let (rows, cols) = grid.dim();

    let mut rotated = Array2::default((cols, rows));
    for (i, row) in transposed.outer_iter().enumerate() {
        rotated.row_mut(i).assign(&row.slice(s![..;-1]));
    }

    rotated
}

fn pad_array(grid: &Array2<char>, padding: usize, pad_char: char) -> Array2<char> {
    let (rows, cols) = grid.dim();

    let padded_rows = rows + 2 * padding;
    let padded_cols = cols + 2 * padding;

    let mut padded = Array2::from_elem((padded_rows, padded_cols), pad_char);

    padded
        .slice_mut(s![padding..padding + rows, padding..padding + cols])
        .assign(grid);

    padded
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn create_2darray_returns_correct_arr() {
        let input = include_str!("input.txt");
        let array = create_2darray(input).expect("failed to create arr");
        assert_eq!(array.dim().0, 140);
        assert_eq!(array.dim().1, 140);
    }

    #[test]
    fn check_horizontal_row() {
        let array = array![['a', 'b', 'c'], ['a', 'b', 'c'], ['a', 'b', 'c']];
        let array_rev = array![['c', 'b', 'a'], ['c', 'b', 'a'], ['c', 'b', 'a']];
        let array_unexpected = array![['v', 'b', 'c'], ['a', 'b', 'c'], ['a', 'b', 'c']];

        let target_slice = vec!['a', 'b', 'c'];
        assert!(horizontal_match(&array.view(), &target_slice));
        assert!(horizontal_match(&array_rev.view(), &target_slice));
        assert!(!horizontal_match(&array_unexpected.view(), &target_slice));
    }

    #[test]
    fn check_diagonal_row() {
        let array = array![['a', 'b', 'c'], ['a', 'b', 'c'], ['a', 'b', 'c']];
        let array_rev = array![['c', 'b', 'a'], ['c', 'b', 'a'], ['c', 'b', 'a']];
        let array_unexpected = array![['v', 'b', 'c'], ['a', 'b', 'c'], ['a', 'b', 'c']];

        let target_slice = vec!['a', 'b', 'c'];

        assert!(diagonal_match(&array.view(), &target_slice));
        assert!(diagonal_match(&array_rev.view(), &target_slice));
        assert!(!diagonal_match(&array_unexpected.view(), &target_slice));
    }

    #[test]
    fn check_array_two_valid() {
        let array = array![['M', 'A', 'M'], ['M', 'A', 'S'], ['S', 'A', 'S']];
        let pattern = ['M', 'A', 'S'];
        let a = check_array_two(&array, pattern);
        assert_eq!(a.iter().filter(|&&a| a).count(), 1);
    }
}
