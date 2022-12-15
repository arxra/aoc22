use std::{os::unix::prelude::FileExt, str::Lines};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

type Input = Vec<Vec<Tree>>;

#[derive(Debug)]
struct Tree {
    dirs: Vec<usize>,
    value: usize,
}

impl Tree {
    fn new(value: usize) -> Self {
        Self {
            value,
            dirs: vec![0; 4],
        }
    }
}

fn get_input(file_name: &str) -> Input {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|a| Tree::new(a as usize))
                .collect()
        })
        .collect()
}

fn solve_p1(file_name: &str) -> usize {
    let mut input = get_input(file_name);
    let x = input.len();
    let y = input[0].len();
    let mut visible = 2 * x + 2 * y - 2 * 2;

    (1..y - 1).cartesian_product(1..x - 1).for_each(|(y, x)| {
        input[x][y].dirs[0] = input[x][y - 1].value.max(input[x][y - 1].dirs[0]);
        input[x][y].dirs[1] = input[x - 1][y].value.max(input[x - 1][y].dirs[1]);
    });
    (1..y - 1)
        .rev()
        .cartesian_product((1..x - 1).rev())
        .for_each(|(y, x)| {
            input[x][y].dirs[2] = input[x + 1][y].value.max(input[x + 1][y].dirs[2]);
            input[x][y].dirs[3] = input[x][y + 1].value.max(input[x][y + 1].dirs[3]);
            if (input[x][y]
                .dirs
                .iter()
                .filter(|a| **a < input[x][y].value)
                .count()
                > 0)
            {
                visible += 1;
            }
        });
    visible
}

fn solve_p2(file_name: &str) -> usize {
    let mut top_scene = 0;
    let mut input = get_input(file_name);
    let xlen = input.len();
    let ylen = input[0].len();

    // we don't need to consider edges as theri scenic product is always 0.
    (1..ylen - 1)
        .cartesian_product(1..xlen - 1)
        .for_each(|(x, y)| {
            let mut res = (0..x)
                .rev()
                .fold_while(0, |acc, xx| {
                    if input[xx][y].value < input[x][y].value {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner()
                * (x + 1..xlen)
                    .fold_while(0, |acc, xx| {
                        if input[xx][y].value < input[x][y].value {
                            Continue(acc + 1)
                        } else {
                            Done(acc + 1)
                        }
                    })
                    .into_inner()
                * (0..y)
                    .rev()
                    .fold_while(0, |acc, yy| {
                        if input[x][yy].value < input[x][y].value {
                            Continue(acc + 1)
                        } else {
                            Done(acc + 1)
                        }
                    })
                    .into_inner()
                * (y + 1..ylen)
                    .fold_while(0, |acc, yy| {
                        if input[x][yy].value < input[x][y].value {
                            Continue(acc + 1)
                        } else {
                            Done(acc + 1)
                        }
                    })
                    .into_inner();

            top_scene = top_scene.max(res);
        });
    top_scene
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 21;
        let result = solve_p1("small/08.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 8;
        let result = solve_p2("small/08.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        assert_eq!(solve_p1("input/08.txt"), 1827);
    }
    #[test]
    fn p2() {
        assert_eq!(solve_p2("input/08.txt"), 335580);
    }
}
