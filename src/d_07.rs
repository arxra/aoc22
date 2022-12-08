use std::str::Lines;

use itertools::Itertools;

#[derive(Debug)]
struct Path {
    tot_size: Option<usize>,
    folders: Vec<Path>,
    files: Vec<(usize, String)>,
}

impl Path {
    fn new() -> Self {
        Self {
            tot_size: None,
            folders: Vec::new(),
            files: Vec::new(),
        }
    }

    fn calc_size(&mut self) {
        self.tot_size = Some(
            self.folders.iter().flat_map(|a| a.tot_size).sum::<usize>()
                + self.files.iter().map(|(s, _)| s).sum::<usize>(),
        )
    }
    fn sum_size(&self, max: usize) -> usize {
        let mut s = 0;
        s += self.folders.iter().map(|a| a.sum_size(max)).sum::<usize>();
        match self.tot_size {
            Some(t) if t < max => s += t,
            _ => (),
        }
        s
    }
}

fn crawl(it: &mut Lines, sizes: &mut Vec<usize>) -> Path {
    let mut path = Path::new();
    while let Some(line) = it.next() {
        match line
            .split(' ')
            .map(|a| a.to_owned())
            .collect::<Vec<String>>()
            {
                a if a.len() == 2 => match (a[0].clone(), a[1].clone()) {
                    (a,b) if let Ok(size) = a.parse()=> path.files.push((size, b.to_string())),
                    _ => (),
                },
                a if a.len() == 3 => match &a[2][..] {
                    ".." => {
                        path.calc_size();
                        sizes.push(path.tot_size.unwrap());
                        return path
                    },
                    a => path.folders.push(crawl(it,sizes)),
                },
                a => {
                    unreachable!()
                }
            }
    }
    path.calc_size();
    sizes.push(path.tot_size.unwrap());
    path
}

fn solve_p1(file_name: &str) -> usize {
    let mut sizes = Vec::new();
    let parsed = crawl(
        &mut std::fs::read_to_string(file_name).unwrap().lines(),
        &mut sizes,
    );
    parsed.sum_size(100000)
}

fn solve_p2(file_name: &str) -> usize {
    let mut sizes = Vec::new();
    let parsed = crawl(
        &mut std::fs::read_to_string(file_name).unwrap().lines(),
        &mut sizes,
    );

    sizes.sort();

    let total_space = 70000000;
    let required_size = 30000000;
    let outer_size = parsed.tot_size.unwrap();
    let available_space = total_space - outer_size;
    let needed_size = required_size - available_space;

    sizes
        .into_iter()
        .filter(|a| a > &needed_size)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_exp() {
        let expected = 95437;
        let result = solve_p1("small/07.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 24933642;
        let result = solve_p2("small/07.txt");
    }
    #[test]
    fn p1() {
        dbg!(solve_p1("input/07.txt"));
    }
    #[test]
    fn p2() {
        dbg!(solve_p2("input/07.txt"));
    }
}
