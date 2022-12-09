use std::collections::HashSet;

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
        match (other.y, other.x) {
            (y, x) if x > self.x + 1 => {
                self.x = x - 1;
                self.y = y;
            }
            (y, x) if y > self.y + 1 => {
                self.y = y - 1;
                self.x = x;
            }
            (y, x) if x < self.x - 1 => {
                self.x = x + 1;
                self.y = y;
            }
            (y, x) if y < self.y - 1 => {
                self.y = y + 1;
                self.x = x;
            }
            _ => (),
        };
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

fn solve_p1(file_name: &str, size: usize) -> usize {
    let mut map = HashSet::new();
    let mut head = Knot::new(0, 0);
    let travels = get_input(file_name);
    let mut knots = vec![Knot::new(0, 0); size];

    travels.into_iter().for_each(|(dir, len)| {
        for _ in (0..len) {
            match dir {
                'R' => head.x += 1,
                'U' => head.y += 1,
                'L' => head.x -= 1,
                'D' => head.y -= 1,
                _ => unreachable!(),
            };
            knots[0].update(&head);
            for i in (1..size) {
                let previous = knots[i - 1];
                knots[i].update(&previous);
            }
            map.insert(knots[size - 1]);
            println!("{map:?}");
        }
    });

    map.len()
}

fn solve_p2(file_name: &str) -> usize {
    solve_p1(file_name, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 13;
        let result = solve_p1("small/09.txt", 1);
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
        let expected = 36;
        let result = solve_p2("small/09_2.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        dbg!(solve_p1("input/09.txt", 1));
    }
    #[test]
    fn p2() {
        panic!();
        // too high: 11955
        let sol = solve_p2("input/09.txt");
        assert!(sol < 11955);
        assert!(sol > 2190);

        dbg!(sol);
    }
}
