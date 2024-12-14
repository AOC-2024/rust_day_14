use std::fs::read_to_string;
use std::str::FromStr;
use regex::Regex;

pub fn count_safety_factor(input_path: &str) -> usize {
    0
}

fn extract_robots(input_path: &str) -> Vec<Robot> {
    read_to_string(input_path).unwrap()
        .lines()
        .map(|line| map_robot(line))
        .collect()
}

fn map_robot(line: &str) -> Robot {
    println!("{line}");
    let button_regex = Regex::new("p=(?<px>[0-9]{1,3}),(?<py>-?[0-9]{1,3}) v=(?<vx>-?[0-9]{1,3}),(?<vy>-?[0-9]{1,3})").unwrap();
    let matched = button_regex.captures_iter(line).next().unwrap();
    let px: isize = FromStr::from_str(matched.name("px").unwrap().as_str()).unwrap();
    let py: isize = FromStr::from_str(matched.name("py").unwrap().as_str()).unwrap();
    let vx: isize = FromStr::from_str(matched.name("vx").unwrap().as_str()).unwrap();
    let vy: isize = FromStr::from_str(matched.name("vy").unwrap().as_str()).unwrap();

    Robot {
        position: Point {
            x: px,
            y: py
        },
        velocity: Point {
            x: vx,
            y: vy
        }
    }
}

#[derive(Debug, PartialEq)]
struct Robot {
    position: Point,
    velocity: Point
}
#[derive(Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_robots() {
        assert_eq!(
            extract_robots("tests/resources/light_puzzle.txt"),
            vec![
                Robot {
                    position: Point {x: 0, y: 4 },
                    velocity: Point { x: 3, y: -3 }
                },
                Robot {
                    position: Point {x: 6, y: 3 },
                    velocity: Point { x: -1, y: -3 }
                }
            ]
        );
    }
}
