use super::point::Point;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Snake {
    pub id: String,
    pub health: u16,
    pub body: Vec<Point>,
    pub head: Point,
    // shout: String,
}

impl Snake {
    pub fn tail(&self) -> &Point {
        self.body.last().expect("snake with no tail")
    }

    pub fn at(&self, point: &Point, ignore_tail: bool) -> bool {
        if ignore_tail && point == self.tail() {
            false
        } else {
            self.body.contains(point)
        }
    }
}

impl PartialEq for Snake {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let snake = Snake {
            id: "a".to_string(),
            health: 0,
            head: Point { x: 1, y: 1 },
            body: vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 },
                Point { x: 2, y: 3 },
                Point { x: 2, y: 3 },
            ],
        };

        // Head
        assert_eq!(snake.at(&Point { x: 1, y: 1 }, false), true);
        assert_eq!(snake.at(&Point { x: 1, y: 1 }, true), true);

        // Body
        assert_eq!(snake.at(&Point { x: 1, y: 3 }, false), true);
        assert_eq!(snake.at(&Point { x: 1, y: 3 }, true), true);

        // Tail
        assert_eq!(snake.at(&Point { x: 2, y: 3 }, false), true);
        assert_eq!(snake.at(&Point { x: 2, y: 3 }, true), false);

        // Somewhere else
        assert_eq!(snake.at(&Point { x: 0, y: 0 }, false), false);
        assert_eq!(snake.at(&Point { x: 0, y: 0 }, true), false);
    }
}
