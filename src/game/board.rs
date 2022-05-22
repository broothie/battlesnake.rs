use std::collections::{HashMap, HashSet};

use colored::*;
use serde::Deserialize;

use super::mv::Move;
use super::point::Point;
use super::snake::Snake;

const COLORS: [&str; 5] = ["blue", "red", "yellow", "purple", "cyan"];

#[derive(Deserialize, Debug, Clone)]
pub struct Board {
    pub height: i16,
    pub width: i16,
    pub food: Vec<Point>,
    pub snakes: Vec<Snake>,
}

impl Board {
    pub fn in_bounds(&self, point: &Point) -> bool {
        0 <= point.x && point.x < self.width && 0 <= point.y && point.y < self.height
    }

    pub fn food_at(&self, point: &Point) -> bool {
        self.food.contains(point)
    }

    pub fn snake_at(&self, point: &Point) -> Option<&Snake> {
        self.snakes.iter().find(|snake| snake.body.contains(point))
    }

    pub fn closest_food(&self, point: &Point) -> Option<&Point> {
        self.food
            .iter()
            .min_by(|a, b| point.distance(a).cmp(&point.distance(b)))
    }

    pub fn pocket_sizes(&self) -> HashMap<Point, usize> {
        let mut checked: HashSet<Point> = HashSet::new();
        let mut pockets: Vec<HashSet<Point>> = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let point = Point::new(x, y);
                if checked.contains(&point) {
                    continue;
                }

                checked.insert(point);

                if let Some(pocket) = self.pocket_at(&point) {
                    checked.extend(pocket.iter());
                    pockets.push(pocket);
                }
            }
        }

        let mut sizes: HashMap<Point, usize> = HashMap::new();
        for pocket in pockets {
            for point in pocket.iter() {
                sizes.insert(point.clone(), pocket.len());
            }
        }

        sizes
    }

    fn pocket_at(&self, point: &Point) -> Option<HashSet<Point>> {
        let mut queue = vec![point.clone()];

        let mut pocket = HashSet::new();
        while let Some(point) = queue.pop() {
            if pocket.contains(&point) || !self.in_bounds(&point) || self.snake_at(&point).is_some()
            {
                continue;
            }

            pocket.insert(point);

            Move::all()
                .iter()
                .map(|mv| point.shift(mv))
                .for_each(|neighbor| queue.push(neighbor));
        }

        if pocket.is_empty() {
            None
        } else {
            Some(pocket)
        }
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for y in (0..self.height).rev() {
            output.push_str(&format!("{}  ", y));

            for x in 0..self.width {
                let point = Point::new(x, y);

                let string = if self.food_at(&point) {
                    "$".green().to_string()
                } else if let Some(snake) = self.snake_at(&point) {
                    let index = self
                        .snakes
                        .iter()
                        .position(|s| s == snake)
                        .expect("snake not found");

                    let color = COLORS[index % COLORS.len()];

                    if point == snake.head { "@" } else { "#" }
                        .color(color)
                        .to_string()
                } else {
                    ".".to_string()
                };

                output.push_str(&(string + " "));
            }

            output += "\n"
        }

        let bottom = (0..self.width)
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        output.push_str(&format!("\n   {}\n", bottom));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_bounds() {
        let board = Board {
            height: 10,
            width: 10,
            food: vec![],
            snakes: vec![],
        };

        assert_eq!(board.in_bounds(&Point { x: 0, y: 0 }), true);
        assert_eq!(board.in_bounds(&Point { x: -1, y: 0 }), false);
        assert_eq!(board.in_bounds(&Point { x: 9, y: 9 }), true);
        assert_eq!(board.in_bounds(&Point { x: 9, y: 10 }), false);
        assert_eq!(board.in_bounds(&Point { x: 10, y: 10 }), false);
    }

    #[test]
    fn closest_food() {
        let board = Board {
            height: 10,
            width: 10,
            food: vec![Point::new(1, 2), Point::new(1, 5)],
            snakes: vec![],
        };

        assert_eq!(
            board.closest_food(&Point::new(0, 0)),
            Some(&Point::new(1, 2))
        );

        assert_eq!(
            board.closest_food(&Point::new(2, 3)),
            Some(&Point::new(1, 2))
        );

        assert_eq!(
            board.closest_food(&Point::new(4, 4)),
            Some(&Point::new(1, 5))
        );
    }

    #[test]
    fn pocket_at() {
        let board = Board {
            height: 5,
            width: 5,
            food: vec![],
            snakes: vec![
                Snake {
                    id: "a".to_string(),
                    health: 10,
                    body: vec![
                        Point::new(3, 4),
                        Point::new(3, 3),
                        Point::new(3, 2),
                        Point::new(2, 2),
                        Point::new(1, 2),
                        Point::new(1, 1),
                        Point::new(1, 0),
                    ],
                    head: Point::new(3, 4),
                },
                Snake {
                    id: "b".to_string(),
                    health: 10,
                    body: vec![Point::new(4, 2), Point::new(4, 1)],
                    head: Point::new(4, 2),
                },
            ],
        };

        assert_eq!(
            board.pocket_at(&Point::new(0, 0)).expect("must be some"),
            HashSet::from([
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
                Point::new(0, 4),
                Point::new(1, 3),
                Point::new(1, 4),
                Point::new(2, 3),
                Point::new(2, 4),
            ])
        );

        assert_eq!(
            board.pocket_at(&Point::new(4, 0)).expect("must be some"),
            HashSet::from([
                Point::new(2, 0),
                Point::new(2, 1),
                Point::new(3, 0),
                Point::new(3, 1),
                Point::new(4, 0),
            ])
        );

        assert_eq!(
            board.pocket_at(&Point::new(4, 4)).expect("must be some"),
            HashSet::from([Point::new(4, 3), Point::new(4, 4)])
        );

        assert!(board.pocket_at(&Point::new(1, 1)).is_none());
    }

    #[test]
    fn pocket_sizes() {
        let board = Board {
            height: 5,
            width: 5,
            food: vec![],
            snakes: vec![
                Snake {
                    id: "a".to_string(),
                    health: 10,
                    body: vec![
                        Point::new(3, 4),
                        Point::new(3, 3),
                        Point::new(3, 2),
                        Point::new(2, 2),
                        Point::new(1, 2),
                        Point::new(1, 1),
                        Point::new(1, 0),
                    ],
                    head: Point::new(3, 4),
                },
                Snake {
                    id: "b".to_string(),
                    health: 10,
                    body: vec![Point::new(4, 2), Point::new(4, 1)],
                    head: Point::new(4, 2),
                },
            ],
        };

        let pocket_sizes = board.pocket_sizes();

        assert_eq!(
            pocket_sizes.get(&Point::new(0, 0)).unwrap_or(&0usize),
            &9usize
        );
        assert_eq!(
            pocket_sizes.get(&Point::new(1, 1)).unwrap_or(&0usize),
            &0usize
        );
    }
}
