use std::time::Instant;

use crate::game::Board;
use crate::game::Command;
use crate::game::Move;
use crate::game::Point;
use crate::game::Snake;
use crate::game::State;
use anyhow::Result;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct MonteCarlo {
    pub state: State,
}

struct Node {
    state: State,
    score: u16,
    children: Vec<Node>,
}

impl Node {
    fn new(state: State) -> Node {
        Node {
            state,
            score: 0,
            children: Vec::new(),
        }
    }

    fn add(&mut self, state: State) -> Node {
        let node = Node::new(state);
        self.children.push(node);

        node
    }
}

impl MonteCarlo {
    pub fn new(state: State) -> Self {
        MonteCarlo { state }
    }

    pub fn decide(&self, deadline: Instant) -> Result<Command> {
        let root = Node::new(self.state);

        while Instant::now() < deadline {
            // state = MonteCarlo::next_state(&state);
        }

        Ok(())
    }

    fn next_state(state: &State) -> State {
        let mut board = state.board.clone();

        let snakes: Vec<Snake> = state
            .board
            .snakes
            .iter()
            .map(|snake| {
                let moves = Move::all();
                let mv = moves.choose(&mut thread_rng()).unwrap_or(&Move::Up);
                let head = snake.head.shift(&mv);
                let mut health = snake.health;
                let mut body: Vec<Point> = snake.body.iter().cloned().collect();

                if state.board.food_at(&snake.head) {
                    health = 100;
                    board.food = board
                        .food
                        .iter()
                        .filter(|food| *food == &snake.head)
                        .cloned()
                        .collect();
                } else {
                    health -= 1;
                    body.pop();
                }

                Snake {
                    body: [vec![head], body].concat(),
                    health,
                    head,
                    ..snake.clone()
                }
            })
            .collect();

        let you = snakes
            .iter()
            .find(|snake| **snake == state.you)
            .expect("")
            .clone();

        let mut food_openings: Vec<Point> = Vec::new();
        for x in 0..board.width {
            for y in 0..board.height {
                let point = Point { x, y };
                food_openings.push(point);
            }
        }

        food_openings = food_openings
            .into_iter()
            .filter(|point| state.board.snake_at(point).is_none())
            .filter(|point| !state.board.food_at(point))
            .collect();

        let mut food = board.food.clone();
        let new_food = food_openings.choose(&mut thread_rng());
        if let Some(new_food) = new_food {
            food.push(*new_food);
        }

        State {
            you,
            turn: state.turn + 1,
            board: Board {
                food,
                snakes,
                ..board
            },
            ..state.clone()
        }
    }
}
