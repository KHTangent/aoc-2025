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
		let is_number_line = line.chars().next().unwrap().is_digit(10);
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
	for line in input.lines() {
		let num = line.parse::<i64>().unwrap_or(0);
		sum += num;
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
	#[ignore]
	fn test_solution2() {
		let answer = solution2(TEST_INPUT);
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
