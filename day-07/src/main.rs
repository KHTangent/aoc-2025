use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum TileTypes {
	Empty,
	Beam,
	Splitter,
}
impl TileTypes {
	fn from_char(c: char) -> Self {
		match c {
			'S' => Self::Beam,
			'^' => Self::Splitter,
			_ => Self::Empty,
		}
	}
}
struct TileMap(Vec<Vec<TileTypes>>);
impl TileMap {
	fn from_string(s: &str) -> Self {
		TileMap(
			s.lines()
				.map(|line| line.chars().map(|c| TileTypes::from_char(c)).collect())
				.collect(),
		)
	}
	fn print(&self) {
		for line in self.0.iter() {
			for c in line.iter() {
				match c {
					TileTypes::Empty => print!("."),
					TileTypes::Beam => print!("|"),
					TileTypes::Splitter => print!("^"),
				}
			}
			println!()
		}
	}
	fn simulate_splitted(&mut self) -> i64 {
		let mut times_splitted = 0;
		for row in 1..self.0.len() {
			for col in 0..self.0[row].len() {
				if self.0[row - 1][col] != TileTypes::Beam {
					continue;
				}
				if self.0[row][col] == TileTypes::Splitter {
					times_splitted += 1;
					if self.0[row][col - 1] == TileTypes::Empty {
						self.0[row][col - 1] = TileTypes::Beam;
					}
					if self.0[row][col + 1] == TileTypes::Empty {
						self.0[row][col + 1] = TileTypes::Beam;
					}
				} else {
					self.0[row][col] = TileTypes::Beam;
				}
			}
		}
		times_splitted
	}
	fn paths_up_from(&self, row: usize, col: usize) -> i64 {
		if row == 0 {
			return 1;
		}
		let mut paths = 0;
		if self.0[row - 1][col] == TileTypes::Beam {
			paths += self.paths_up_from(row - 1, col);
		}
		if col != 0 && self.0[row][col - 1] == TileTypes::Splitter {
			paths += self.paths_up_from(row - 1, col - 1);
		}
		if (col + 1) < self.0[0].len() && self.0[row][col + 1] == TileTypes::Splitter {
			paths += self.paths_up_from(row - 1, col + 1);
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
	for i in 0..map.0[0].len() {
		if map.0[last_row_index][i] == TileTypes::Beam {
			paths += map.paths_up_from(last_row_index, i);
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
