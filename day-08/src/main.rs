use std::fs;

#[derive(Debug)]
struct Junction {
	pos: (i64, i64, i64),
	circuit: usize,
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
			circuit: 0,
		}
	}
	fn distance_to(&self, other: &Junction) -> f64 {
		let dx = (self.pos.0 as f64) - (other.pos.0 as f64);
		let dy = (self.pos.1 as f64) - (other.pos.1 as f64);
		let dz = (self.pos.2 as f64) - (other.pos.2 as f64);
		f64::sqrt(dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0))
	}
}

fn solution(input: &str, connections: usize) -> i64 {
	let mut junctions: Vec<Junction> = input.lines().map(Junction::from_str).collect();
	let mut all_distances: Vec<(usize, usize, f64)> = Vec::with_capacity(junctions.len().pow(2));
	for i in 0..junctions.len() {
		junctions[i].circuit = i;
	}

	for i in 0..junctions.len() {
		all_distances.extend(
			junctions
				.iter()
				.enumerate()
				.filter(|(index, _)| i != *index)
				.map(|(index, j)| (i, index, junctions[i].distance_to(&j))),
		);
	}
	all_distances.sort_by(|a, b| a.2.total_cmp(&b.2));

	let mut circuits_device_count: Vec<i64> = vec![1; junctions.len()];
	let mut connections_made: Vec<(usize, usize)> = Vec::with_capacity(connections);
	for (i, j, _) in all_distances {
		if connections_made.len() >= connections {
			break;
		}
		if connections_made.contains(&(i, j)) || connections_made.contains(&(j, i)) {
			continue;
		}
		let c1 = junctions[i].circuit;
		let c2 = junctions[j].circuit;
		if c1 != c2 {
			let mut changed = 0;
			for k in 0..junctions.len() {
				if junctions[k].circuit == c2 {
					junctions[k].circuit = c1;
					changed += 1;
				}
			}
			circuits_device_count[c2] = 0;
			circuits_device_count[c1] += changed;
		}
		connections_made.push((i, j));
	}

	circuits_device_count.sort();
	circuits_device_count.iter().rev().take(3).product()
}

fn solution2(input: &str) -> i64 {
	let mut junctions: Vec<Junction> = input.lines().map(Junction::from_str).collect();
	let mut all_distances: Vec<(usize, usize, f64)> = Vec::with_capacity(junctions.len().pow(2));
	for i in 0..junctions.len() {
		junctions[i].circuit = i;
	}

	for i in 0..junctions.len() {
		all_distances.extend(
			junctions
				.iter()
				.enumerate()
				.filter(|(index, _)| i != *index)
				.map(|(index, j)| (i, index, junctions[i].distance_to(&j))),
		);
	}
	all_distances.sort_by(|a, b| a.2.total_cmp(&b.2));

	let mut circuits_device_count: Vec<i64> = vec![1; junctions.len()];
	let mut connections_made: Vec<(usize, usize)> = vec![];
	for (i, j, _) in all_distances {
		if connections_made.contains(&(i, j)) || connections_made.contains(&(j, i)) {
			continue;
		}
		let c1 = junctions[i].circuit;
		let c2 = junctions[j].circuit;
		if c1 != c2 {
			let mut changed = 0;
			for k in 0..junctions.len() {
				if junctions[k].circuit == c2 {
					junctions[k].circuit = c1;
					changed += 1;
				}
			}
			circuits_device_count[c2] = 0;
			circuits_device_count[c1] += changed;
		}
		connections_made.push((i, j));
		if *circuits_device_count.iter().max().unwrap() == junctions.len() as i64 {
			break;
		}
	}
	let last_connection = connections_made[connections_made.len() - 1];
	junctions[last_connection.0].pos.0 * junctions[last_connection.1].pos.0
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
	fn test_solution2() {
		let answer = solution2(TEST_INPUT);
		assert_eq!(answer, 25272);
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
