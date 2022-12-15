fn get_input(file_name: &str) -> Vec<Option<isize>> {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split(' ').collect();
            match l[0] {
                "noop" => None,
                "addx" => Some(l[1].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn add(cycle: isize, reg: isize, ans: &mut Vec<Vec<char>>) -> isize {
    if (cycle - 1) % 40 == 0 {
        ans.push(Vec::new());
    }
    let value = match (cycle - 1) % 40 {
        a if (reg - 1..=reg + 1).contains(&a) => '#',
        _ => '.',
    };
    ans[((cycle - 1) / 40) as usize].push(value);
    match (cycle, reg) {
        (c, reg) if [20, 60, 100, 140, 180, 220].contains(&c) => c * reg,
        _ => 0,
    }
}

fn solve_p1(file_name: &str) -> isize {
    let mut sum = 0;
    let mut reg = 1;
    let mut add_cycles = 1;
    let mut ans = Vec::new();
    get_input(file_name)
        .into_iter()
        .enumerate()
        .for_each(|(i, num)| {
            // start of cycle
            let mut cycle = i as isize + add_cycles;
            if num.is_some() {
                sum += add(cycle, reg, &mut ans);
                add_cycles += 1;
                cycle += 1;
            }

            // cycle
            sum += add(cycle, reg, &mut ans);

            // End of cycle
            if let Some(n) = num {
                reg += n;
            }
        });

    ans.into_iter()
        .map(|a| a.iter().collect::<String>())
        .for_each(|a| println!("{a}"));
    sum
}

#[allow(dead_code)]
fn solve_p2(file_name: &str) -> isize {
    solve_p1(file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 13140;
        let result = solve_p1("small/10.txt");
        assert_eq!(result, expected);
    }
    // #[test]
    // fn p2_exp() {
    //     let expected = 1;
    //     let result = solve_p2("small/10.txt");
    //     assert_eq!(result, expected);
    // }
    #[test]
    fn p1() {
        assert_eq!(solve_p1("input/10.txt"), 16060);
    }
    #[test]
    fn p2() {
        solve_p2("input/10.txt");
        panic!();
    }
}
