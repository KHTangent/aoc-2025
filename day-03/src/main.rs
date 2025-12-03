use std::fs;

fn solution(input: &String) -> i64 {
	let mut sum: i64 = 0;
	for line in input.lines() {
		let (first_digit_index, first_digit) = line
			.chars()
			.take(line.len() - 1)
			.enumerate()
			.fold((0, '0'), |acc, el| if acc.1 < el.1 { el } else { acc });
		let largest_after_first_digit = line
			.chars()
			.skip(first_digit_index + 1)
			.fold('0', |acc, el| el.max(acc));
		let jolts = format!("{}{}", first_digit, largest_after_first_digit)
			.parse::<i64>()
			.unwrap();
		sum += jolts;
	}
	sum
}

fn solution2(input: &String) -> i64 {
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
