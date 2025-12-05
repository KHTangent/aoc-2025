use std::fs;

#[derive(Debug)]
struct Range {
	min: i64,
	max: i64,
}
impl Range {
	fn new(min: i64, max: i64) -> Self {
		Range { min, max }
	}
	fn has(&self, val: i64) -> bool {
		val >= self.min && val <= self.max
	}
	fn overlaps(&self, other: &Range) -> bool {
		(self.max >= other.min && self.min <= other.min)
			|| (other.max >= self.min && other.min <= self.min)
	}
	fn combine(&self, other: &Range) -> Range {
		Range::new(self.min.min(other.min), self.max.max(other.max))
	}
	fn size(&self) -> i64 {
		self.max - self.min + 1
	}
}

fn solution(input: &String) -> i64 {
	let (fresh_ranges, ingredient_list) = input.split_once("\n\n").unwrap();
	let mut ranges: Vec<Range> = vec![];
	for raw_range in fresh_ranges.lines() {
		let (min, max) = raw_range.split_once("-").unwrap();
		ranges.push(Range::new(min.parse().unwrap(), max.parse().unwrap()));
	}
	ingredient_list
		.lines()
		.map(|v| v.parse::<i64>().unwrap())
		.filter(|&v| ranges.iter().any(|r| r.has(v)))
		.count() as i64
}

fn solution2(input: &String) -> i64 {
	let (fresh_ranges, _) = input.split_once("\n\n").unwrap();
	let mut ranges: Vec<Range> = vec![];
	for raw_range in fresh_ranges.lines() {
		let (min, max) = raw_range.split_once("-").unwrap();
		ranges.push(Range::new(min.parse().unwrap(), max.parse().unwrap()));
	}
	ranges.sort_by_key(|r| r.min);
	let mut i = 0;
	while i < ranges.len() - 1 {
		let mut j = i + 1;
		let mut changes = false;
		while j < ranges.len() {
			if ranges[i].overlaps(&ranges[j]) {
				let removed = ranges.remove(j);
				ranges[i] = ranges[i].combine(&removed);
				changes = true;
			} else {
				j += 1;
			}
		}
		if !changes {
			i += 1;
		}
	}
	ranges.iter().map(|r| r.size()).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

	#[test]
	fn test_solution() {
		let answer = solution(&TEST_INPUT.to_string());
		assert_eq!(answer, 3);
	}

	#[test]
	fn test_solution2() {
		let answer = solution2(&TEST_INPUT.to_string());
		assert_eq!(answer, 14);
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
