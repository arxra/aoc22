use std::cmp::Ordering;
use std::fmt::Display;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Self>),
    Num(usize),
}

impl Packet {
    fn new(inp: &mut Vec<char>) -> Self {
        // pop goes backwards, but that is fine!
        match inp.pop() {
            Some(']') => {
                let mut v = Vec::new();
                while *inp.last().unwrap() != '[' {
                    v.push(Self::new(inp));
                }
                inp.pop(); // pop the [
                if let Some(',') = inp.last() {
                    inp.pop();
                }
                v.reverse();
                Self::List(v)
            }
            Some(a) if ('0'..='9').contains(&a) => {
                let mut num = format!("{a}");
                if *inp.last().unwrap() == '1' {
                    num.insert(0, inp.pop().unwrap());
                }
                if *inp.last().unwrap() == ',' {
                    inp.pop();
                }
                Self::Num(num.parse().unwrap())
            }
            _ => unreachable!(),
        }
    }
    fn other_smaller(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a
                .iter()
                .zip_longest(b.iter())
                .fold_while(None, |acc, eob| match eob {
                    itertools::EitherOrBoth::Both(a, b) => {
                        let res = a.other_smaller(b);
                        if res.is_some() {
                            Done(res)
                        } else {
                            Continue(acc)
                        }
                    }
                    itertools::EitherOrBoth::Right(_) => Done(Some(true)),
                    itertools::EitherOrBoth::Left(_) => Done(Some(false)),
                })
                .into_inner(),
            (a @ Self::List(_), b @ Self::Num(_)) => a.other_smaller(&Self::List(vec![b.clone()])),
            (a @ Self::Num(_), b @ Self::List(_)) => {
                Self::List(vec![a.to_owned()]).other_smaller(b)
            }
            (Self::Num(a), Self::Num(b)) if a < b => Some(true),
            (Self::Num(a), Self::Num(b)) if a > b => Some(false),
            (Self::Num(a), Self::Num(b)) if a == b => None, // In case they are the same
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.other_smaller(other) {
            None => Some(Ordering::Equal),
            Some(true) => Some(Ordering::Less),
            Some(false) => Some(Ordering::Greater),
        }
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.other_smaller(other) {
            None => Ordering::Equal,
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(l) => {
                f.write_str("[")?;
                l.iter().for_each(|a| a.fmt(f).unwrap());
                f.write_str("]")
            }
            Self::Num(n) => f.write_str(&format!("{n}")),
        }
    }
}

fn parse_inp(file_name: &str) -> Vec<(Packet, Packet)> {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .filter_map(|a| {
            if a.is_empty() {
                None
            } else {
                Some(Packet::new(&mut a.to_owned().chars().collect()))
            }
        })
        .tuples()
        .collect()
}

#[allow(dead_code)]
fn solve_p1(file_name: &str) -> usize {
    let inp = parse_inp(file_name);
    let smaller_set = inp
        .into_iter()
        .map(|(a, b)| a.other_smaller(&b))
        .enumerate();
    smaller_set
        .filter_map(|(i, b)| {
            if let Some(true) = b {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
fn solve_p2(file_name: &str) -> usize {
    let inp = parse_inp(file_name);
    let index_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Num(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Num(6)])]),
    ];
    let (a, b): (Vec<Packet>, Vec<Packet>) = inp.into_iter().unzip();
    a.iter()
        .chain(b.iter())
        .chain(index_packets.iter())
        .sorted()
        .enumerate()
        .filter(|(_, a)| index_packets.contains(a))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 13;
        let result = solve_p1("small/13.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 140;
        let result = solve_p2("small/13.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        let res = solve_p1("input/13.txt");
        assert!(res > 3885);
        assert!(res < 5768);
        assert_eq!(res, 5623);
    }
    #[test]
    fn p2() {
        let sol = solve_p2("input/13.txt");
        assert_eq!(sol, 20570);
    }
}
