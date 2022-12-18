#![allow(dead_code)]
use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug)]
enum Op {
    Mul(usize),
    Plus(usize),
    MulSel,
    PlusSel,
}

impl Op {
    fn from_str(st: &[&str]) -> Self {
        match (st[0], st[1].parse::<usize>()) {
            ("*", Ok(a)) => Op::Mul(a),
            ("+", Ok(a)) => Op::Plus(a),
            ("*", _) => Op::MulSel,
            ("+", _) => Op::PlusSel,
            a => {
                println!("Got {a:?} in op");
                unreachable!()
            }
        }
    }
    fn apply(&self, old: usize) -> usize {
        match &self {
            Op::Mul(a) => old * a,
            Op::Plus(a) => old + a,
            Op::MulSel => old * old,
            Op::PlusSel => old + old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    test: usize,
    target_true: usize,
    target_false: usize,
    inspected_items: usize,
}

impl Monkey {
    fn consume_new(inp: &mut VecDeque<String>) -> Self {
        let items = inp
            .pop_front()
            .unwrap()
            .split(&[' ', ','])
            .flat_map(|a| a.parse::<usize>())
            .collect();
        let op = Op::from_str(&(inp.pop_front().unwrap().split(' ').collect::<Vec<&str>>())[6..=7]);
        let test = get_last_usize(inp);
        let target_true = get_last_usize(inp);
        let target_false = get_last_usize(inp);
        let inspected_items = 0;

        Monkey {
            items,
            op,
            test,
            target_true,
            target_false,
            inspected_items,
        }
    }
}

fn get_last_usize(inp: &mut VecDeque<String>) -> usize {
    inp.pop_front()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn get_input(file_name: &str) -> Vec<Monkey> {
    let mut inp: VecDeque<String> = std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|a| a.to_owned())
        .collect();
    let mut monkeys = Vec::new();
    // Pops the monkey with index
    while inp.pop_front().is_some() {
        monkeys.push(Monkey::consume_new(&mut inp));
        inp.pop_front();
    }
    monkeys
}

fn get_top_prod(monkeys: Vec<Monkey>) -> usize {
    monkeys
        .into_iter()
        .map(|a| a.inspected_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn solve_p1(file_name: &str) -> usize {
    let mut monkeys = get_input(file_name);
    for _ in 0..20 {
        for mnkid in 0..monkeys.len() {
            monkeys[mnkid].inspected_items += monkeys[mnkid].items.len();
            for i in 0..monkeys[mnkid].items.len() {
                let new_i = monkeys[mnkid].op.apply(monkeys[mnkid].items[i]) / 3;
                if new_i % monkeys[mnkid].test == 0 {
                    let target = monkeys[mnkid].target_true;
                    monkeys[target].items.push(new_i);
                } else {
                    let target = monkeys[mnkid].target_false;
                    monkeys[target].items.push(new_i);
                }
            }
            monkeys[mnkid].items = Vec::new();
        }
    }
    get_top_prod(monkeys)
}

fn solve_p2(file_name: &str) -> usize {
    let mut monkeys = get_input(file_name);
    let comdiv: usize = monkeys.iter().map(|a| a.test).product();
    for _ in 0..10000 {
        for mnkid in 0..monkeys.len() {
            monkeys[mnkid].inspected_items += monkeys[mnkid].items.len();
            for i in 0..monkeys[mnkid].items.len() {
                let new_i = monkeys[mnkid].op.apply(monkeys[mnkid].items[i]) % comdiv;
                if new_i % monkeys[mnkid].test == 0 {
                    let target = monkeys[mnkid].target_true;
                    monkeys[target].items.push(new_i);
                } else {
                    let target = monkeys[mnkid].target_false;
                    monkeys[target].items.push(new_i);
                }
            }
            monkeys[mnkid].items = Vec::new();
        }
    }
    get_top_prod(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 10605;
        let result = solve_p1("small/11.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 2713310158;
        let result = solve_p2("small/11.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        assert_eq!(solve_p1("input/11.txt"), 50830);
    }
    #[test]
    fn p2() {
        let exp = 14399640002;
        let res = solve_p2("input/11.txt");
        assert_eq!(res, exp);
    }
}
