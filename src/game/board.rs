use std::collections::{HashMap, HashSet};

use colored::*;
use serde::Deserialize;

use super::mv::Move;
use super::point::Point;
use super::snake::Snake;

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

                let pocket = self.pocket_at(&point);
                checked.extend(pocket.iter());
                pockets.push(pocket);
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

    fn pocket_at<'a>(&'a self, point: &Point) -> HashSet<Point> {
        let mut pocket = HashSet::from([point.clone()]);
        let mut queue = vec![point.clone()];

        while let Some(point) = queue.pop() {
            let neighbors: Vec<Point> = Move::all()
                .iter()
                .map(|mv| point.shift(mv))
                .filter(|point| self.in_bounds(point))
                .filter(|point| self.snake_at(point).is_some())
                .collect();

            for neighbor in neighbors {
                pocket.insert(neighbor);
                queue.push(neighbor.clone());
            }
        }

        pocket
    }
}

const COLORS: [&str; 5] = ["blue", "red", "yellow", "purple", "cyan"];

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
}
