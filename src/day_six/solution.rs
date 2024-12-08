use std::iter::Cycle;

use itertools::Itertools;
use ndarray::Array2;

pub fn solution_day_six() {
    let input = include_str!("input.txt");
    part_one(input);
}

fn part_one(input: &str) -> usize {
    let map_arr = init_array(input).unwrap();
    let game_map = Map::new(map_arr);
    let mut player = Player::new(&game_map);
    game_loop(game_map, &mut player)
}

fn game_loop(game_map: Map, player: &mut Player) -> usize {
    let mut visited_positions = vec![player.current_pos];
    loop {
        match player.take_step(&game_map) {
            Some(pos) => visited_positions.push(pos),
            None => break,
        }
    }
    let unique = visited_positions.iter().unique().count();
    println!("Game finished. Found {} unique positions", unique);
    unique
}

#[derive(Clone)]
struct Player {
    current_pos: (usize, usize),
    pub steps_taken: u32,
    directions: Cycle<std::vec::IntoIter<(isize, isize)>>,
    current_direction: (isize, isize),
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let mut directions = vec![
            (-1, 0), // UP
            (0, 1),  // RIGHT
            (1, 0),  // DOWN
            (0, -1), // LEFT
        ]
        .into_iter()
        .cycle();

        let current_pos = map.clone().find('^').get(0).copied().unwrap_or((0, 0));

        let current_direction = directions.next().unwrap();
        Self {
            current_pos,
            steps_taken: 0,
            directions,
            current_direction,
        }
    }

    pub fn take_step(&mut self, map: &Map) -> Option<(usize, usize)> {
        println!("BEFORE STEP");
        println!("steps taken: {}", self.steps_taken);
        println!("current direction: {:?}", &self.current_direction);
        println!("current position: {:?}", &self.current_pos);

        let next_pos = (
            (self.current_pos.0 as isize + self.current_direction.0) as usize,
            (self.current_pos.1 as isize + self.current_direction.1) as usize,
        );

        if !map.check_bounds(next_pos) {
            return None;
        }

        if map.check_obs(next_pos) {
            self.current_direction = self.directions.next().unwrap();
        } else {
            self.steps_taken += 1;
            self.current_pos = next_pos;
        }

        println!("AFTER STEP");
        println!("steps taken: {}", self.steps_taken);
        println!("current direction: {:?}", &self.current_direction);
        println!("current position: {:?}", &self.current_pos);
        Some(self.current_pos)
    }
}

#[derive(Clone)]
struct Map {
    inner_map: Array2<char>,
    obstacles: Vec<(usize, usize)>,
}

impl Map {
    pub fn new(map: Array2<char>) -> Self {
        let obstacles = map
            .indexed_iter()
            .filter_map(|(i, &value)| if value == '#' { Some(i) } else { None })
            .collect();

        Self {
            inner_map: map,
            obstacles,
        }
    }

    fn find(&self, target: char) -> Vec<(usize, usize)> {
        self.inner_map
            .indexed_iter()
            .filter_map(|(i, &value)| if value == target { Some(i) } else { None })
            .collect()
    }

    fn check_bounds(&self, position: (usize, usize)) -> bool {
        let (max_x, max_y) = self.inner_map.dim();
        position.0 < max_x && position.1 < max_y
    }

    fn check_obs(&self, position: (usize, usize)) -> bool {
        self.obstacles.contains(&position)
    }
}

fn init_array(input: &str) -> Result<Array2<char>, anyhow::Error> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = data.len();
    let cols = data[0].len();

    let flat_data = data.into_iter().flatten().collect();
    let array: Array2<char> = Array2::from_shape_vec((rows, cols), flat_data)?;
    Ok(array)
}

#[cfg(test)]
mod tests {
    use super::part_one;

    #[test]
    fn tst() {
        let input = include_str!("test.txt");
        let answer = part_one(input);
        assert_eq!(answer, 21);
    }
}
