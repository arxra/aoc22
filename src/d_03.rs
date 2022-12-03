use std::collections::HashSet;

use itertools::Itertools;

type File = Vec<(Vec<char>, Vec<char>)>;

fn to_prio(a: &char) -> Option<usize> {
    if a.is_uppercase() {
        Some(*a as usize - 38)
    } else {
        Some(*a as usize - 96)
    }
}

fn solve_p1(file_name: &str) -> usize {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|f| {
            let hlen = f.len() / 2;
            let mut it = f.chars();
            let v1 = (&mut it).take(hlen).collect::<Vec<char>>();
            let v2: Vec<char> = it.collect();
            (v1, v2)
        })
        .map(|(v1, v2)| {
            v1.into_iter()
                .unique()
                .filter_map(|a| if v2.contains(&a) { to_prio(&a) } else { None })
                .sum::<usize>()
        })
        .sum()
}

fn solve_p2(file_name: &str) -> usize {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut a| {
            let s1: HashSet<char> = a.next().unwrap().chars().collect();
            let s2: HashSet<char> = a.next().unwrap().chars().collect();
            let s3: HashSet<char> = a.next().unwrap().chars().collect();
            let interset: HashSet<char> = s1.intersection(&s2).map(|a| a.to_owned()).collect();
            let team = interset.intersection(&s3).next().unwrap();
            to_prio(team).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 157;
        let result = solve_p1("small/03.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 70;
        let result = solve_p2("small/03.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        let result = solve_p1("input/03.txt");
        println!("d3p1: {result}");
    }
    #[test]
    fn p2() {
        let result = solve_p2("input/03.txt");
        println!("d3p2: {result}");
    }
}
