use ndarray::{Array2, ArrayView, ArrayView1, ArrayView2};

pub fn solution_day_four() {
    let input = include_str!("input.txt");
    part_one(input);
}

fn part_one(input: &str) {
    let array = create_2darray(input);
}

fn create_2darray(input: &str) -> Result<Array2<char>, anyhow::Error> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = data.len();
    let cols = data[0].len();

    let flat_data = data.into_iter().flatten().collect();
    let array: Array2<char> = Array2::from_shape_vec((rows, cols), flat_data)?;
    Ok(array)
}

fn horizontal_match(a: &Array2<char>, m: &[char]) -> bool {
    let row = a.row(0).to_vec();
    match_slice(&row, m)
}

fn diagonal_match(a: Array2<char>, m: &[char]) -> bool {
    let diag = a.diag().to_vec();
    match_slice(&diag, m)
}

fn match_slice(row: &[char], m: &[char]) -> bool {
    let mut rev = row.to_owned();
    rev.reverse();
    row == m || rev == m
}

fn rot90(m: &mut Array2<char>) -> Result<Array2<char>, anyhow::Error> {
    let axes_list: Vec<usize> = (0..m.ndim()).collect();
    let mut swapped_axes_list = axes_list.clone();

    let copy = swapped_axes_list[0].clone();
    swapped_axes_list[0] = swapped_axes_list[1];
    swapped_axes_list[1] = copy;

    return Ok(transpose_and_flip(m, 1, &swapped_axes_list));
}

// Function to flip the array along the given axis
fn flip(m: &Array2<char>, axis: usize) -> Array2<char> {
    let mut flipped = m.to_owned();
    flipped.swap_axes(axis, (axis + 1) % m.ndim()); // Perform axis flip
    flipped
}

// Function to transpose the array based on the axes list
fn transpose_and_flip(m: &Array2<char>, axis: usize, axes_list: &[usize]) -> Array2<char> {
    let transposed = m.view().reversed_axes().to_owned(); // Transpose the ndarray
    flip(&transposed, axis)
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
        assert!(horizontal_match(&array, &target_slice));
        assert!(horizontal_match(&array_rev, &target_slice));
        assert!(!horizontal_match(&array_unexpected, &target_slice));
    }

    #[test]
    fn check_diagonal_row() {
        let array = array![['a', 'b', 'c'], ['a', 'b', 'c'], ['a', 'b', 'c']];
        let array_rev = array![['c', 'b', 'a'], ['c', 'b', 'a'], ['c', 'b', 'a']];
        let array_unexpected = array![['v', 'b', 'c'], ['a', 'b', 'c'], ['a', 'b', 'c']];

        let target_slice = vec!['a', 'b', 'c'];

        assert!(diagonal_match(array, &target_slice));
        assert!(diagonal_match(array_rev, &target_slice));
        assert!(!diagonal_match(array_unexpected, &target_slice));
    }

    #[test]
    fn transpose_transposes() {
        let mut array = array![['a', 'b', 'c'], ['d', 'e', 'f'], ['g', 'h', 'i']];
        let target = array![['g', 'd', 'a'], ['h', 'e', 'c'], ['g', 'd', 'a']];

        assert_eq!(rot90(&mut array).unwrap(), &target);
    }
}
