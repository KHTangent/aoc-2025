use std::fs;

fn solution(input: &String) -> i64 {
	let mut sum = 0;
	let mut current_position = 50;
	let num_positions = 100;
	for line in input.lines() {
		let (direction, number) = line.split_at(1);
		let mut number = number.parse::<i64>().unwrap_or(0);
		number %= num_positions;
		if direction == "L" {
			number = num_positions - number;
		}
		current_position = (current_position + number) % num_positions;
		if current_position == 0 {
			sum += 1;
		}
	}
	sum
}

fn solution2(input: &String) -> i64 {
	let mut sum = 0;
	let mut current_position = 50;
	let num_positions = 100;
	for line in input.lines() {
		let (direction, number) = line.split_at(1);
		let number = number.parse::<i64>().unwrap_or(0);
		for _ in 0..number {
			let move_by = match direction {
				"L" => -1,
				"R" => 1,
				_ => panic!(),
			};
			current_position = (current_position + move_by) % num_positions;
			if current_position == 0 {
				sum += 1;
			}
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 3);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 6);
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
