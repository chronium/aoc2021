#[cfg(test)]
mod tests {
    use crate::{day2, day2::Move};

    const INPUT: [&'static str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn parse_first() {
        let expected = vec![
            Move::Forward(5),
            Move::Down(5),
            Move::Forward(8),
            Move::Up(3),
            Move::Down(8),
            Move::Forward(2),
        ];

        let result = day2::parse(&INPUT);

        assert_eq!(result, expected);
    }

    #[test]
    fn apply_first() {
        let moves = day2::parse(&INPUT);

        let mut result = vec![(0, 0)];

        moves
            .iter()
            .for_each(|m| result.push(m.apply_first(result.last().unwrap())));

        assert_eq!(
            result,
            [(0, 0), (5, 0), (5, 5), (13, 5), (13, 2), (13, 10), (15, 10)]
        );
    }

    #[test]
    fn example_first() {
        let moves = day2::parse(&INPUT);

        let result = day2::execute_first(moves);

        assert_eq!(result, 150);
    }

    #[test]
    fn apply_second() {
        let moves = day2::parse(&INPUT);

        let mut result = vec![((0, 0), 0)];

        moves
            .iter()
            .for_each(|m| result.push(m.apply_second(result.last().unwrap())));

        assert_eq!(
            result,
            [
                ((0, 0), 0),
                ((5, 0), 0),
                ((5, 0), 5),
                ((13, 40), 5),
                ((13, 40), 2),
                ((13, 40), 10),
                ((15, 60), 10)
            ]
        );
    }

    #[test]
    fn example_second() {
        let moves = day2::parse(&INPUT);

        let result = day2::execute_second(moves);

        assert_eq!(result, 900);
    }
}

type Coords = (i32, i32);
type CoordsAim = (Coords, i32);

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Move {
    pub fn apply_first(&self, to: &Coords) -> Coords {
        match &self {
            Self::Forward(n) => (to.0 + n, to.1),
            Self::Down(n) => (to.0, to.1 + n),
            Self::Up(n) => (to.0, to.1 - n),
        }
    }
    pub fn apply_second(&self, to: &CoordsAim) -> CoordsAim {
        let coords = to.0;
        let aim = to.1;
        match &self {
            Self::Forward(n) => ((coords.0 + n, coords.1 + n * aim), aim),
            Self::Down(n) => (coords, aim + n),
            Self::Up(n) => (coords, aim - n),
        }
    }

    pub fn parse(dir: &str, count: i32) -> Self {
        match dir.to_lowercase().as_ref() {
            "forward" => Move::Forward(count),
            "down" => Move::Down(count),
            "up" => Move::Up(count),
            s => unreachable!("{}", s),
        }
    }
}

pub fn parse(input: &[&str]) -> Vec<Move> {
    input
        .iter()
        .map(|i| i.split(" "))
        .map(|mut split| {
            (
                split.next().unwrap().clone(),
                i32::from_str_radix(split.next().unwrap(), 10).unwrap(),
            )
        })
        .map(|(dir, count)| Move::parse(dir, count))
        .collect::<Vec<_>>()
}

pub fn execute_first(input: Vec<Move>) -> i32 {
    CoordsOps(input.iter().fold(CoordsOps::zero(), |position, mov| {
        mov.apply_first(&position)
    }))
    .mul()
}

pub fn execute_second(input: Vec<Move>) -> i32 {
    CoordsAimOps(
        input
            .iter()
            .fold(CoordsAimOps::zero(), |position_aim, mov| {
                mov.apply_second(&position_aim)
            }),
    )
    .mul()
}

pub struct CoordsOps(Coords);

impl CoordsOps {
    fn mul(&self) -> i32 {
        self.0 .0 * self.0 .1
    }

    fn zero() -> Coords {
        (0, 0)
    }
}

pub struct CoordsAimOps(CoordsAim);

impl CoordsAimOps {
    fn mul(&self) -> i32 {
        self.0 .0 .0 * self.0 .0 .1
    }

    fn zero() -> CoordsAim {
        ((0, 0), 0)
    }
}
