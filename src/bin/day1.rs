fn main() {
    println!(
        "part 1: {}",
        part1(include_str!("../inputs/day1.txt").to_string())
    );

    println!(
        "part 2: {}",
        part2(include_str!("../inputs/day1.txt").to_string())
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

fn part2(input: String) -> String {
    input
        .clone()
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|s| {
            let nums = &[
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
                "six", "seven", "eight", "nine",
            ];
            let wordmap = [
                ("one", "1"),
                ("two", "2"),
                ("three", "3"),
                ("four", "4"),
                ("five", "5"),
                ("six", "6"),
                ("seven", "7"),
                ("eight", "8"),
                ("nine", "9"),
            ];

            let mut parsed: Vec<i32> = Vec::new();

            s.char_indices().for_each(|(i, _)| {
                let s = &s[i..];
                for n in nums {
                    if s.starts_with(n) {
                        let intret = match n.parse::<i32>() {
                            Ok(v) => v,
                            Err(_) => {
                                let mut v = 0;
                                for (word, num) in wordmap.iter() {
                                    if n == word {
                                        v = num.parse::<i32>().unwrap();
                                    }
                                }
                                v
                            }
                        };

                        parsed.push(intret);
                    }
                }
            });

            format!("{}{}", parsed.first().unwrap(), parsed.last().unwrap())
                .parse::<i32>()
                .unwrap()
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

    #[rstest]
    #[case(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        "281"
    )]
    fn test2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(crate::part2(input.to_string()), expected);
    }
}
