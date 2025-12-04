use std::fs;

fn parse_map(input: &String) -> Vec<Vec<bool>> {
	let line_length = input.find("\n").unwrap();
	let mut map: Vec<Vec<bool>> = vec![];
	map.push(vec![false; line_length + 2]);
	for (i, line) in input.lines().enumerate() {
		map.push(Vec::with_capacity(line_length + 2));
		let new_line = &mut map[i + 1];
		let mut a = line
			.chars()
			.map(|c| match c {
				'@' => true,
				_ => false,
			})
			.collect::<Vec<bool>>();
		new_line.push(false);
		new_line.append(&mut a);
		new_line.push(false);
	}
	map.push(vec![false; line_length + 2]);
	map
}

fn get_tiles_around(map: &Vec<Vec<bool>>, row: usize, col: usize) -> Vec<bool> {
	vec![
		map[row - 1][col - 1],
		map[row - 1][col],
		map[row - 1][col + 1],
		map[row][col - 1],
		map[row][col + 1],
		map[row + 1][col - 1],
		map[row + 1][col],
		map[row + 1][col + 1],
	]
}

fn solution(input: &String) -> i64 {
	let map = parse_map(input);
	let mut sum = 0;
	for row in 1..(map.len() - 1) {
		for col in 1..(map[0].len() - 1) {
			if !map[row][col] {
				continue;
			}
			let tiles_around = get_tiles_around(&map, row, col);
			if tiles_around.iter().filter(|&e| *e).count() < 4 {
				sum += 1;
			}
		}
	}
	sum
}

fn solution2(input: &String) -> i64 {
	let mut map = parse_map(input);
	let mut sum = 0;
	let mut to_remove: Vec<(usize, usize)> = vec![];
	loop {
		for row in 1..(map.len() - 1) {
			for col in 1..(map[0].len() - 1) {
				if !map[row][col] {
					continue;
				}
				let tiles_around = get_tiles_around(&map, row, col);
				if tiles_around.iter().filter(|&e| *e).count() < 4 {
					sum += 1;
					to_remove.push((row, col));
				}
			}
		}
		if to_remove.is_empty() {
			break;
		}
		for &(row, col) in to_remove.iter() {
			map[row][col] = false;
		}
		to_remove.clear();
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 13);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 43);
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
