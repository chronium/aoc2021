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

    #[test]
    fn example_second() {
        let res = day3::execute_second::<5>(&INPUT);

        assert_eq!(res, 230)
    }
}

fn count(input: &[&str], index: usize) -> (usize, usize) {
    input.iter().fold((0, 0), |counts, str| {
        match str.chars().nth(index).unwrap() {
            '0' => (counts.0 + 1, counts.1),
            '1' => (counts.0, counts.1 + 1),
            _ => unreachable!(),
        }
    })
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

fn calculate_rating<'a, const T: usize, const LEFT: char, const RIGHT: char>(
    mut input: Vec<&'a str>,
) -> &'a str {
    for index in 0..T {
        let (zs, os) = count(&input, index);

        input = input
            .into_iter()
            .filter(|str| str.chars().nth(index).unwrap() == if zs > os { LEFT } else { RIGHT })
            .collect::<Vec<_>>();

        if input.len() == 1 {
            break;
        }
    }

    input[0]
}

pub fn execute_second<const T: usize>(input: &[&str]) -> u32 {
    let o2_rating_str = calculate_rating::<T, '0', '1'>(input.clone().to_vec());
    let co2_rating_str = calculate_rating::<T, '1', '0'>(input.clone().to_vec());

    let o2_rating = u32::from_str_radix(o2_rating_str, 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_rating_str, 2).unwrap();

    o2_rating * co2_rating
}
