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

fn rotate(m: &mut Array2<char>) -> Result<Array2<char>, anyhow::Error> {
    // in   c1  c2  c3      out c1  c2  c3
    // r1   1   2   3       r1  7   4   1
    // r2   4   5   6  =>   r2  8   5   2
    // r3   7   8   9       r3  9   6   3

    // c1 [1, 4, 7] => flip => r1 [7, 4, 1]
    // c2 [2, 5, 8] => flip => r2 [8, 5, 2]
    // c3 [3, 6, 9] => flip => r3 [9, 6, 3]
    //

    let mut teehee: Vec<char> = m
        .columns()
        .into_iter()
        .map(|col| -> Vec<char> { col.to_slice().into_iter().rev().clone() })
        .flatten()
        .collect();

    let array: Array2<char> = Array2::from_shape_vec((3, 3), teehee).expect("spanish inquisition");
    println!("why god ?{:?}", array);
    let tee = m.column_mut(0).t().to_owned();
    println!("{}", &tee);
    m.swap((0, 0), (2, 0));
    todo!()
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
        assert_eq!(rotate(&mut array).unwrap(), &target);
        assert_eq!(rotate(&mut array).unwrap(), &target);
    }
}
