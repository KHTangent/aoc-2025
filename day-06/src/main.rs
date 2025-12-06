use std::fs;

enum Operands {
	ADD,
	MULTIPLY,
}
impl Operands {
	fn from_str(s: &str) -> Self {
		match s {
			"+" => Operands::ADD,
			"*" => Operands::MULTIPLY,
			_ => {
				println!("Got {}", s);
				panic!()
			}
		}
	}
}

fn solution(input: &str) -> i64 {
	let mut sum = 0;
	let mut inputs: Vec<Vec<i64>> = vec![];
	let mut operands: Vec<Operands> = vec![];
	for line in input.lines() {
		let is_number_line = line.trim_start().chars().next().unwrap().is_digit(10);
		if is_number_line {
			inputs.push(
				line.split_whitespace()
					.map(|v| v.parse().unwrap())
					.collect(),
			);
		} else {
			operands.extend(line.split_whitespace().map(Operands::from_str));
		}
	}
	let num_tasks = operands.len();
	let numbers_per_task = inputs.len();
	for i in 0..num_tasks {
		match operands[i] {
			Operands::ADD => {
				let problem_sum = (0..numbers_per_task)
					.map(|j| inputs[j][i])
					.fold(0, |prev, v| prev + v);
				sum += problem_sum;
			}
			Operands::MULTIPLY => {
				let problem_product = (0..numbers_per_task)
					.map(|j| inputs[j][i])
					.fold(1, |prev, v| prev * v);
				sum += problem_product;
			}
		}
	}
	sum
}

fn solution2(input: &str) -> i64 {
	let mut sum = 0;
	let mut numbers_lines_raw: Vec<Vec<char>> = vec![];
	let mut operands: Vec<Operands> = vec![];
	for line in input.lines() {
		let is_number_line = line.trim_start().chars().next().unwrap().is_digit(10);
		if is_number_line {
			numbers_lines_raw.push(line.chars().collect());
		} else {
			operands.extend(line.split_whitespace().map(Operands::from_str));
		}
	}
	let numbers_length = numbers_lines_raw.len();
	let mut number_lines_index = numbers_lines_raw[0].len();
	for operand in operands.into_iter().rev() {
		let mut numbers: Vec<i64> = vec![];
		loop {
			number_lines_index -= 1;
			let combined: String = (0..numbers_length)
				.map(|j| numbers_lines_raw[j][number_lines_index])
				.collect();
			let trimmed = combined.trim();
			if trimmed.is_empty() {
				break;
			} else {
				numbers.push(trimmed.parse().unwrap());
			}
			if number_lines_index == 0 {
				break;
			}
		}
		match operand {
			Operands::ADD => {
				let problem_sum = numbers.into_iter().fold(0, |prev, v| prev + v);
				sum += problem_sum;
			}
			Operands::MULTIPLY => {
				let problem_product = numbers.into_iter().fold(1, |prev, v| prev * v);
				sum += problem_product;
			}
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

	#[test]
	fn test_solution() {
		let answer = solution(TEST_INPUT);
		assert_eq!(answer, 4277556);
	}

	#[test]
	fn test_solution2() {
		let answer = solution2(TEST_INPUT);
		assert_eq!(answer, 3263827);
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
