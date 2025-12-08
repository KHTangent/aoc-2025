use std::fs;

#[derive(Debug)]
struct Junction {
	pos: (i64, i64, i64),
	circuit: Option<usize>,
}
impl Junction {
	fn from_str(s: &str) -> Self {
		let mut parts = s.split(",").map(|v| v.parse::<i64>().unwrap());
		Junction {
			pos: (
				parts.next().unwrap(),
				parts.next().unwrap(),
				parts.next().unwrap(),
			),
			circuit: None,
		}
	}
	fn distance_to(&self, other: &Junction) -> i64 {
		f64::sqrt(
			(self.pos.0 - other.pos.0).pow(2) as f64
				+ (self.pos.1 - other.pos.1).pow(2) as f64
				+ (self.pos.2 - other.pos.2).pow(2) as f64,
		)
		.round() as i64
	}
}

fn solution(input: &str) -> i64 {
	let mut junctions: Vec<Junction> = input.lines().map(Junction::from_str).collect();
	let mut circuits_device_count: Vec<i64> = vec![];
	for i in 0..(junctions.len() - 1) {
		if junctions[i].circuit.is_some() {
			continue;
		}
		let mut shortest_distance = i64::MAX;
		let mut shortest_distance_index = 0;
		for j in 0..junctions.len() {
			if i == j {
				continue;
			}
			let distance = junctions[i].distance_to(&junctions[j]);
			if distance < shortest_distance {
				shortest_distance = distance;
				shortest_distance_index = j;
			}
		}
		match junctions[shortest_distance_index].circuit {
			Some(circuit) => {
				junctions[i].circuit = Some(circuit);
				circuits_device_count[circuit] += 1;
			},
			None => {
				circuits_device_count.push(2);
				junctions[i].circuit = Some(circuits_device_count.len()-1);
				junctions[shortest_distance_index].circuit = Some(circuits_device_count.len()-1);
			},
		}
	}
	circuits_device_count.iter().rev().take(3).product()
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

	const TEST_INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

	#[test]
	fn test_solution() {
		let answer = solution(TEST_INPUT);
		assert_eq!(answer, 40);
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
