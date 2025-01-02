fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mass = line.parse::<u32>().unwrap();
            let mut f = 0;
            let mut new_fuel = fuel(mass);
            loop {
                f += new_fuel;
                if new_fuel > 0 {
                    new_fuel = fuel(new_fuel);
                } else {
                    break;
                }
            }
            f

        })
        .sum::<u32>()
        .to_string()
}

fn fuel(mass: u32) -> u32 {
    let fraction = mass / 3;

    if fraction > 2 {
        fraction - 2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("14");

        assert_eq!(result, "2");
    }
}
