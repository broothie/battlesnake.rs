use super::board::Board;
use super::mv::Move;
use super::point::Point;
use super::snake::Snake;

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct State {
    pub turn: u16,
    pub board: Board,
    pub you: Snake,
}

impl State {
    fn process<F>(&self, process: &str, moves: Vec<Move>, f: F) -> Vec<Move>
    where
        F: Fn(Point) -> bool,
    {
        let before = moves.clone();

        let after: Vec<Move> = moves
            .into_iter()
            .filter(|mv| f(self.you.head.shift(&mv)))
            .collect();

        if after.is_empty() {
            println!("{}: skipping because empty", process);
            before
        } else if before == after {
            println!("{}: no changes", process);
            after
        } else {
            println!("{}: {:?} -> {:?}", process, before, after);
            after
        }
    }

    pub fn decide(&self, hunger_coefficient: f32) -> Result<Move> {
        let mut moves = Move::all();

        moves = self.process("in bounds", moves, |point| self.board.in_bounds(&point));
        moves = self.process("snake collisions", moves, |point| {
            !self
                .board
                .snakes
                .iter()
                .any(|snake| snake.at(&point, snake != &self.you || self.turn > 2))
        });

        moves = self.process("threatened", moves, |point| !self.threatened(&point));

        let pocket_sizes = self.board.pocket_sizes();
        let largest = moves
            .iter()
            .map(|mv| {
                pocket_sizes
                    .get(&self.you.head.shift(&mv))
                    .unwrap_or(&0usize)
            })
            .max();

        if let Some(largest) = largest {
            moves = self.process("select largest pocket", moves, |point| {
                pocket_sizes.get(&point).unwrap_or(&0usize) == largest
            });
        }

        if let Some(closest_food) = self.board.closest_food(&self.you.head) {
            let distance = closest_food.distance(&self.you.head);

            if self.need_food(distance, hunger_coefficient) || self.compete_for_biggest() {
                moves = self.process("food moves", moves, |point| {
                    closest_food.distance(&point) < distance
                });
            }
        }

        moves = self.process("kill moves", moves, |point| self.kill_chance(&point));

        println!("selecting move from {:?}", moves);
        moves.shuffle(&mut thread_rng());
        let mv = moves.get(0).expect("failed to get move");

        Ok(*mv)
    }

    fn need_food(&self, distance: i16, hunger_coefficient: f32) -> bool {
        distance as f32 > self.you.health as f32 * hunger_coefficient
    }

    fn compete_for_biggest(&self) -> bool {
        let biggest = self
            .board
            .snakes
            .iter()
            .filter(|snake| *snake != &self.you)
            .map(|snake| snake.length())
            .max();

        biggest.is_some() && self.you.length() <= biggest.unwrap()
    }

    fn threatened(&self, point: &Point) -> bool {
        Move::all()
            .iter()
            .map(|mv| point.shift(mv))
            .filter(|point| self.board.in_bounds(point))
            .filter(|point| point != &self.you.head)
            .filter(|point| {
                self.board
                    .snakes
                    .iter()
                    .filter(|snake| snake.length() >= self.you.length())
                    .any(|snake| point == &snake.head)
            })
            .peekable()
            .peek()
            .is_some()
    }

    fn kill_chance(&self, point: &Point) -> bool {
        Move::all()
            .iter()
            .map(|mv| point.shift(mv))
            .filter(|point| self.board.in_bounds(point))
            .filter(|point| point != &self.you.head)
            .filter(|point| {
                self.board
                    .snakes
                    .iter()
                    .filter(|snake| snake.length() < self.you.length())
                    .any(|snake| point == &snake.head)
            })
            .peekable()
            .peek()
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threatened() {
        let you = Snake {
            id: "you".to_string(),
            health: 0,
            body: vec![Point::new(7, 3), Point::new(8, 3), Point::new(8, 4)],
            head: Point::new(7, 3),
        };

        let snakes = vec![
            you.clone(),
            Snake {
                id: "Big A".to_string(),
                health: 0,
                body: vec![
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(0, 2),
                    Point::new(0, 3),
                ],
                head: Point::new(0, 0),
            },
            Snake {
                id: "Same B".to_string(),
                health: 0,
                body: vec![Point::new(3, 2), Point::new(3, 3), Point::new(3, 4)],
                head: Point::new(3, 2),
            },
            Snake {
                id: "Lil C".to_string(),
                health: 0,
                body: vec![Point::new(2, 8), Point::new(2, 9)],
                head: Point::new(2, 8),
            },
        ];

        let board = Board {
            height: 10,
            width: 10,
            food: vec![Point::new(7, 7)],
            snakes,
        };

        let state = State {
            turn: 0,
            you: you.clone(),
            board,
        };

        // You
        assert_eq!(state.threatened(&Point::new(6, 3)), false);
        assert_eq!(state.threatened(&Point::new(7, 2)), false);
        assert_eq!(state.threatened(&Point::new(7, 4)), false);

        // Big A, corner
        assert_eq!(state.threatened(&Point::new(1, 0)), true);

        // Same B
        assert_eq!(state.threatened(&Point::new(2, 2)), true);
        assert_eq!(state.threatened(&Point::new(3, 1)), true);
        assert_eq!(state.threatened(&Point::new(4, 2)), true);

        // Lil C
        assert_eq!(state.threatened(&Point::new(1, 8)), false);
        assert_eq!(state.threatened(&Point::new(2, 7)), false);
        assert_eq!(state.threatened(&Point::new(3, 8)), false);

        // Elsewhere
        assert_eq!(state.threatened(&Point::new(4, 4)), false);
    }
}
