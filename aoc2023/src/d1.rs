mod aoc;
use regex_macro::regex;

fn part_one(lines: &[String]) -> u32
{
    let mut sum = 0;

    for line in lines
    {
        let digits : Vec<u32> = line.chars()
            .filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap())
            .collect();

        if digits.is_empty()
        {
            continue;
        }

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let num = first * 10 + last;
        sum += num;
    }

    sum
}

fn str_to_digit(s: &str) -> Option<u32>
{
    match s
    {
        "one"   => Some(1),
        "two"   => Some(2),
        "three" => Some(3),
        "four"  => Some(4),
        "five"  => Some(5),
        "six"   => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine"  => Some(9),
        _       => None
    }
}

fn part_two(lines: &[String]) -> u32
{
    let re = regex!(r"(one|two|three|four|five|six|seven|eight|nine|\d)");

    lines.iter().map(|l| -> u32
    {
        let numbers : Vec<u32> = re.find_iter(l).map(|c|
        {
            let s = c.as_str();
            s.parse::<u32>().unwrap_or_else(|_| str_to_digit(s).unwrap())
        })
        .collect();

        numbers.first().unwrap() * 10 + numbers.last().unwrap()
    })
    .sum()
}

#[test]
fn assert_solutions()
{
    let l1 = aoc::read_file("examples/d1-1.txt");
    assert_eq!(part_one(&l1), 142);

    let l2 = aoc::read_file("examples/d1-2.txt");
    assert_eq!(part_two(&l2), 281);
}

fn main()
{
    let lines = aoc::read_file("inputs/d1.txt");
    let s1 = part_one(&lines);
    let s2 = part_two(&lines);
    dbg!(s1);
    dbg!(s2);
}
