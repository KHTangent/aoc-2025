use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq, Debug)]
enum TileType {
	Empty,
	Beam,
	Splitter,
}
impl TileType {
	fn from_char(c: char) -> Self {
		match c {
			'S' => Self::Beam,
			'^' => Self::Splitter,
			_ => Self::Empty,
		}
	}
}

struct TileMap(Vec<Vec<TileType>>);
type MemoizeCache = HashMap<(usize, usize), i64>;
impl TileMap {
	fn from_string(s: &str) -> Self {
		TileMap(
			s.lines()
				.map(|line| line.chars().map(|c| TileType::from_char(c)).collect())
				.collect(),
		)
	}
	fn simulate_splitted(&mut self) -> i64 {
		let mut times_splitted = 0;
		for row in 1..self.0.len() {
			for col in 0..self.0[row].len() {
				if self.0[row - 1][col] != TileType::Beam {
					continue;
				}
				if self.0[row][col] == TileType::Splitter {
					times_splitted += 1;
					if self.0[row][col - 1] == TileType::Empty {
						self.0[row][col - 1] = TileType::Beam;
					}
					if self.0[row][col + 1] == TileType::Empty {
						self.0[row][col + 1] = TileType::Beam;
					}
				} else {
					self.0[row][col] = TileType::Beam;
				}
			}
		}
		times_splitted
	}
	fn paths_up_from(&self, row: usize, col: usize, cache: &mut MemoizeCache) -> i64 {
		if cache.contains_key(&(row, col)) {
			return *cache.get(&(row, col)).unwrap();
		}
		if row == 0 {
			return 1;
		}
		let mut paths = 0;
		if self.0[row - 1][col] == TileType::Beam {
			let new_paths = self.paths_up_from(row - 1, col, cache);
			cache.insert((row - 1, col), new_paths);
			paths += new_paths;
		}
		if col != 0 && self.0[row][col - 1] == TileType::Splitter {
			let new_paths = self.paths_up_from(row - 1, col - 1, cache);
			cache.insert((row - 1, col), new_paths);
			paths += new_paths;
		}
		if (col + 1) < self.0[0].len() && self.0[row][col + 1] == TileType::Splitter {
			let new_paths = self.paths_up_from(row - 1, col + 1, cache);
			cache.insert((row - 1, col), new_paths);
			paths += new_paths;
		}
		paths
	}
}

fn solution(input: &str) -> i64 {
	TileMap::from_string(input.trim()).simulate_splitted()
}

fn solution2(input: &str) -> i64 {
	let mut paths = 0;
	let mut map = TileMap::from_string(input.trim());
	map.simulate_splitted();
	let last_row_index = map.0.len() - 1;
	let mut cache = MemoizeCache::new();
	for i in 0..map.0[0].len() {
		if map.0[last_row_index][i] == TileType::Beam {
			paths += map.paths_up_from(last_row_index, i, &mut cache);
		}
	}
	paths
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

	#[test]
	fn test_solution() {
		let answer = solution(TEST_INPUT);
		assert_eq!(answer, 21);
	}

	#[test]
	fn test_solution2() {
		let answer = solution2(TEST_INPUT);
		assert_eq!(answer, 40);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let file = get_entire_input_file();
	let answer = solution(&file);
	println!("Answer task 1: {}", answer);
	let answer = solution2(&file);
	println!("Answer task 2: {}", answer);
}
