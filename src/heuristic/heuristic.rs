use super::super::game::*;
use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};

pub struct Heuristic {
    pub state: State,
}

impl Heuristic {
    pub fn decide(&self, hunger_coefficient: f32) -> Result<Command> {
        let mut moves = Move::all();

        moves = self.process("in bounds", moves, |point| {
            self.state.board.in_bounds(&point)
        });
        moves = self.process("snake collisions", moves, |point| {
            !self
                .state
                .board
                .snakes
                .iter()
                .any(|snake| snake.at(&point, snake != &self.state.you || self.state.turn > 2))
        });

        moves = self.process("threatened", moves, |point| !self.threatened(&point));

        if self.state.game.ruleset.name == "royale" {
            moves = self.process("hazards", moves, |point| {
                !self.state.board.hazard_at(&point)
            });
        }

        let pocket_sizes = self.state.board.pocket_sizes();
        let largest = moves
            .iter()
            .map(|mv| {
                pocket_sizes
                    .get(&self.state.you.head.shift(&mv))
                    .unwrap_or(&0usize)
            })
            .max();

        if let Some(size) = largest {
            moves = self.process("select largest pocket", moves, |point| {
                pocket_sizes.get(&point).unwrap_or(&0usize) == size
            });
        }

        if let Some(closest_food) = self.state.board.closest_food(&self.state.you.head) {
            let distance = closest_food.distance(&self.state.you.head);

            if self.need_food(distance, hunger_coefficient) || self.compete_for_biggest() {
                moves = self.process("food moves", moves, |point| {
                    closest_food.distance(&point) < distance
                });
            }
        }

        moves = self.process("kill moves", moves, |point| self.kill_chance(&point));

        let closest_smaller_snake = self
            .state
            .board
            .snakes
            .iter()
            .filter(|snake| snake.length() < self.state.you.length())
            .min_by(|snake_a, snake_b| {
                let distance_a = self.state.you.head.distance(&snake_a.head);
                let distance_b = self.state.you.head.distance(&snake_b.head);

                distance_a.cmp(&distance_b)
            });

        if let Some(snake) = closest_smaller_snake {
            moves = self.process("seek kill", moves, |point| {
                let current_distance = self.state.you.head.distance(&snake.head);
                let new_distance = point.distance(&snake.head);

                new_distance < current_distance
            });
        }

        moves = self.process("circle", moves, |point| {
            let tail = self.state.you.tail();
            let current_distance = self.state.you.head.distance(&tail);
            let new_distance = point.distance(&tail);

            new_distance < current_distance
        });

        println!(
            "game {}, turn {}, selecting move from {:?}",
            self.state.game.id, self.state.turn, moves
        );

        moves.shuffle(&mut thread_rng());
        let mv = moves.get(0).expect("failed to get move");

        let shout = if self.state.board.food_at(&self.state.you.head.shift(mv)) {
            "gulp"
        } else {
            ""
        };

        Ok(Command {
            mv: *mv,
            shout: shout.to_string(),
        })
    }

    fn process<F>(&self, process: &str, moves: Vec<Move>, f: F) -> Vec<Move>
    where
        F: Fn(Point) -> bool,
    {
        let before = moves.clone();

        let after: Vec<Move> = moves
            .into_iter()
            .filter(|mv| f(self.state.you.head.shift(&mv)))
            .collect();

        if after.is_empty() {
            println!(
                "game {}, turn {}, {}: skipping because empty",
                self.state.game.id, self.state.turn, process
            );
            before
        } else if before == after {
            println!(
                "game {}, turn {}, {}: no changes",
                self.state.game.id, self.state.turn, process
            );
            after
        } else {
            println!(
                "game {}, turn {}, {}: {:?} -> {:?}",
                self.state.game.id, self.state.turn, process, before, after
            );
            after
        }
    }

    fn need_food(&self, distance: i16, hunger_coefficient: f32) -> bool {
        self.state.you.health < 10
            || distance as f32 > self.state.you.health as f32 * hunger_coefficient
    }

    fn compete_for_biggest(&self) -> bool {
        let biggest = self
            .state
            .board
            .snakes
            .iter()
            .filter(|snake| *snake != &self.state.you)
            .map(|snake| snake.length())
            .max();

        biggest.is_some() && self.state.you.length() <= biggest.unwrap()
    }

    fn threatened(&self, point: &Point) -> bool {
        Move::all()
            .iter()
            .map(|mv| point.shift(mv))
            .filter(|point| self.state.board.in_bounds(point))
            .filter(|point| point != &self.state.you.head)
            .filter(|point| {
                self.state
                    .board
                    .snakes
                    .iter()
                    .filter(|snake| snake.length() >= self.state.you.length())
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
            .filter(|point| self.state.board.in_bounds(point))
            .filter(|point| point != &self.state.you.head)
            .filter(|point| {
                self.state
                    .board
                    .snakes
                    .iter()
                    .filter(|snake| snake.length() < self.state.you.length())
                    .any(|snake| point == &snake.head)
            })
            .peekable()
            .peek()
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Ruleset;

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
            hazards: vec![],
            snakes,
        };

        let h = Heuristic {
            state: State {
                game: Game {
                    id: "asdf".to_string(),
                    ruleset: Ruleset {
                        name: "standard".to_string(),
                    },
                },
                turn: 0,
                you: you.clone(),
                board,
            },
        };

        // You
        assert_eq!(h.threatened(&Point::new(6, 3)), false);
        assert_eq!(h.threatened(&Point::new(7, 2)), false);
        assert_eq!(h.threatened(&Point::new(7, 4)), false);

        // Big A, corner
        assert_eq!(h.threatened(&Point::new(1, 0)), true);

        // Same B
        assert_eq!(h.threatened(&Point::new(2, 2)), true);
        assert_eq!(h.threatened(&Point::new(3, 1)), true);
        assert_eq!(h.threatened(&Point::new(4, 2)), true);

        // Lil C
        assert_eq!(h.threatened(&Point::new(1, 8)), false);
        assert_eq!(h.threatened(&Point::new(2, 7)), false);
        assert_eq!(h.threatened(&Point::new(3, 8)), false);

        // Elsewhere
        assert_eq!(h.threatened(&Point::new(4, 4)), false);
    }
}
