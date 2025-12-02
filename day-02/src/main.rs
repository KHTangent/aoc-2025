use std::fs;

fn solution(input: &String) -> i64 {
	let mut total = 0;
	for range in input.trim().split(",") {
		let (range_start, range_end) = range
			.split_once("-")
			.and_then(|(start, end)| {
				Some((start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()))
			})
			.unwrap();
		for number in range_start..(range_end + 1) {
			let number_string = number.to_string();
			let number_len = number_string.len();
			if number_string.starts_with("0") {
				total += number;
				continue;
			}
			if number_len % 2 == 1 {
				continue;
			}
			let (first_half, second_half) = number_string.split_at(number_len / 2);
			if first_half == second_half {
				total += number;
			}
		}
	}
	total
}

fn solution2(input: &String) -> i64 {
	let mut total = 0;
	for range in input.trim().split(",") {
		let (range_start, range_end) = range
			.split_once("-")
			.and_then(|(start, end)| {
				Some((start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()))
			})
			.unwrap();
		for number in range_start..(range_end + 1) {
			let number_string = number.to_string();
			let number_len = number_string.len();
			for i in 1..(number_len / 2 + 1) {
				if number_len % i != 0 {
					continue;
				}
				let sub = &number_string[0..i];
				let occurances = number_string.match_indices(sub).count();
				if occurances == number_len / sub.len() {
					total += number;
					break;
				}
			}
		}
	}
	total
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
		);
		let answer = solution(&input);
		assert_eq!(answer, 1227775554);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 4174379265);
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
