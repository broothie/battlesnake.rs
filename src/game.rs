use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct State {
	game: Game,
	pub turn: u16,
	board: Board,
	you: Snake,
}

impl State {
	pub fn decide(&self) -> Result<Move> {
		let mut moves = Move::all();

		moves = moves
			.into_iter()
			.filter(|mv| self.board.in_bounds(&self.you.head.shift(mv)))
			.filter(|mv| !self.board.snakes.iter().any(|snake| snake.at(&self.you.head.shift(mv), true)))
			.filter(|mv| !self.threatened(&self.you.head.shift(mv)))
			.collect();

		println!("moves {:?}", moves);
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
			.filter(|point| *point != self.you.head)
			.filter(|point| self.board.snakes.iter().any(|snake| *point == snake.head))
			.peekable()
			.peek()
			.is_some()
	}
}

#[derive(Deserialize, Debug)]
pub struct Game {
	id: String,
}

#[derive(Deserialize, Debug)]
pub struct Board {
	height: i16,
	width: i16,
	food: Vec<Point>,
	snakes: Vec<Snake>,
}

impl Board {
	fn in_bounds(&self, point: &Point) -> bool {
		0 <= point.x && point.x < self.width && 0 <= point.y && point.y < self.height
	}
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Point {
	x: i16,
	y: i16,
}

impl Point {
	fn shift(&self, mv: &Move) -> Point {
		match mv {
			Move::Up => Point { x: self.x, y: self.y + 1 },
			Move::Down => Point { x: self.x, y: self.y - 1 },
			Move::Left => Point { x: self.x - 1, y: self.y },
			Move::Right => Point { x: self.x + 1, y: self.y },
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct Snake {
	id: String,
	name: String,
	health: u16,
	body: Vec<Point>,
	head: Point,
	shout: String,
}

impl Snake {
	fn at(&self, point: &Point, ignore_tail: bool) -> bool {
		if ignore_tail {
			let mut body = self.body.clone();
			body.remove(body.len() - 1);

			body.contains(point)
		} else {
			self.body.contains(point)
		}
	}
}

#[derive(Serialize, Debug)]
pub struct Command {
	#[serde(rename = "move")]
	pub mv: Move,
}

#[derive(Serialize, PartialEq, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Move {
	Up,
	Down,
	Left,
	Right,
}

impl Move {
	fn all() -> Vec<Self> {
		vec![Move::Up, Move::Down, Move::Left, Move::Right]
	}
}
