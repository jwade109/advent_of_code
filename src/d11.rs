mod aoc;

#[derive(Debug)]
enum Op
{
    AddSelf,
    MulSelf,
    Add(i32),
    Mul(i32)
}

#[derive(Debug)]
struct Monkey
{
    id: i32,
    items: Vec<i32>,
    op: Op,
    test: i32,
    if_true: i32,
    if_false: i32,
    count: i32
}

fn str_to_op(left: &str, op: &str, right: &str) -> Option<Op>
{
    if left == right
    {
        return match op
        {
            "*" => Some(Op::MulSelf),
            "+" => Some(Op::AddSelf),
            _   => None
        };
    }

    let p : i32 = right.parse().ok()?;

    return match op
    {
        "*" => Some(Op::Mul(p)),
        "+" => Some(Op::Add(p)),
        _   => None
    };
}

fn parse_monkeys(lines: &Vec<String>) -> Vec<Monkey>
{
    let mut ret : Vec<Monkey> = vec![];

    let mut i = 0;
    while i + 5 < lines.len()
    {
        let tokens : Vec<Vec<String>> = (1..=5)
            .map(|k| aoc::split(lines[i+k].as_str(), " ")
            .into_iter()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect())
            .collect();

        let start_items = (2..tokens[0].len())
            .map(|i| aoc::split(tokens[0][i].as_str(), ",")[0]
            .parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let op = str_to_op(&tokens[1][3],
            &tokens[1][4], &tokens[1][5]).unwrap();

        let monkey = Monkey
        {
            id: ret.len() as i32,
            items: start_items,
            op: op,
            test: tokens[2][3].parse::<i32>().unwrap(),
            if_true: tokens[3][5].parse().unwrap(),
            if_false: tokens[4][5].parse().unwrap(),
            count: 0
        };

        ret.push(monkey);

        i += 7;
    }

    return ret;
}

fn process_item(mut worry_level: i32, m: &Monkey) -> (i32, i32)
{
    // worry_level %= m.test;

    match m.op
    {
        Op::MulSelf => worry_level *= worry_level,
        Op::AddSelf => worry_level += worry_level,
        Op::Add(x)  => worry_level += x,
        Op::Mul(x)  => worry_level *= x
    };

    worry_level /= 3;

    let dst = if worry_level % m.test == 0
    {
        m.if_true
    }
    else
    {
        m.if_false
    };

    return (worry_level, dst)
}

fn inspect_items(m: &Monkey) -> Vec<(i32, i32)>
{
    return m.items.iter()
        .map(|item| process_item(*item, m))
        .collect();
}

fn main()
{
    let mut monkeys = parse_monkeys(&aoc::read_lines());

    for round in 1..=20
    {
        for i in 0..monkeys.len()
        {
            let results = inspect_items(&monkeys[i]);
            monkeys[i].items.clear();
            monkeys[i].count += results.len() as i32;
            for (item, dst) in results
            {
                let monk = &mut monkeys[dst as usize];
                monk.items.push(item)
            }
        }

        println!("Round {}", round);
        for m in &monkeys
        {
            println!("{} {:?} {}", m.id, m.items, m.count);
        }
    }


    let mut counts : Vec<i32> = monkeys.iter()
        .map(|m| m.count).collect();
    counts.sort();
    counts.reverse();

    let product = counts[0] * counts[1];
    println!("Monkey business: {}", product);
}