use super::mv::Move;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point { x, y }
    }

    pub fn shift(&self, mv: &Move) -> Point {
        match mv {
            Move::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Move::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
            Move::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Move::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    pub fn distance(&self, other: &Point) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn towards(&self, other: &Point) -> Vec<Move> {
        let mut pairs: Vec<(Move, i16)> = Move::all()
            .into_iter()
            .map(|mv| (mv, self.shift(&mv).distance(other)))
            .collect();

        pairs.sort_by(|(_, distance_a), (_, distance_b)| distance_a.cmp(distance_b));

        if pairs[0].1 == pairs[1].1 {
            vec![pairs[0].0, pairs[1].0]
        } else {
            vec![pairs[0].0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift() {
        let one_one = Point { x: 1, y: 1 };

        [
            (Move::Up, Point { x: 1, y: 2 }),
            (Move::Down, Point { x: 1, y: 0 }),
            (Move::Left, Point { x: 0, y: 1 }),
            (Move::Right, Point { x: 2, y: 1 }),
        ]
        .iter()
        .for_each(|(mv, point)| {
            assert_eq!(point, &one_one.shift(mv));
        });
    }

    #[test]
    fn distance() {
        assert_eq!(Point { x: 3, y: 3 }.distance(&Point { x: 5, y: 0 }), 5);
    }

    #[test]
    fn towards() {
        let three_three = Point { x: 3, y: 3 };

        assert_eq!(
            three_three.towards(&Point { x: 0, y: 0 }),
            vec![Move::Down, Move::Left]
        );
        assert_eq!(three_three.towards(&Point { x: 0, y: 3 }), vec![Move::Left]);
        assert_eq!(
            three_three.towards(&Point { x: 0, y: 5 }),
            vec![Move::Up, Move::Left]
        );
        assert_eq!(three_three.towards(&Point { x: 3, y: 5 }), vec![Move::Up]);
        assert_eq!(
            three_three.towards(&Point { x: 5, y: 5 }),
            vec![Move::Up, Move::Right]
        );
        assert_eq!(
            three_three.towards(&Point { x: 5, y: 3 }),
            vec![Move::Right]
        );
        assert_eq!(
            three_three.towards(&Point { x: 5, y: 0 }),
            vec![Move::Down, Move::Right]
        );
        assert_eq!(three_three.towards(&Point { x: 3, y: 0 }), vec![Move::Down]);
    }
}
