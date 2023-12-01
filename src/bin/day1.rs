fn main() {
    println!(
        "part 1: {}",
        part1(include_str!("../inputs/day1.txt").to_string())
    );
}

fn part1(input: String) -> String {
    input
        .clone()
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|s| {
            let nums = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let first = s.find(nums).unwrap();
            let last = s.rfind(nums).unwrap();

            let c = s.chars().collect::<Vec<_>>();

            format!("{}{}", &c[first], &c[last]).parse::<i32>().unwrap()
        })
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>()
        .to_string()
}

mod test {
    use rstest::rstest;

    #[rstest]
    #[case(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        "142"
    )]
    fn test1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(crate::part1(input.to_string()), expected);
    }
}
