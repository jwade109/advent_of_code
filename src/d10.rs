mod aoc;

#[derive(Debug, Clone, Copy)]
enum Op
{
    Noop,
    Addx(i32)
}

fn parse_line(line: &str) -> Option<Op>
{
    let tokens = aoc::split(line, " ");

    return match tokens[0]
    {
        "noop" => Some(Op::Noop),
        "addx" => Some(Op::Addx(tokens[1].parse::<i32>().ok()?)),
        _      => None
    };
}

fn main()
{
    let lines = aoc::read_lines();

    let mut cycles : Vec<(String, usize, i32)> = vec![];
    let mut x = 1;

    cycles.push(("".to_string(), cycles.len(), x));

    for line in lines
    {
        let op = parse_line(&line).unwrap();
        println!("{:?}", op);

        match op
        {
            Op::Noop =>
            {
                cycles.push(("noop".to_string(), cycles.len(), x));
            },
            Op::Addx(dx) =>
            {
                cycles.push(("addx".to_string(), cycles.len(), x));
                cycles.push(("addx".to_string(), cycles.len(), x));
                x += dx;
            }
        }
    }

    let mut sum = 0;

    for i in [20, 60, 100, 140, 180, 220]
    {
        let (_, c, x) = cycles[i];
        let product = c as i32 * x;
        println!("{} * {} = {}", c, x, product);
        sum += product;
    }

    println!("Sum of signals: {}", sum);

    for (_, c, x) in cycles
    {
        if c == 0
        {
            continue;
        }

        let m = ((c as i32) - 1) % 40;

        if m >= x - 1 && m <= x + 1
        {
            print!("#");
        }
        else
        {
            print!(".");
        }
        if c % 40 == 0
        {
            println!();
        }
    }
}