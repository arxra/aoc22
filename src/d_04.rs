

fn solve_p1(file_name: &str) -> usize {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|f| {
            f.split_terminator(&['-', ','])
                .map(|a| a.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|i| match (i[0], i[1], i[2], i[3]) {
            (a, b, c, d) if a >= c && b <= d => true,
            (a, b, c, d) if c >= a && d <= b => true,
            _a => false,
        })
        .count()
}

fn solve_p2(file_name: &str) -> usize {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|f| {
            f.split_terminator(&['-', ','])
                .map(|a| a.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|i| match (i[0], i[1], i[2], i[3]) {
            (a, _b, c, d) if a >= c && a <= d => true,
            (_a, b, c, d) if b >= c && b <= d => true,
            (a, b, c, _d) if c >= a && c <= b => true,
            (a, b, _c, d) if d >= a && d <= b => true,
            _a => false,
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 2;
        let result = solve_p1("small/04.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 4;
        let result = solve_p2("small/04.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        dbg!(solve_p1("input/04.txt"));
    }
    #[test]
    fn p2() {
        dbg!(solve_p2("input/04.txt"));
    }
}
