mod aoc;

#[derive(Debug)]
struct Race
{
    time: u64,
    dist: u64
}

fn compute_win_paths(r: &Race) -> usize
{
    (0..=r.time).map(|v|
    {
        v * (r.time - v)
    })
    .filter(|t| t > &r.dist).count()
}

fn get_races(lines: &[String]) -> Vec<Race>
{
    let times = aoc::split(&lines[0], " ");
    let dists = aoc::split(&lines[1], " ");

    (1..times.len()).map(|i|
    {
        Race
        {
            time: times[i].parse().unwrap(),
            dist: dists[i].parse().unwrap()
        }
    })
    .collect()
}

fn get_race_p2(lines: &[String]) -> Race
{
    let times = aoc::split(&lines[0], " ");
    let dists = aoc::split(&lines[1], " ");

    Race
    {
        time: (1..times.len()).map(|i| times[i]).collect::<String>().parse().unwrap(),
        dist: (1..times.len()).map(|i| dists[i]).collect::<String>().parse().unwrap(),
    }
}

fn part_one(lines: &[String]) -> u64
{
    let races = get_races(lines);
    races.iter().map(|r| compute_win_paths(r)).product::<usize>() as u64
}

fn part_two(lines: &[String]) -> u64
{
    let race = get_race_p2(lines);
    [race].iter().map(|r| compute_win_paths(r)).product::<usize>() as u64
}

#[test]
fn assert_solutions()
{
    let l1 = aoc::read_file("examples/d6.txt");
    assert_eq!(part_one(&l1), 288);

    let l2 = aoc::read_file("examples/d6.txt");
    assert_eq!(part_two(&l2), 71503);
}

fn main()
{
    let lines = aoc::read_file("inputs/d6.txt");
    let s1 = part_one(&lines);
    let s2 = part_two(&lines);
    dbg!(s1);
    dbg!(s2);
}
