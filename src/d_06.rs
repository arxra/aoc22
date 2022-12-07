use std::collections::HashSet;

use itertools::Itertools;

fn solve(file_name: &str, offset: usize) -> usize {
    let inp = std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .to_owned();
    for a in (offset..inp.len()) {
        if inp[a - offset..a].chars().collect::<HashSet<char>>().len() == offset {
            return a;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 10;
        let result = solve("small/06.txt", 4);
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 29;
        let result = solve("small/06.txt", 14);
    }
    #[test]
    fn p1() {
        dbg!(solve("input/06.txt", 4));
    }
    #[test]
    fn p2() {
        dbg!(solve("input/06.txt", 14));
    }
}
