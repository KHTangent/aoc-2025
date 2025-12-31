use std::fs;

fn solution(input: &str) -> i64 {
	let points: Vec<(i64, i64)> = input
		.lines()
		.map(|s| s.split_once(",").unwrap())
		.map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
		.collect();
	let mut all_rectangles: Vec<i64> = Vec::with_capacity(points.len().pow(2) - points.len());
	for i in 0..points.len() {
		let p1 = &points[i];
		all_rectangles.extend(
			points
				.iter()
				.enumerate()
				.filter(|(index, _)| i != *index)
				.map(|(_, p2)| (p1.0 - p2.0 - 1).abs() * (p1.1 - p2.1 - 1).abs()),
		);
	}
	*all_rectangles.iter().max().unwrap()
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

	const TEST_INPUT: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

	#[test]
	fn test_solution() {
		let answer = solution(TEST_INPUT);
		assert_eq!(answer, 50);
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
