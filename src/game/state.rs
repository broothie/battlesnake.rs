use super::board::Board;
use super::mv::Move;
use super::point::Point;
use super::snake::Snake;

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Debug)]
pub struct State {
    pub turn: u16,
    pub board: Board,
    pub you: Snake,
}

impl State {
    pub fn decide(&self, food_coefficient: f32) -> Result<Move> {
        let mut moves = Move::all();

        moves = moves
            .into_iter()
            // Remove out of bounds moves
            .filter(|mv| self.board.in_bounds(&self.you.head.shift(mv)))
            // Remove collision moves
            .filter(|mv| {
                !self.board.snakes.iter().any(|snake| {
                    snake.at(
                        &self.you.head.shift(mv),
                        snake != &self.you || self.turn > 2,
                    )
                })
            })
            // Remove threatened moves
            .filter(|mv| !self.threatened(&self.you.head.shift(mv)))
            .collect();

        println!("valid moves: {:?}", moves);
        if let Some(closest_food) = self.board.closest_food(&self.you.head) {
            let distance = closest_food.distance(&self.you.head);
            if distance as f32 > self.you.health as f32 * food_coefficient {
                let towards = self.you.head.towards(closest_food);

                let move_set: HashSet<Move> = HashSet::from_iter(moves.iter().cloned());
                let towards_set: HashSet<Move> = HashSet::from_iter(towards.iter().cloned());

                let intersection: Vec<Move> =
                    move_set.intersection(&towards_set).cloned().collect();
                if !intersection.is_empty() {
                    println!("moving towards food: {:?}", intersection);
                    moves = intersection;
                }
            }
        }

        if moves.is_empty() {
            moves = Move::all();
        }

        moves.shuffle(&mut thread_rng());
        let mv = moves.get(0).expect("failed to get move");

        Ok(*mv)
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
