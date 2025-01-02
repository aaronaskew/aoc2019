fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap() / 3 - 2)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("");

        assert_eq!(result, "");
    }
}
