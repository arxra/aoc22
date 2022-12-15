use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

fn get_input(file_name: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect()
}

fn get_or_insert(
    to_visit: &mut BTreeMap<usize, Vec<(usize, usize)>>,
    cost: usize,
    cords: (usize, usize),
) {
    if let Some(v) = to_visit.get_mut(&(cost + 1)) {
        v.push(cords);
    } else {
        to_visit.insert(cost + 1, vec![cords]);
    };
}

fn solve(
    to_visit: &mut BTreeMap<usize, Vec<(usize, usize)>>,
    l: &HashMap<char, usize>,
    inp: &[Vec<char>],
) -> usize {
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some((cost, arr)) = to_visit.pop_first() {
        // dbg!("processing ({}{})", x,y);
        for (x, y) in arr {
            if inp[x][y] == 'E' {
                return cost;
            }
            if visited.get(&(x, y)).is_some() {
                // dbg!("already visited ({x}{y})");
                continue;
            }
            visited.insert((x, y), cost);
            if x > 0 && *l.get(&inp[x][y]).unwrap() >= l.get(&inp[x - 1][y]).unwrap() - 1 {
                get_or_insert(to_visit, cost, (x - 1, y))
            }
            if y > 0 && *l.get(&inp[x][y]).unwrap() >= l.get(&inp[x][y - 1]).unwrap() - 1 {
                get_or_insert(to_visit, cost, (x, y - 1))
            }
            if x < inp.len() - 1
                && *l.get(&inp[x][y]).unwrap() >= l.get(&inp[x + 1][y]).unwrap() - 1
            {
                get_or_insert(to_visit, cost, (x + 1, y))
            }
            if y < inp[0].len() - 1
                && *l.get(&inp[x][y]).unwrap() >= l.get(&inp[x][y + 1]).unwrap() - 1
            {
                get_or_insert(to_visit, cost, (x, y + 1))
            }
        }
        // dbg!(&to_visit);
    }

    unreachable!();
}

#[allow(dead_code)]
fn solve_p1(file_name: &str) -> usize {
    let inp = get_input(file_name);
    let mut to_visit = BTreeMap::new();
    let l: HashMap<char, usize> = ('a'..='z')
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a + 1))
        .chain([('S', usize::MAX)])
        .chain([('E', 27)]) // z == 26, E = z+1
        .collect();

    'out: for x in 0..inp.len() {
        for y in 0..inp[0].len() {
            if inp[x][y] == 'S' {
                to_visit.insert(0, vec![(x, y)]);
                break 'out;
            }
        }
    }
    solve(&mut to_visit, &l, &inp)
}

#[allow(dead_code)]
fn solve_p2(file_name: &str) -> usize {
    let inp = get_input(file_name);
    let mut to_visit = BTreeMap::new();
    let l: HashMap<char, usize> = ('a'..='z')
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a + 1))
        .chain([('S', usize::MAX)])
        .chain([('E', 27)]) // z == 26, E = z+1
        .collect();
    let mut min = usize::MAX;

    for x in 0..inp.len() {
        for y in 0..inp[0].len() {
            if inp[x][y] == 'a' {
                to_visit.insert(0, vec![(x, y)]);
                min = min.min(solve(&mut to_visit, &l, &inp));
            }
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 31;
        let result = solve_p1("small/12.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 29;
        let result = solve_p2("small/12.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        let res = solve_p1("input/12.txt");
        assert_eq!(res, 380);
    }
    #[test]
    fn p2() {
        let sol = solve_p2("input/12.txt");
        assert_eq!(sol, 375);
    }
}
