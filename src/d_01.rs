type File = Vec<Vec<usize>>;

fn solve_p1(file_name: &str) -> usize {
    file(file_name)
        .into_iter()
        .map(|a| a.into_iter().sum::<usize>())
        .max()
        .unwrap()
}

fn solve_p2(file_name: &str) -> usize {
    let mut sums = file(file_name)
        .into_iter()
        .map(|a| a.into_iter().sum::<usize>())
        .collect::<Vec<usize>>();

    (0..3).into_iter().map(|_a| sums.pop().unwrap()).sum()
}

fn file(file_name: &str) -> File {
    let mut v = Vec::new();
    let mut temp = Vec::new();
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .for_each(|f| {
            if f.is_empty() {
                v.push(temp.clone());
                temp = Vec::new();
            } else {
                temp.push(f.parse().unwrap());
            }
        });
    v.push(temp);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1_exp() {
        let expected = 24000;
        let result = solve_p1("small/01.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn d1p2_exp() {
        let expected = 45000;
        let result = solve_p2("small/01.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn d1p1() {
        let result = solve_p1("input/01.txt");
        println!("d1p1: {}", result);
    }
    #[test]
    fn d1p2() {
        let result = solve_p2("input/01.txt");
        println!("d1p2: {}", result);
    }
}
