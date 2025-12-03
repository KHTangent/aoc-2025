use std::fs;

fn largest_subset_of_n(line: &str, digits: usize) -> i64 {
	let mut total: i64 = 0;
	let mut to_skip = 0;
	for skip_end in (0..digits).rev() {
		let (digit_index, digit) = line
			.chars()
			.take(line.len() - skip_end)
			.enumerate()
			.skip(to_skip)
			.fold((0, '0'), |acc, el| if acc.1 < el.1 { el } else { acc });
		let digit = digit.to_digit(10).unwrap() as i64;
		total = total * 10 + digit;
		to_skip = digit_index + 1;
	}
	total
}

fn solution(input: &String) -> i64 {
	let mut sum: i64 = 0;
	for line in input.lines() {
		sum += largest_subset_of_n(line, 2);
	}
	sum
}

fn solution2(input: &String) -> i64 {
	let mut sum: i64 = 0;
	for line in input.lines() {
		sum += largest_subset_of_n(line, 12);
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"987654321111111
811111111111119
234234234234278
818181911112111
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 357);
	}

	#[test]
	#[ignore]
	fn test_solution2() {
		let input = String::from(
			r"987654321111111
811111111111119
234234234234278
818181911112111
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 3121910778619);
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
