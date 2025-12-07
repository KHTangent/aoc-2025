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
	fn parse_map(s: &str) -> Vec<Vec<Self>> {
		s.lines()
			.map(|line| line.chars().map(|c| TileTypes::from_char(c)).collect())
			.collect()
	}
}
fn print_map(map: &Vec<Vec<TileTypes>>) {
	for line in map.iter() {
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

fn solution(input: &str) -> i64 {
	let mut times_splitted = 0;
	let mut map = TileTypes::parse_map(input.trim());
	for row in 1..map.len() {
		for col in 0..map[row].len() {
			if map[row - 1][col] != TileTypes::Beam {
				continue;
			}
			if map[row][col] == TileTypes::Splitter {
				times_splitted += 1;
				if map[row][col - 1] == TileTypes::Empty {
					map[row][col - 1] = TileTypes::Beam;
				}
				if map[row][col + 1] == TileTypes::Empty {
					map[row][col + 1] = TileTypes::Beam;
				}
			} else {
				map[row][col] = TileTypes::Beam;
			}
		}
	}
	times_splitted
}

fn solution2(input: &str) -> i64 {
	(solution(input) - 1) * 2
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
