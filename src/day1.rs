#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn example_first() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;

        let res = day1::execute_first(&input);

        assert_eq!(res, expected);
    }

    #[test]
    fn example_second() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;

        let res = day1::execute_second(&input);

        assert_eq!(res, expected);
    }
}

pub fn execute_first(input: &[i32]) -> i32 {
    input
        .iter()
        .enumerate()
        .map(|(i, n)| if i > 0 && n > &input[i - 1] { 1 } else { 0 })
        .sum()
}

pub fn execute_second(input: &[i32]) -> i32 {
    let windows = input.windows(3).collect::<Vec<_>>();
    windows
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            if i > 0 && n.iter().sum::<i32>() > windows[i - 1].iter().sum() {
                1
            } else {
                0
            }
        })
        .sum()
}
