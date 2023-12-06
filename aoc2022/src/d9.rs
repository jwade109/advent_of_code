use std::cmp::max;
use std::collections::HashSet;

mod aoc;

fn parse_line(line: &str) -> (i32, i32)
{
    let splits = aoc::split(&line, " ");
    let count : i32 = splits[1].parse().unwrap();

    return match splits[0]
    {
        "R" => (count , 0),
        "U" => (0,  count),
        "L" => (-count, 0),
        "D" => (0, -count),
        _   => (0, 0)
    };
}

fn get_path(start: (i32, i32), offset: (i32, i32)) -> Vec<(i32, i32)>
{
    let (sx, sy) = start;
    let (mx, my) = offset;

    let dx = mx.signum();
    let dy = my.signum();
    let mag = max(mx.abs(), my.abs());

    return (1..=mag).map(|i| (sx + dx * i, sy + dy * i)).collect::<Vec<(i32, i32)>>();
}

fn update_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32)
{
    let (hx, hy) = head;
    let (tx, ty) = tail;

    let dx = (hx - tx).abs();
    let dy = (hy - ty).abs();

    if dx < 2 && dy < 2
    {
        return tail;
    }

    let sx = (hx - tx).signum();
    let sy = (hy - ty).signum();

    return (tx + sx, ty + sy);
}

fn simulate(lines: &Vec<String>, n: usize)
{
    // n+1 knots are generated

    let mut segments : Vec<(i32, i32)> = (0..=n).map(|_| (0, 0)).collect();

    let mut tail_locations : HashSet<(i32, i32)> = HashSet::new();

    tail_locations.insert(segments[n]);

    for line in lines
    {
        let offset = parse_line(&line);
        let path = get_path(segments[0], offset);

        for coord in path
        {
            segments[0] = coord;
            for i in 0..n
            {
                segments[i+1] = update_tail(segments[i], segments[i+1]);
            }

            tail_locations.insert(segments[n]);
            // println!("{:?} {:?}", coord, segments[n])
        }
    }

    println!("Positions visited: {}", tail_locations.len());
}

fn main()
{
    let lines = aoc::read_lines();

    simulate(&lines, 1);
    simulate(&lines, 9);
}
