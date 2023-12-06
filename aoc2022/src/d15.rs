use regex::Regex;
use std::collections::HashSet;
use std::ops::Range;

mod aoc;

type Coord = (i32, i32);

fn parse_line(line: &str) -> (Coord, Coord)
{
    let re = Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        ).unwrap();

    let caps = re.captures(line).unwrap();

    let sx : i32 = caps[1].parse().unwrap();
    let sy : i32 = caps[2].parse().unwrap();
    let bx : i32 = caps[3].parse().unwrap();
    let by : i32 = caps[4].parse().unwrap();

    return ((sx, sy), (bx, by));
}

fn manhattan_dist(a: Coord, b: Coord) -> i32
{
    return (b.0 - a.0).abs() + (b.1 - a.1).abs();
}

fn test_row(row: i32, sensor: &Coord, beacon: &Coord) -> Range<i32>
{
    let md = manhattan_dist(*sensor, *beacon);
    let dy = (row - sensor.1).abs();
    if dy >= md
    {
        return 0..0;
    }
    let delta = md - dy;
    return sensor.0 - delta..sensor.0 + delta + 1;
}

fn eval_impossible_positions(row: i32, sensors: &Vec<Coord>, beacons: &Vec<Coord>) -> i32
{
    let mut impossible = HashSet::new();

    for i in 0..sensors.len()
    {
        let sensor = &sensors[i];
        let beacon = &beacons[i];

        let test = test_row(row, &sensor, &beacon);
        for x in test
        {
            if beacons.contains(&(x, row))
            {
                continue;
            }
            impossible.insert(x);
        }
    }

    return impossible.len() as i32;
}

fn part_one(sensors: &Vec<Coord>, beacons: &Vec<Coord>)
{
    let im_10      = eval_impossible_positions(10,      &sensors, &beacons);
    let im_2000000 = eval_impossible_positions(2000000, &sensors, &beacons);

    println!("y = 10:      {}", im_10);
    println!("y = 2000000: {}", im_2000000);
}

// fn part_two(sensors: &Vec<Coord>, beacons: &Vec<Coord>)
// {
//     // TODO
// }

fn main()
{
    let lines = aoc::read_lines();

    let mut sensors = vec![];
    let mut beacons = vec![];

    for line in lines
    {
        let (sensor, beacon) = parse_line(&line);
        sensors.push(sensor);
        beacons.push(beacon);
    }

    part_one(&sensors, &beacons);
    // part_two(&sensors, &beacons);
}