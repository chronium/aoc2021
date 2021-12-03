#[cfg(test)]
mod tests {
    use crate::day3;

    const INPUT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn example_first() {
        let res = day3::execute_first::<5>(&INPUT);

        assert_eq!(res, 198)
    }
}

pub fn execute_first<const T: usize>(input: &[&str]) -> u32 {
    let mut counts = [(0, 0); T];

    input.iter().for_each(|str| {
        str.char_indices().for_each(|(index, char)| match char {
            '0' => counts[index].0 += 1,
            '1' => counts[index].1 += 1,
            _ => unreachable!(),
        })
    });

    let gamma_string = counts
        .map(|c| if c.0 < c.1 { '1' } else { '0' })
        .iter()
        .collect::<String>();

    let epsilon_string = counts
        .map(|c| if c.0 < c.1 { '0' } else { '1' })
        .iter()
        .collect::<String>();

    let gamma = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}
