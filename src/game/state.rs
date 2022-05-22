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
    pub fn decide(&self, hunger_coefficient: f32) -> Result<Move> {
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

        // Compute pockets
        let pocket_sizes = self.board.pocket_sizes();

        // Remove small pockets
        let pocket_moves: Vec<Move> = moves
            .iter()
            .filter(|mv| {
                pocket_sizes
                    .get(&self.you.head.shift(mv))
                    .unwrap_or(&0usize)
                    > &(self.you.length() as usize)
            })
            .cloned()
            .collect();

        if !pocket_moves.is_empty() {
            moves = pocket_moves;
        }

        println!("valid moves: {:?}", moves);
        if let Some(closest_food) = self.board.closest_food(&self.you.head) {
            let distance = closest_food.distance(&self.you.head);
            if self.need_food(distance, hunger_coefficient) || self.compete_for_biggest() {
                let towards = self.you.head.towards(closest_food);

                let intersection = vec_intersect(&moves, &towards);
                if !intersection.is_empty() {
                    println!("moving towards food: {:?}", intersection);
                    moves = intersection;
                }
            }
        } else if let Some(kill_moves) = self.kill_moves() {
            let intersection = vec_intersect(&moves, &kill_moves);
            if !intersection.is_empty() {
                println!("attempting kill: {:?}", intersection);
                moves = intersection;
            }
        }

        if moves.is_empty() {
            moves = Move::all();
        }

        moves.shuffle(&mut thread_rng());
        let mv = moves.get(0).expect("failed to get move");

        Ok(*mv)
    }

    fn need_food(&self, distance: i16, hunger_coefficient: f32) -> bool {
        distance as f32 > self.you.health as f32 * hunger_coefficient
    }

    fn compete_for_biggest(&self) -> bool {
        let biggest = self.board.snakes.iter().map(|snake| snake.length()).max();

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

    fn kill_moves(&self) -> Option<Vec<Move>> {
        let moves: Vec<Move> = Move::all()
            .into_iter()
            .filter(|mv| self.kill_chance(&self.you.head.shift(mv)))
            .collect();

        if moves.is_empty() {
            None
        } else {
            Some(moves)
        }
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

fn vec_intersect<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    let a_set: HashSet<T> = HashSet::from_iter(a.iter().cloned());
    let b_set: HashSet<T> = HashSet::from_iter(b.iter().cloned());

    a_set.intersection(&b_set).cloned().collect()
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
