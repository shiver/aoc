use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, Read};

use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

mod aoc_2018 {
    use super::*;

    pub fn day1(input: &str) -> Result<()> {
        fn part1(input: &str) -> Result<()> {
            let mut freq = 0;
            for line in input.lines() {
                let change: i32 = line.parse()?;
                freq += change;
            }

            println!("day1 part1: {}", freq);
            Ok(())
        }

        fn part2(input: &str) -> Result<()> {
            let mut lookup: HashSet<i32> = HashSet::new();
            let mut freq = 0;
            for line in input.lines().cycle() {
                let change: i32 = line.parse()?;
                freq += change;
                if !lookup.contains(&freq) {
                    lookup.insert(freq);
                } else {
                    break;
                }
            }

            println!("day1 part2: {}", freq);
            Ok(())
        }

        part1(&input)?;
        part2(&input)?;
        Ok(())
    }

    pub fn day2(input: &str) -> Result<()> {
        fn part1(input: &str) -> Result<()> {
            let (mut twos, mut threes) = (0, 0);

            for line in input.lines() {
                let mut lookup: HashMap<char, u8> = HashMap::new();
                for ch in line.chars() {
                    let count = lookup.entry(ch).or_insert(0);
                    *count += 1;
                }

                if lookup.values().filter(|&v| *v == 2).count() > 0 {
                    twos += 1;
                }

                if lookup.values().filter(|&v| *v == 3).count() > 0 {
                    threes += 1;
                }
            }

            println!("day2 part1: {}", twos * threes);

            Ok(())
        }

        fn part2(input: &str) -> Result<()> {
            'outer: for base in input.lines() {
                for comparison in input.lines() {
                    let reduced: String = base
                        .chars()
                        .zip(comparison.chars())
                        .filter(|(a, b)| a == b)
                        .map(|(a, _)| a)
                        .collect();

                    if base.len() - reduced.len() == 1 {
                        println!("day2 part 2: {:?}", reduced);
                        break 'outer;
                    }
                }
            }
            Ok(())
        }

        part1(&input)?;
        part2(&input)?;

        Ok(())
    }

    pub fn day3(input: &str) -> Result<()> {
        #[derive(Debug)]
        struct Plot {
            owner: u16,
            x: u16,
            y: u16,
            width: u16,
            height: u16,
        };

        fn part1(input: &str) -> Result<()> {
            let re = Regex::new(
                r"#(?P<owner>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<width>\d+)x(?P<height>\d+)",
            )?;

            let mut grid: HashMap<(u16, u16), u8> = HashMap::new();

            for line in input.lines() {
                let captures = re.captures(&line).expect("No matches");
                let plot = Plot {
                    owner: captures["owner"].parse()?,
                    x: captures["x"].parse()?,
                    y: captures["y"].parse()?,
                    width: captures["width"].parse()?,
                    height: captures["height"].parse()?,
                };

                for y in plot.y..(plot.y + plot.height) {
                    for x in plot.x..(plot.x + plot.width) {
                        let count = grid.entry((x, y)).or_insert(0);
                        *count += 1;
                    }
                }
            }

            let res: Vec<&u8> = grid.values().filter(|&v| *v >= 2).collect();
            println!("day3 part1: {}", res.len());

            Ok(())
        }

        part1(&input)?;
        Ok(())
    }
}

fn pick_day(year: &str, day: &str) -> Result<fn(&str) -> Result<()>> {
    use std::io::{Error, ErrorKind};

    if year == "2018" {
        if day == "day1" {
            return Ok(aoc_2018::day1);
        } else if day == "day2" {
            return Ok(aoc_2018::day2);
        } else if day == "day3" {
            return Ok(aoc_2018::day3);
        }
    }

    Err(Box::new(Error::new(
        ErrorKind::Other,
        "Unknown year/day combination!",
    )))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("EG:\n  $ aoc 2018 day1 < input/2018/day1");
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        let func = pick_day(&args[1], &args[2])?;
        func(&input)?;
    }

    Ok(())
}
