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
	fn distance_to(&self, other: &Junction) -> f64 {
		f64::sqrt(
			(self.pos.0 - other.pos.0).pow(2) as f64
				+ (self.pos.1 - other.pos.1).pow(2) as f64
				+ (self.pos.2 - other.pos.2).pow(2) as f64,
		)
	}
}

fn solution(input: &str, connections: i64) -> i64 {
	let mut junctions: Vec<Junction> = input.lines().map(Junction::from_str).collect();
	let mut all_distances: Vec<(usize, usize, f64)> = Vec::with_capacity(junctions.len().pow(2));
	for i in 0..junctions.len() {
		all_distances.extend(
			junctions
				.iter()
				.enumerate()
				.filter(|(index, _)| i != *index)
				.map(|(index, j)| (i, index, j.distance_to(&junctions[i]))),
		);
	}
	let mut circuits_device_count: Vec<i64> = vec![];
	all_distances.sort_by(|a, b| a.2.total_cmp(&b.2));
	let mut taken = 0;
	for (i, j, _) in all_distances {
		if connections == taken {
			break;
		}
		taken += 1;
		match (junctions[i].circuit, junctions[j].circuit) {
			(Some(s1), Some(s2)) => {
				if s1 == s2 {
					taken -= 1;
					continue;
				}
				let mut changed = 0;
				for k in 0..junctions.len() {
					if junctions[k].circuit == Some(s2) {
						junctions[k].circuit = Some(s1);
						changed += 1;
					}
				}
				circuits_device_count[s2] = 0;
				circuits_device_count[s1] += changed - 1;
			}
			(None, None) => {
				junctions[i].circuit = Some(circuits_device_count.len());
				junctions[j].circuit = Some(circuits_device_count.len());
				circuits_device_count.push(2);
			}
			(None, Some(s)) => {
				junctions[i].circuit = Some(s);
				circuits_device_count[s] += 1;
			}
			(Some(s), None) => {
				junctions[j].circuit = Some(s);
				circuits_device_count[s] += 1;
			}
		}
	}
	circuits_device_count.sort();
	for j in junctions.iter() {
		println!("{:?}", j);
	}
	println!("{:?}", circuits_device_count);
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
		let answer = solution(TEST_INPUT, 10);
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
	let answer = solution(&file, 1000);
	println!("Answer task 1: {}", answer);
	let answer = solution2(&file);
	println!("Answer task 2: {}", answer);
}
