use itertools::Itertools;

fn solve(file_name: &str, p1: bool) -> String {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 10];
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .for_each(|line| match line {
            "" => stacks.iter_mut().for_each(|v| v.reverse()),
            a if a[..2] == *" 1" => (),
            a if a[..4] == *"move" => {
                let nums: Vec<usize> = a.split(' ').flat_map(|f| f.parse::<usize>()).collect();
                if p1 {
                    for _ in (0..nums[0]) {
                        if let Some(val) = stacks[nums[1] - 1].pop() {
                            stacks[nums[2] - 1].push(val)
                        }
                    }
                } else {
                    let len = stacks[nums[1] - 1].len();
                    let mut tempstack = stacks[nums[1] - 1]
                        .drain(len - nums[0]..)
                        .collect::<Vec<char>>();
                    stacks[nums[2] - 1].append(&mut tempstack);
                }
            }
            a => {
                let a: Vec<char> = a.chars().collect();
                (0..stacks.len()).for_each(|i| {
                    if let Some(inner) = a.get(1 + i * 4) {
                        if *inner != ' ' {
                            stacks[i].push(*inner)
                        }
                    }
                });
            }
        });
    stacks
        .into_iter()
        .flat_map(|v| v.into_iter().last())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = "CMZ";
        let result = solve("small/05.txt", true);
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = "MCD";
        let result = solve("small/05.txt", false);
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        dbg!(solve("input/05.txt", true));
    }
    #[test]
    fn p2() {
        dbg!(solve("input/05.txt", false));
    }
}
