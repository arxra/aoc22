use std::fmt::{Display, Write};

use itertools::Itertools;

type Input = Vec<Vec<(usize, usize)>>;

#[derive(Debug)]
enum Elem {
    Sand,
    Rock,
    Air,
}

impl Elem {
    fn char(&self) -> char {
        match self {
            Elem::Sand => 'o',
            Elem::Rock => '#',
            Elem::Air => '.',
        }
    }
}

impl Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.char())
    }
}

#[derive(Debug)]
struct Cave {
    tiles: Vec<Vec<Elem>>,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles
            .iter()
            .map(|y| {
                y.iter().map(|x| f.write_char(x.char()));
                f.write_str("\n")
            })
            .next()
            .unwrap()
    }
}

impl Cave {
    fn from_input(input: Input) -> Self {
        let mut tiles = vec![Vec::new()];
        input.iter().for_each(|line| {line.iter().tuple_windows().for_each(|((x,y),(xx,yy))|{
            if x < xx
        })})
    }
}

fn parse_inp(file_name: &str) -> Input {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|a| {
            a.split(" -> ")
                .map(|w| {
                    w.split(',')
                        .flat_map(|t| t.parse::<usize>())
                        .tuples::<(usize, usize)>()
                        .next()
                        .unwrap()
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

#[allow(dead_code)]
fn solve_p1(file_name: &str) -> usize {
    let input = parse_inp(file_name);
    unimplemented!()
}

#[allow(dead_code)]
fn solve_p2(file_name: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 24;
        let result = solve_p1("small/14.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 140;
        let result = solve_p2("small/14.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        let res = solve_p1("input/14.txt");
        assert!(res > 3885);
        assert!(res < 5768);
        assert_eq!(res, 5623);
    }
    #[test]
    fn p2() {
        let sol = solve_p2("input/14.txt");
        assert_eq!(sol, 20570);
    }
}
