use core::cmp::max;

mod aoc;

#[derive(Debug)]
struct Move
{
    quantity: u32,
    src: u32,
    dst: u32
}

fn parse_crate_line(line: &str) -> Vec<char>
{
    let mut ret = Vec::new();
    let mut i = 1;
    while i < line.len()
    {
        let c = line.chars().collect::<Vec<char>>()[i];
        ret.push(c);
        i += 4;
    }
    return ret;
}

fn parse_move_line(line: &str) -> Option<Move>
{
    let splits = aoc::split(&line, " ");
    return Some(Move
    {
        quantity: splits[1].parse().ok()?,
        src:      splits[3].parse().ok()?,
        dst:      splits[5].parse().ok()?
    })
}

fn take_boxes(src: &mut Vec<char>, count: u32) -> Vec<char>
{
    let mut res = vec![];
    for _ in 0..count
    {
        let c = src.pop().unwrap();
        res.push(c);
    }
    return res;
}

fn put_boxes(src: &Vec<char>, dst: &mut Vec<char>)
{
    for c in src
    {
        dst.push(*c);
    }
}

fn stack_the_boxes(boxes: &Vec<Vec<char>>) -> Vec<Vec<char>>
{
    let mut ret : Vec<Vec<char>> = vec![];

    let mut maxlen = 0;

    for b in boxes
    {
        maxlen = max(maxlen, b.len());
    }

    for i in 0..maxlen
    {
        ret.push(vec![]);
        for b in boxes
        {
            if b.len() <= i
            {
                continue;
            }
            let c = b[i];
            if c != ' '
            {
                ret[i].push(c);
            }
        }
    }

    for mut b in &mut ret
    {
        b.reverse()
    }

    return ret;
}

fn main()
{
    let lines = aoc::read_lines();

    let mut finished_boxes = false;

    let mut unstacked : Vec<Vec<char>> = Vec::new();
    let mut stacks    : Vec<Vec<char>> = Vec::new();

    for line in lines
    {
        if line.is_empty()
        {
            finished_boxes = true;
            unstacked.pop();
            stacks = stack_the_boxes(&unstacked);
            println!("{:?}", stacks);
            continue;
        }

        if finished_boxes
        {
            let m = parse_move_line(&line).unwrap();
            println!("{}, {}, {}", m.quantity, m.src, m.dst);

            println!("{:?}", stacks);

            let mut s = &mut stacks[(m.src - 1) as usize];
            let b = take_boxes(&mut s, m.quantity);
            s = &mut stacks[(m.dst - 1) as usize];
            put_boxes(&b, &mut s);
            continue;
        }

        let boxes = parse_crate_line(&line);
        unstacked.push(boxes);
    }

    println!("{:?}", stacks);

    for s in stacks
    {
        print!("{}", s[s.len() - 1]);
    }
    println!();
}
