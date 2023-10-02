use std::cmp::max;

mod aoc;

fn parse_line(line: &str) -> Vec<u8>
{
    return line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
}

fn is_visible(r: usize, c: usize, trees: &Vec<Vec<u8>>) -> bool
{
    if r == 0 || c == 0 || r + 1 == trees.len() || c + 1 == trees.len()
    {
        return true;
    }

    let size = trees.len();
    let focus_height = trees[r][c];

    // r is vertical index, c is horizontal

    let north_occluded : bool = (0..r)
        .map(|i| trees[i][c])
        .any(|h| h >= focus_height);

    let south_occluded : bool = (r+1..size)
        .map(|i| trees[i][c])
        .any(|h| h >= focus_height);

    let west_occluded : bool = (0..c)
        .map(|i| trees[r][i])
        .any(|h| h >= focus_height);

    let east_occluded : bool = (c+1..size)
        .map(|i| trees[r][i])
        .any(|h| h >= focus_height);

    return !north_occluded ||
           !south_occluded ||
           !east_occluded ||
           !west_occluded;
}

fn scenic_score(r: usize, c: usize, trees: &Vec<Vec<u8>>) -> usize
{
    let mut count = 0;
    let mut score = 1;

    let size = trees.len();
    let focus_height = trees[r][c];

    for i in (0..r).rev()
    {
        let h = trees[i][c];
        count += 1;
        if h >= focus_height
        {
            break;
        }
    }

    score *= count;
    count = 0;

    for i in r+1..size
    {
        let h = trees[i][c];
        count += 1;
        if h >= focus_height
        {
            break;
        }
    }

    score *= count;
    count = 0;

    for i in (0..c).rev()
    {
        let h = trees[r][i];
        count += 1;
        if h >= focus_height
        {
            break;
        }
    }

    score *= count;
    count = 0;

    for i in c+1..size
    {
        let h = trees[r][i];
        count += 1;
        if h >= focus_height
        {
            break;
        }
    }

    score *= count;
    return score;
}

fn main()
{
    let lines = aoc::read_lines();
    let trees : Vec<Vec<u8>> = lines.iter()
        .map(|l| parse_line(&l)).collect();
    let size = trees.len();

    let mut count = 0;
    let mut best_score = 0;

    for i in 0..size
    {
        for j in 0..size
        {
            let score = scenic_score(i, j, &trees);
            best_score = max(score, best_score);

            println!("{} {} {} {}", i, j, trees[i][j], score);

            if is_visible(i, j, &trees)
            {
                count += 1;
            }
        }
    }
    println!("Trees: {}", count);
    println!("Best score: {}", best_score);
}
