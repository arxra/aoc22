use std::fmt::{Display, Write};

use itertools::Itertools;

type Input = Vec<Vec<(usize, usize)>>;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Elem {
    Sand,
    Rock,
    Air,
}

impl Elem {
    fn char(&self) -> char {
        match self {
            Elem::Sand => 'O',
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
        self.tiles.iter().for_each(|y| {
            y.iter()
                .skip(300)
                .take(400)
                .for_each(|x| f.write_char(x.char()).unwrap());
            f.write_str("\n").unwrap();
        });
        Ok(())
    }
}

impl Cave {
    fn new(tiles: Vec<Vec<Elem>>) -> Self {
        Self { tiles }
    }

    fn add_floor(&mut self) {
        self.tiles.push(vec![Elem::Air; 1000]);
        self.tiles.push(vec![Elem::Rock; 1000]);
    }

    fn from_input(input: Input) -> Self {
        let mut tiles = vec![Vec::new()];
        input.iter().for_each(|line| {
            line.iter()
                .tuple_windows()
                .map(|((x, y), (xx, yy)): (&(usize, usize), &(usize, usize))| {
                    if *x == *xx {
                        (if y < yy { *y..=*yy } else { *yy..=*y })
                            .map(|y| (*x, y))
                            .collect()
                    } else {
                        (if x < xx { *x..=*xx } else { *xx..=*x })
                            .map(|x| (x, *y))
                            .collect()
                    }
                })
                .for_each(|p: Vec<(usize, usize)>| {
                    p.into_iter().for_each(|(x, y)| {
                        while tiles.len() <= y {
                            tiles.push(vec![Elem::Air; 1000])
                        }
                        tiles[y][x] = Elem::Rock
                    })
                })
        });
        Self::new(tiles)
    }

    fn drop_sand(&mut self) -> bool {
        let mut pos = (500, 0);
        let mut old_pos = (500, 2);

        while old_pos != pos {
            if let Some(new_pos) = self.new_sand_pos(pos) {
                if new_pos == (500, 0) {
                    return false;
                }
                old_pos = pos;
                pos = new_pos;
            } else {
                return false;
            }
        }
        self.tiles[pos.1][pos.0] = Elem::Sand;
        true
    }
    fn new_sand_pos(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        // println!("Sand falls: ({x},{y})");
        // Sand falls outside of Cave
        if y + 1 == self.tiles.len() {
            None
        } else if self.tiles[y + 1][x] == Elem::Air {
            // Sand falls straight down
            Some((x, y + 1))
        } else if self.tiles[y + 1][x - 1] == Elem::Air {
            // Sand falls down left
            Some((x - 1, y + 1))
        } else if self.tiles[y + 1][x + 1] == Elem::Air {
            //Sand falls down right
            Some((x + 1, y + 1))
        } else {
            // Sand falls not at all
            Some((x, y))
        }
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
    let mut cave = Cave::from_input(input);
    let mut dropped_sand = 0;
    while cave.drop_sand() {
        // println!("{cave}");
        dropped_sand += 1;
    }
    dropped_sand
}

#[allow(dead_code)]
fn solve_p2(file_name: &str) -> usize {
    let input = parse_inp(file_name);
    let mut cave = Cave::from_input(input);
    cave.add_floor();
    let mut dropped_sand = 1;
    while cave.drop_sand() {
        dropped_sand += 1;
    }
    println!("{cave}");
    dropped_sand
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
        let expected = 93;
        let result = solve_p2("small/14.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        let res = solve_p1("input/14.txt");
        assert_eq!(res, 763);
    }
    #[test]
    fn p2() {
        let sol = solve_p2("input/14.txt");
        assert_eq!(sol, 23921);
    }
}
