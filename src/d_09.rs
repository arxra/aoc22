use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Knot {
    y: isize,
    x: isize,
}

impl Knot {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
    fn update(&mut self, other: &Self) {
        // Should we move?
        if (self.y - other.y).abs() < 2 && (self.x - other.x).abs() < 2 {
            return;
        }
        // How should we move?
        match (other.y, other.x) {
            // along x-axis
            (y, x) if x == self.x && y > self.y => self.y += 1,
            (y, x) if x == self.x && y < self.y => self.y -= 1,
            // along y-axis
            (y, x) if y == self.y && x > self.x => self.x += 1,
            (y, x) if y == self.y && x < self.x => self.x -= 1,
            // along the Diagonal
            (y, x) => {
                // along which of the 4 diagonals should we move?
                let dy = self.y - y;
                let dx = self.x - x;
                if dx > 0 {
                    self.x -= 1;
                } else {
                    self.x += 1;
                }
                if dy > 0 {
                    self.y -= 1;
                } else {
                    self.y += 1;
                }
            }
        };
    }
}

#[allow(unused)]
fn draw_rope(rope: &[Knot], dim: isize) {
    let mut v = HashMap::new();
    for (i, knot) in rope.iter().enumerate() {
        v.insert((knot.y, knot.x), i);
    }
    println!("########################");
    for c in (-dim..dim).rev().cartesian_product(-dim..dim) {
        if let Some(i) = v.get(&c) {
            if *i == 0 {
                print!("H");
            } else {
                print!("{i}");
            }
        } else {
            print!(".");
        }
        if c.1 == dim - 1 {
            println!();
        }
    }
}

fn get_input(file_name: &str) -> Vec<(char, isize)> {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
            let (c, mov) = l.split_at(2);
            let c = c.chars().into_iter().next().unwrap();
            let mov = mov.parse().unwrap();
            (c, mov)
        })
        .collect()
}

#[allow(dead_code)]
fn solve(file_name: &str, size: usize) -> usize {
    let mut map = HashSet::new();
    let travels = get_input(file_name);
    let mut knots = vec![Knot::new(0, 0); size];

    travels.into_iter().for_each(|(dir, steps)| {
        for _ in (0..steps) {
            match dir {
                'R' => knots[0].x += 1,
                'U' => knots[0].y += 1,
                'L' => knots[0].x -= 1,
                'D' => knots[0].y -= 1,
                _ => unreachable!(),
            };
            for i in (1..size) {
                let previous = knots[i - 1];
                knots[i].update(&previous);
            }
            map.insert(knots[size - 1]);
            //draw_rope(&knots, 5);
        }
    });

    map.len()
}

#[allow(dead_code)]
fn solve_p2(file_name: &str) -> usize {
    solve(file_name, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 13;
        let result = solve("small/09.txt", 2);
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 1;
        let result = solve_p2("small/09.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_2_exp() {
        let expected = 37;
        let result = solve_p2("small/09_2.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        assert_eq!(solve("input/09.txt", 2), 5878);
    }
    #[test]
    fn p2() {
        // too high: 11955
        let sol = solve_p2("input/09.txt");
        assert!(sol < 11955);
        assert!(sol > 2190);
        assert_eq!(sol, 2405);
    }
}
