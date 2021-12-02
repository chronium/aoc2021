#[cfg(test)]
mod tests {
    use crate::{day2, day2::Move};

    const FIRST_INPUT: [&'static str; 6] = [
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

        let result = day2::parse_first(&FIRST_INPUT);

        assert_eq!(result, expected);
    }

    #[test]
    fn apply_first() {
        let moves = day2::parse_first(&FIRST_INPUT);

        let mut result = vec![(0, 0)];

        moves
            .iter()
            .for_each(|m| result.push(m.apply(result.last().unwrap())));

        assert_eq!(
            result,
            [(0, 0), (5, 0), (5, 5), (13, 5), (13, 2), (13, 10), (15, 10)]
        );
    }

    #[test]
    fn example_first() {
        let moves = day2::parse_first(&FIRST_INPUT);

        let result = day2::execute_first(moves);

        assert_eq!(result, 150);
    }
}

type Coords = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Move {
    pub fn apply(&self, to: &Coords) -> Coords {
        match &self {
            Self::Forward(n) => (to.0 + n, to.1),
            Self::Down(n) => (to.0, to.1 + n),
            Self::Up(n) => (to.0, to.1 - n),
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

pub fn parse_first(input: &[&str]) -> Vec<Move> {
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
    let mut position = (0, 0);
    input.iter().for_each(|m| position = m.apply(&position));

    position.0 * position.1
}
