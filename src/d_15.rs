use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list0,
    sequence::separated_pair,
    *,
};
use tracing::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Sensor {
    x: i64,
    y: i64,
    range: i64,
}

impl Sensor {
    fn distance_to(&self, x: i64, y: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs()
    }
    fn outside_y_range(&self, y: i64) -> i64 {
        let dist_to_y = (self.y - y).abs();
        let range_on_x = self.range - dist_to_y;
        self.x + range_on_x + 1
    }
}

impl From<((i64, i64), (i64, i64))> for Sensor {
    fn from(value: ((i64, i64), (i64, i64))) -> Self {
        let range = (value.0 .0 - value.1 .0).abs() + (value.0 .1 - value.1 .1).abs();
        Self {
            x: value.0 .0,
            y: value.0 .1,
            range,
        }
    }
}

type PosPair = (Sensor, (i64, i64));

#[instrument(skip(input))]
fn parse_line(input: &str) -> IResult<&str, PosPair> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor) = separated_pair(complete::i64, tag(", y="), complete::i64)(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon) = separated_pair(complete::i64, tag(", y="), complete::i64)(input)?;
    Ok((input, ((sensor, beacon).into(), beacon)))
}

fn parse(input: &str) -> IResult<&str, Vec<PosPair>> {
    let (input, lines) = separated_list0(line_ending, parse_line)(input)?;

    Ok((&input, lines))
}

#[allow(dead_code)]
#[instrument(skip(data))]
fn solve_p1(data: &str, row: i64) -> i64 {
    // Parse input
    let (_, input) = parse(data).unwrap();

    // beacon_set to sub
    let beacon_set: Vec<i64> = input
        .iter()
        .filter_map(|(_, (x, y))| if *y == row { Some(*x) } else { None })
        .unique()
        .collect();

    debug!(?beacon_set);

    // Map sensors to range
    input
        .iter()
        .filter_map(|(sensor, _)| {
            debug!(?sensor, sensor.range);
            let sensor_to_row_dist = (sensor.y - row).abs();
            if sensor_to_row_dist < sensor.range {
                Some((sensor.x, sensor.range - sensor_to_row_dist))
            } else {
                None
            }
        })
        .flat_map(|(x, range)| x - range..=x + range)
        .unique()
        .count() as i64
        - beacon_set.len() as i64
}

#[allow(dead_code)]
#[instrument(skip(data, row))]
fn solve_p2(data: &str, row: i64) -> i64 {
    // Parse input
    let (_, input) = parse(data).unwrap();
    let input = input
        .into_iter()
        .sorted_by(|a, b| a.0.range.cmp(&b.0.range))
        .rev()
        .collect_vec();

    warn!(?input);

    for y in 0..=row {
        let mut x = 0;
        'a: while x <= row {
            for (sens, _) in input.iter() {
                let dist_to_sens = sens.distance_to(x, y);
                if dist_to_sens <= sens.range {
                    debug!(x, y, ?sens);
                    let jump_pos = sens.outside_y_range(y);
                    x = jump_pos;
                    continue 'a;
                }
            }
            info!(x, y, "Found");
            return 4000000 * x + y;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = include_str!("../small/15.txt");
    const INPUT: &str = include_str!("../input/15.txt");

    #[test]
    fn p1_exp() {
        let expected = 26;
        let result = solve_p1(SMALL, 10);
        assert_eq!(result, expected);
    }
    #[test]
    fn p2_exp() {
        let expected = 56000011;
        let result = solve_p2(SMALL, 20);
        assert_eq!(result, expected);
    }
    #[test]
    fn p1() {
        let res = solve_p1(INPUT, 2000000);
        assert_eq!(res, 5108096);
    }
    #[test]
    fn p2() {
        let sol = solve_p2(INPUT, 4000000);
        assert_ne!(sol, 1208003992);
        assert_eq!(sol, 4000000);
    }
}
