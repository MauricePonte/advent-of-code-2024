use std::{iter::Cycle, usize::MAX};

use itertools::Itertools;
use ndarray::Array2;

pub fn solution_day_six() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) -> usize {
    let map_arr = init_array(input).unwrap();
    let mut game_map = Map::new(map_arr);
    let mut player = Player::new(&game_map);
    game_loop(&mut game_map, &mut player)
}

fn part_two(input: &str) -> Vec<(usize, usize)> {
    let map_arr = init_array(input).unwrap();
    let mut game_map = Map::new(map_arr);
    let mut player = Player::new(&game_map);
    game_loop(&mut game_map, &mut player);
    println!(
        "Answer to part two is: {} possible obs",
        game_map.possible.iter().unique().count()
    );
    game_map.possible
}

fn game_loop(game_map: &mut Map, player: &mut Player) -> usize {
    let mut visited_positions = vec![player.current_pos];
    loop {
        match player.take_step(game_map) {
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
    directions: std::iter::Peekable<Cycle<std::vec::IntoIter<(isize, isize)>>>,
    current_direction: (isize, isize),
    visited_obs: Vec<((usize, usize), (isize, isize))>,
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
        .cycle()
        .peekable();

        let current_pos = map.clone().find('^').get(0).copied().unwrap_or((0, 0));
        let current_direction = directions.next().unwrap();
        let visited_obs = Vec::new();
        Self {
            current_pos,
            steps_taken: 0,
            directions,
            current_direction,
            visited_obs,
        }
    }

    pub fn take_step(&mut self, map: &mut Map) -> Option<(usize, usize)> {
        //println!("BEFORE STEP");
        println!("current direction: {:?}", &self.current_direction);
        println!("current position: {:?}", &self.current_pos);

        let next_pos = (
            (self.current_pos.0 as isize + self.current_direction.0) as usize,
            (self.current_pos.1 as isize + self.current_direction.1) as usize,
        );

        if !map.is_position_valid(&next_pos) {
            return None;
        }

        if map.check_obs(&next_pos) {
            self.current_direction = self.directions.next().unwrap();
            self.visited_obs.push((next_pos, self.current_direction));
        } else {
            if self.look_right(map, next_pos) {
                println!("found {:?}", next_pos);
                println!("cdir {:?}", self.current_direction);
                map.possible.push(next_pos);
            }
            self.steps_taken += 1;
            self.current_pos = next_pos;
        }

        //println!("AFTER STEP");
        Some(self.current_pos)
    }

    fn look_right(&self, map: &mut Map, next_pos: (usize, usize)) -> bool {
        if !map.is_position_valid(&next_pos) && !map.check_obs(&next_pos) {
            return false;
        }

        let next_dir = self.directions.clone().peek().unwrap().clone();
        if self.visited_obs.contains(&(next_pos, next_dir)) {
            return true;
        }

        return self.look_right(
            map,
            (
                ((next_pos.0 as isize) + next_dir.0) as usize,
                ((next_pos.1 as isize) + next_dir.1) as usize,
            ),
        );
    }
}

#[derive(Clone)]
struct Map {
    inner_map: Array2<char>,
    obstacles: Vec<(usize, usize)>,
    possible: Vec<(usize, usize)>,
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
            possible: Vec::new(),
        }
    }

    fn find(&self, target: char) -> Vec<(usize, usize)> {
        self.inner_map
            .indexed_iter()
            .filter_map(|(i, &value)| if value == target { Some(i) } else { None })
            .collect()
    }

    fn is_position_valid(&self, position: &(usize, usize)) -> bool {
        let (x, y) = position;
        (*x != 0 && *y != 0) && (*x < self.inner_map.nrows() && *y < self.inner_map.ncols())
    }

    fn check_obs(&self, position: &(usize, usize)) -> bool {
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
    use super::*;

    #[test]
    fn tst() {
        let input = include_str!("test.txt");
        let answer = part_two(input);
        println!("possible {:?}", answer);
        assert_eq!(answer.iter().count(), 6);
    }
}
