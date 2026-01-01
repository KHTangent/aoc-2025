use std::{collections::HashSet, fs};

type Point = (i64, i64);

fn parse_points(input: &str) -> Vec<Point> {
	input
		.lines()
		.map(|s| s.split_once(",").unwrap())
		.map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
		.collect()
}

fn solution(input: &str) -> i64 {
	let points = parse_points(input);
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

fn find_bounds(p1: &Point, p2: &Point) -> (Point, Point) {
	(
		(p1.0.min(p2.0), p1.1.min(p2.1)),
		(p1.0.max(p2.0), p1.1.max(p2.1)),
	)
}

fn mark_tiles(usable_tiles: &mut HashSet<Point>, p1: &Point, p2: &Point) {
	let ((start_x, start_y), (end_x, end_y)) = find_bounds(p1, p2);
	for x in start_x..(end_x + 1) {
		for y in start_y..(end_y + 1) {
			usable_tiles.insert((x, y));
		}
	}
}

fn solution2(input: &str) -> i64 {
	let points = parse_points(input);
	let mut usable_tiles: HashSet<Point> = HashSet::new();
	for i in 0..(points.len() - 1) {
		mark_tiles(&mut usable_tiles, &points[i], &points[i + 1]);
	}
	mark_tiles(&mut usable_tiles, &points[points.len() - 1], &points[0]);
	let mut min_point: Point = (i64::MAX, i64::MAX);
	let mut max_point: Point = (0, 0);
	for (x, y) in points.iter() {
		min_point.0 = min_point.0.min(*x);
		min_point.1 = min_point.1.min(*y);
		max_point.0 = max_point.0.max(*x);
		max_point.1 = max_point.1.max(*y);
	}

	for x in (min_point.0 + 1)..max_point.0 {
		for y in (min_point.1 + 1)..max_point.1 {
			if usable_tiles.contains(&(x, y)) {
				continue;
			}
			let mut found = false;
			for y1 in (y + 1)..(max_point.1 + 1) {
				if usable_tiles.contains(&(x, y1)) {
					found = true;
					break;
				}
			}
			if !found {
				continue;
			}
			found = false;
			for x1 in (x + 1)..(max_point.0 + 1) {
				if usable_tiles.contains(&(x1, y)) {
					found = true;
					break;
				}
			}
			if !found {
				continue;
			}
			found = false;
			for y1 in (min_point.1..y).rev() {
				if usable_tiles.contains(&(x, y1)) {
					found = true;
					break;
				}
			}
			if !found {
				continue;
			}
			found = false;
			for x1 in (min_point.0..x).rev() {
				if usable_tiles.contains(&(x1, y)) {
					found = true;
					break;
				}
			}
			if !found {
				continue;
			}

			usable_tiles.insert((x, y));
		}
	}

	let mut all_rectangles: Vec<(Point, Point, i64)> =
		Vec::with_capacity(points.len().pow(2) - points.len());
	for i in 0..points.len() {
		let p1 = &points[i];
		all_rectangles.extend(
			points
				.iter()
				.enumerate()
				.filter(|(index, _)| i != *index)
				.map(|(_, p2)| (*p1, *p2, (p1.0 - p2.0 - 1).abs() * (p1.1 - p2.1 - 1).abs())),
		);
	}
	all_rectangles.sort_unstable_by_key(|(_, _, a)| *a);
	'outer: for (p1, p2, area) in all_rectangles.into_iter().rev() {
		let ((start_x, start_y), (end_x, end_y)) = find_bounds(&p1, &p2);
		for x in start_x..(end_x + 1) {
			for y in start_y..(end_y + 1) {
				if !usable_tiles.contains(&(x, y)) {
					continue 'outer;
				}
			}
		}
		return area;
	}
	0
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
	fn test_solution2() {
		let answer = solution2(TEST_INPUT);
		assert_eq!(answer, 24);
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
