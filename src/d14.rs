use std::cmp::{min, max};
use std::collections::HashSet;

mod aoc;

type Coord = (i32, i32);
type Line = Vec<Coord>;

static SAND_START : (i32, i32) = (500, 0);

#[derive(Debug)]
struct Cave
{
    rocks: HashSet<Coord>,
    sand:  HashSet<Coord>
}

fn parse_line(line: &str) -> Option<Line>
{
    if line.is_empty()
    {
        return None;
    }

    let mut ret = vec![];
    let tokens = aoc::split(&line, " -> ");
    for token in tokens
    {
        let subtokens = aoc::split(&token, ",");
        let x : i32 = subtokens[0].parse().unwrap();
        let y : i32 = subtokens[1].parse().unwrap();
        ret.push((x, y));
    }
    return Some(ret);
}

fn sparse_line_to_dense(line: &Line) -> Line
{
    let mut ret = vec![];

    for i in 0..line.len() - 1
    {
        let a = &line[i];
        let b = &line[i+1];

        let dx = b.0 - a.0;
        let dy = b.1 - a.1;

        for j in 0..=max(dx.abs(), dy.abs())
        {
            let p = (a.0 + dx.signum() * j, a.1 + dy.signum() * j);
            if !ret.contains(&p)
            {
                ret.push(p);
            }
        }
    }

    return ret;
}

fn get_cave_height(cave: &Cave) -> i32
{
    let mut maxy = 0;
    for (_, y) in &cave.rocks
    {
        maxy = max(maxy, *y);
    }
    return maxy;
}

fn draw_cave(cave: &Cave)
{
    let mut xrange = (i32::MAX, 0);
    for (x, _) in &cave.rocks
    {
        xrange.0 = min(xrange.0, *x);
        xrange.1 = max(xrange.1, *x);
    }
    for (x, _) in &cave.sand
    {
        xrange.0 = min(xrange.0, *x);
        xrange.1 = max(xrange.1, *x);
    }

    for y in 0..=get_cave_height(&cave) + 3
    {
        for x in xrange.0..=xrange.1
        {
            if (x, y) == SAND_START
            {
                print!("+");
            }
            else if cave.sand.contains(&(x, y))
            {
                print!("O");
            }
            else if cave.rocks.contains(&(x, y))
            {
                print!("#");
            }
            else
            {
                print!(".");
            }
        }
        println!();
    }

    println!("Sand: {}", cave.sand.len());
}

fn down(c: Coord) -> Coord
{
    return (c.0, c.1 + 1);
}

fn left(c: Coord) -> Coord
{
    return (c.0 - 1, c.1);
}

fn right(c: Coord) -> Coord
{
    return (c.0 + 1, c.1);
}

#[derive(Debug, PartialEq)]
enum Support
{
    MiddleFree,
    LeftFree,
    RightFree,
    All
}

fn is_occupied(cave: &Cave, coord: &Coord) -> bool
{
    return cave.rocks.contains(coord) || cave.sand.contains(coord);
}

fn support_status(cave: &Cave, coord: &Coord) -> Support
{
    let b = down(*coord);
    let l = left(b);
    let r = right(b);

    if !is_occupied(&cave, &b)
    {
        return Support::MiddleFree;
    }
    if !is_occupied(&cave, &l)
    {
        return Support::LeftFree;
    }
    if !is_occupied(&cave, &r)
    {
        return Support::RightFree;
    }
    return Support::All;
}

fn add_sand(cave: &Cave) -> Option<Coord>
{
    let mut sand = SAND_START;
    let mut sup = support_status(&cave, &sand);
    let maxy = get_cave_height(&cave);

    while sup != Support::All
    {
        sand = match sup
        {
            Support::All        => sand,
            Support::LeftFree   => left(down(sand)),
            Support::MiddleFree => down(sand),
            Support::RightFree  => right(down(sand))
        };

        sup = support_status(&cave, &sand);

        if sand.1 > maxy
        {
            // return None; // part 1
            return Some(sand);
        }
    }

    return Some(sand);
}

fn main()
{
    let lines = aoc::read_lines();

    let parsed : Vec<_> = lines.iter()
        .map(|l| sparse_line_to_dense(&parse_line(&l).unwrap()))
        .collect();

    let mut cave = Cave
    {
        rocks: HashSet::new(),
        sand: HashSet::new()
    };

    for line in parsed
    {
        cave.rocks.extend(line);
    }

    draw_cave(&cave);

    loop
    {
        let s = add_sand(&cave);
        match s
        {
            Some(c) =>
            {
                cave.sand.insert(c);
                if c == SAND_START
                {
                    break;
                }
            },
            None => break
        }

    }

    draw_cave(&cave);
}
