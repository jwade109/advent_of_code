mod aoc;

#[derive(Debug, Copy, Clone)]
enum RPS
{
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum WinCon
{
    Win,
    Loss,
    Draw
}

fn parse_rps_strat(strat: &str) -> Option<RPS>
{
    match strat
    {
        "A" => Some(RPS::Rock),
        "B" => Some(RPS::Paper),
        "C" => Some(RPS::Scissors),
        "X" => Some(RPS::Rock),
        "Y" => Some(RPS::Paper),
        "Z" => Some(RPS::Scissors),
        _ => None
    }
}

fn x_beats_y(x: &RPS, y: &RPS) -> bool
{
    match (x, y)
    {
        (RPS::Rock, RPS::Scissors) => true,
        (RPS::Scissors, RPS::Paper) => true,
        (RPS::Paper, RPS::Rock) => true,
        _ => false
    }
}

fn get_shape_score(rps: &RPS) -> i32
{
    match rps
    {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3
    }
}

fn get_score(us: &RPS, them: &RPS) -> i32
{
    let shape_score = get_shape_score(us);

    if x_beats_y(us, them)
    {
        return 6 + shape_score;
    }
    if x_beats_y(them, us)
    {
        return shape_score;
    }
    return 3 + shape_score;
}

fn parse_win_con(ending: &str) -> Option<WinCon>
{
    match ending
    {
        "X" => Some(WinCon::Loss),
        "Y" => Some(WinCon::Draw),
        "Z" => Some(WinCon::Win),
        _ => None
    }
}

fn ensure_ending(them: &RPS, ending: &WinCon) -> RPS
{
    match (them, ending)
    {
        (_, WinCon::Draw) => *them,
        (RPS::Rock,     WinCon::Win)  => RPS::Paper,
        (RPS::Paper,    WinCon::Win)  => RPS::Scissors,
        (RPS::Scissors, WinCon::Win)  => RPS::Rock,
        (RPS::Paper,    WinCon::Loss) => RPS::Rock,
        (RPS::Scissors, WinCon::Loss) => RPS::Paper,
        (RPS::Rock,     WinCon::Loss) => RPS::Scissors,
    }
}

fn main()
{
    let lines = aoc::read_lines();

    let mut total_score_p1 : i32 = 0;
    let mut total_score_p2 : i32 = 0;

    for line in lines
    {
        let splits = line.split(" ").collect::<Vec<&str>>();

        let them = parse_rps_strat(splits[0]).unwrap();
        let us = parse_rps_strat(splits[1]).unwrap();
        let score_p1 = get_score(&us, &them);

        let ending = parse_win_con(splits[1]).unwrap();
        let best_move = ensure_ending(&them, &ending);
        let score_p2 = get_score(&best_move, &them);

        println!("{:?} -> {:?}, {}", them, us, score_p1);
        println!("{:?} -> {:?}, {}", them, best_move, score_p2);

        total_score_p1 += score_p1;
        total_score_p2 += score_p2;
    }

    println!("Total score (part 1): {}", total_score_p1);
    println!("Total score (part 2): {}", total_score_p2);
}
