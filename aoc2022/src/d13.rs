use std::cmp::{max, Ordering};

mod aoc;

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone)]
enum Node
{
    Num(i32),
    List(Vec<Node>)
}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        let c = compare(self, other);
        return match c
        {
            Some(true)  => Ordering::Less,
            Some(false) => Ordering::Greater,
            None        => Ordering::Equal
        };
    }
}

fn split_top_level(line: &str) -> Vec<String>
{
    let mut tokens : Vec<String> = vec![];

    let mut level = 0;
    let mut start_idx = 0;
    for (i, c) in line.chars().enumerate()
    {
        let mut push_token = false;

        match c
        {
            '[' => { level += 1; },
            ']' =>
            {
                level -= 1;
            },
            ',' =>
            {
                push_token = level == 0;
            },
            _   => ()
        }

        if push_token
        {
            let t = line[start_idx..i].to_string();
            start_idx = i + 1;
            tokens.push(t);
        }
    }

    let t = line[start_idx..line.len()].to_string();
    tokens.push(t);

    return tokens.into_iter().filter(|t| !t.is_empty()).collect();
}

fn parse_line(line: &str) -> Option<Node>
{
    // println!("Parse: {}", &line);

    if line.is_empty()
    {
        return None;
    }

    // this might be a number, let's try it
    match line.parse().ok()
    {
        Some(num) => return Some(Node::Num(num)),
        _         => ()
    }

    let substr = &line[1..line.len() - 1];

    let tokens = split_top_level(&substr);

    let nodes : Vec<Node> = tokens.iter().map(|t| parse_line(t).unwrap()).collect();

    return Some(Node::List(nodes));
}

fn parse_line_debug(line: &str) -> Option<Node>
{
    println!("Parsing: {}", &line);
    let ret = parse_line(&line);
    println!("Result: {:?}", &ret);
    return ret;
}

#[test]
fn run_tests()
{
    assert_eq!(parse_line_debug("6"),    Some(Node::Num(6)));
    assert_eq!(parse_line_debug("12"),   Some(Node::Num(12)));

    assert_eq!(
        parse_line_debug("[12,14]"),
        Some(
            Node::List
            (
                vec![Node::Num(12), Node::Num(14)]
            )
        )
    );

    assert_eq!(
        parse_line_debug("[12,14,[3]]"),
        Some(
            Node::List
            (
                vec!
                [
                    Node::Num(12),
                    Node::Num(14),
                    Node::List
                    (
                        vec![Node::Num(3)]
                    )
                ]
            )
        )
    );

    assert_eq!(
        parse_line_debug("[[1],[2,3,4]]"),
        Some(
            Node::List
            (
                vec!
                [
                    Node::List
                    (
                        vec!
                        [
                            Node::Num(1)
                        ]
                    ),
                    Node::List
                    (
                        vec!
                        [
                            Node::Num(2),
                            Node::Num(3),
                            Node::Num(4)
                        ]
                    )
                ]
            )
        )
    );

    assert_eq!(
        parse_line_debug("[[[]]]"),
        Some
        (
            Node::List
            (
                vec!
                [
                    Node::List
                    (
                        vec!
                        [
                            Node::List(vec![])
                        ]
                    )
                ]
            )
        )
    );

    assert_eq!(
        parse_line_debug("[[[3]]]"),
        Some
        (
            Node::List
            (
                vec!
                [
                    Node::List
                    (
                        vec!
                        [
                            Node::List(vec![Node::Num(3)])
                        ]
                    )
                ]
            )
        )
    );
}

fn compare(left: &Node, right: &Node) -> Option<bool>
{
    println!("Compare {:?} vs {:?}", &left, &right);

    match (left, right)
    {
        (Node::Num(a), Node::Num(b)) =>
        {
            if a < b
            {
                println!("Left is smaller: right order");
                return Some(true);
            }
            if a > b
            {
                println!("Right is smaller: wrong order");
                return Some(false);
            }
            return None
        },
        (Node::List(llist), Node::List(rlist)) =>
        {
            for i in 0..max(llist.len(), rlist.len())
            {
                if i >= llist.len()
                {
                    println!("Left side ran out of items: right order");
                    return Some(true);
                }
                if i >= rlist.len()
                {
                    println!("Right side ran out of items: wrong order");
                    return Some(false);
                }
                let la = &llist[i];
                let ra = &rlist[i];
                let cmp = compare(&la, &ra);
                if cmp.is_some()
                {
                    return cmp;
                }
            }
        },
        (Node::List(list), Node::Num(number)) =>
        {
            println!("Mixed types! Convert right and retry");

            let new_left = Node::List(list.to_vec());
            let new_right = Node::List(vec![Node::Num(*number)]);
            let cmp = compare(&new_left, &new_right);
            if cmp.is_some()
            {
                return cmp;
            }
        },
        (Node::Num(number), Node::List(list)) =>
        {
            println!("Mixed types! Convert left and retry");

            let new_left = Node::List(vec![Node::Num(*number)]);
            let new_right = Node::List(list.to_vec());
            let cmp = compare(&new_left, &new_right);
            if cmp.is_some()
            {
                return cmp;
            }
        }
    }

    return None;
}

fn part_one()
{
    let lines = aoc::read_lines();

    let nodes : Vec<Node> = lines.iter()
        .map(|l| parse_line(&l))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let mut idx = 0;
    let mut sum = 0;
    while idx + 1 < nodes.len()
    {
        let no = idx / 2 + 1;
        let left =  &nodes[idx];
        let right = &nodes[idx+1];
        println!("{} ===================", no);
        let res = compare(&left, &right);
        if res.unwrap_or(false)
        {
            sum += no;
        }
        println!("{:?}\n", res);

        idx += 2;
    }

    println!("Sum: {}", sum);
}

fn part_two()
{
    let lines = aoc::read_lines();

    let mut nodes : Vec<Node> = lines.iter()
        .map(|l| parse_line(&l))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let div1 = parse_line("[[2]]").unwrap();
    let div2 = parse_line("[[6]]").unwrap();

    nodes.push(div1.clone());
    nodes.push(div2.clone());

    nodes.sort_by(|a, b| a.cmp(&b));

    for node in &nodes
    {
        println!("{:?}", node);
    }

    let i1 = nodes.iter().position(|n| *n == div1).unwrap() + 1;
    let i2 = nodes.iter().position(|n| *n == div2).unwrap() + 1;
    println!("{:?} {:?}", i1, i2);
    println!("Product: {}", i1 * i2);
}

fn main()
{
    part_two();
}