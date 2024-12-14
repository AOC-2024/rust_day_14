use regex::Regex;
use std::fs::read_to_string;
use std::ops::Div;
use std::str::FromStr;
use itertools::Itertools;

pub fn count_safety_factor(input_path: &str) -> usize {
    let mut map = Map::new(extract_robots(input_path));
    for _ in 0..100 {
        map.wait();
    }

    map.count_safety_factors()
}

pub fn count_x_mas_tree_regroup_seconds(input_path: &str) -> usize {
    let mut map = Map::new(extract_robots(input_path));
    map.seconds_to_regroup_as_x_mas_tree()
}

fn extract_robots(input_path: &str) -> Vec<Robot> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|line| map_robot(line))
        .collect()
}

fn map_robot(line: &str) -> Robot {
     let button_regex = Regex::new(
        "p=(?<px>[0-9]{1,3}),(?<py>-?[0-9]{1,3}) v=(?<vx>-?[0-9]{1,3}),(?<vy>-?[0-9]{1,3})",
    )
    .unwrap();
    let matched = button_regex.captures_iter(line).next().unwrap();
    let px: isize = FromStr::from_str(matched.name("px").unwrap().as_str()).unwrap();
    let py: isize = FromStr::from_str(matched.name("py").unwrap().as_str()).unwrap();
    let vx: isize = FromStr::from_str(matched.name("vx").unwrap().as_str()).unwrap();
    let vy: isize = FromStr::from_str(matched.name("vy").unwrap().as_str()).unwrap();

    Robot {
        position: Point { x: px, y: py },
        velocity: Point { x: vx, y: vy },
    }
}

#[derive(Debug, PartialEq)]
struct Robot {
    position: Point,
    velocity: Point,
}
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

struct Map {
    bounds: Point,
    seconds_elapsed: usize,
    robots: Vec<Robot>,
}

impl Map {
    fn new(robots: Vec<Robot>) -> Map {
        let bounds = Self::get_bounds(&robots);
        Map {
            robots,
            seconds_elapsed: 0,
            bounds,
        }
    }

    fn wait(&mut self) {
        self.seconds_elapsed += 1;

        self.robots = self
            .robots
            .iter()
            .map(|robot| self.next_robot_position(robot))
            .collect()
    }

    fn next_robot_position(&self, robot: &Robot) -> Robot {
        let x = (robot.position.x + robot.velocity.x).rem_euclid(self.bounds.x + 1);
        let y = (robot.position.y + robot.velocity.y).rem_euclid(self.bounds.y + 1);
        Robot {
            position: Point { x, y },
            velocity: robot.velocity.clone(),
        }
    }

    fn count_safety_factors(&self) -> usize {
        let excluding_middle_x = self.bounds.x.div(2);
        let excluding_middle_y = self.bounds.y.div(2);
        let mut quadrans = (0, 0, 0, 0);
        self.robots.iter().for_each(|robot| {
            if robot.position.x == excluding_middle_x || robot.position.y == excluding_middle_y {
                return
            }
            if robot.position.x < excluding_middle_x && robot.position.y < excluding_middle_y {
                quadrans.0 += 1;
            }
            else if robot.position.x < excluding_middle_x && robot.position.y > excluding_middle_y {
                quadrans.1 += 1;
            }
            else if robot.position.x > excluding_middle_x && robot.position.y < excluding_middle_y {
                quadrans.2 += 1;
            } else {
                quadrans.3 += 1;
            }
        });

        quadrans.0 * quadrans.1 * quadrans.2 * quadrans.3
    }

    fn get_bounds(robots: &Vec<Robot>) -> Point {
        let x = robots.iter().map(|robot| robot.position.x).max().unwrap();
        let y = robots.iter().map(|robot| robot.position.y).max().unwrap();
        Point { x, y }
    }
}

impl Map {
    fn seconds_to_regroup_as_x_mas_tree(&mut self) -> usize {
        loop {
            // Update the state of the map
            self.wait();

            //self.pretty_print();

            let diff: usize = self.robots
                .iter()
                .combinations(2)
                .map(|robot| {
                    ((robot[0].position.x - robot[1].position.x).abs()
                        + (robot[0].position.y - robot[1].position.y).abs()) as usize
                })
                .sum::<usize>()
                / self.robots.len();

            if diff <= 10_000 {
                self.pretty_print();
                return self.seconds_elapsed;
            }
        }
    }

    fn pretty_print(&self) {
        let (min_x, max_x, min_y, max_y) = self.get_robot_bounds();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.robots.iter().any(|robot| robot.position.x == x && robot.position.y == y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn get_robot_bounds(&self) -> (isize, isize, isize, isize) {
        let min_x = self.robots.iter().map(|robot| robot.position.x).min().unwrap();
        let max_x = self.robots.iter().map(|robot| robot.position.x).max().unwrap();
        let min_y = self.robots.iter().map(|robot| robot.position.y).min().unwrap();
        let max_y = self.robots.iter().map(|robot| robot.position.y).max().unwrap();
        (min_x, max_x, min_y, max_y)
    }

    fn are_robots_within_bounds(&self, min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> bool {
        self.robots.iter().all(|robot| {
            robot.position.x >= min_x && robot.position.x <= max_x &&
                robot.position.y >= min_y && robot.position.y <= max_y
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_robot_out_of_bounds() {
        let mut map = Map::new(vec![Robot {
            position: Point { x: 2, y: 4 },
            velocity: Point { x: 2, y: -3 },
        }, Robot {
            position: Point { x: 10, y: 6 },
            velocity: Point { x: 0, y: 0 },
        }]);
        map.wait();
        assert_eq!(map.robots.get(0).unwrap().position, Point { x: 4, y: 1 });

        map.wait();
        assert_eq!(map.robots.get(0).unwrap().position, Point { x: 6, y: 5 });

        map.wait();
        assert_eq!(map.robots.get(0).unwrap().position, Point { x: 8, y: 2 });

        map.wait();
        assert_eq!(map.robots.get(0).unwrap().position, Point { x: 10, y: 6 });

        map.wait();
        assert_eq!(map.robots.get(0).unwrap().position, Point { x: 1, y: 3 });
    }

    #[test]
    fn should_move_robots_two_second() {
        let mut map = Map::new(extract_robots("tests/resources/move_one_time.txt"));
        map.wait();
        map.wait();
        assert_eq!(map.seconds_elapsed, 2);
        assert_eq!(map.robots.get(0).unwrap().position, Point { x: 3, y: 4 });
        assert_eq!(map.robots.get(1).unwrap().position, Point { x: 2, y: 2 });
    }

    #[test]
    fn should_move_robots_one_second() {
        let mut map = Map::new(extract_robots("tests/resources/move_one_time.txt"));
        map.wait();
        assert_eq!(map.seconds_elapsed, 1);
        assert_eq!(map.robots.get(0).unwrap().position, Point { x: 3, y: 4 });
        assert_eq!(map.robots.get(1).unwrap().position, Point { x: 1, y: 1 });
    }

    #[test]
    fn should_find_bounds() {
        let map = Map::new(extract_robots("tests/resources/puzzle.txt"));
        assert_eq!(map.bounds, Point { x: 10, y: 6 });
    }

    #[test]
    fn should_extract_robots() {
        assert_eq!(
            extract_robots("tests/resources/light_puzzle.txt"),
            vec![
                Robot {
                    position: Point { x: 0, y: 4 },
                    velocity: Point { x: 3, y: -3 }
                },
                Robot {
                    position: Point { x: 6, y: 3 },
                    velocity: Point { x: -1, y: -3 }
                }
            ]
        );
    }
}
