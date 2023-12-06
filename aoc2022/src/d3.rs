use std::collections::HashSet;

mod aoc;

fn get_item_prio(c: char) -> u32
{
    if c.is_lowercase()
    {
        return c as u32 - 96
    }
    return c as u32 - 38
}

fn str_to_set(s: &String) -> HashSet<char>
{
    return s.chars().collect();
}

fn part_one()
{
    let lines = aoc::read_lines();

    let mut prio_sum = 0;

    for line in lines
    {
        let halflen = line.len() / 2;

        let first: String = line[..halflen].to_string();
        let second: String = line[halflen..].to_string();

        let set1 = str_to_set(&first);
        let set2 = str_to_set(&second);

        let inter = set1.intersection(&set2).next().unwrap();

        let prio = get_item_prio(*inter);

        println!("{}, {}, {:?}, {}", first, second, inter, prio);

        prio_sum += prio;
    }

    println!("Priority sum: {}", prio_sum);
}

fn part_two()
{
    let lines = aoc::read_lines();

    let mut prio_sum = 0;

    for i in 0..lines.len()
    {
        if i % 3 > 0
        {
            continue;
        }

        let l1 : &String = &lines[i];
        let l2 : &String = &lines[i+1];
        let l3 : &String = &lines[i+2];

        let sets = vec![str_to_set(l1), str_to_set(l2), str_to_set(l3)];

        let mut iter = sets.iter();

        // black magic
        let common = iter
            .next()
            .map(|set| {
                iter.fold(set.clone(), |set1, set2| {
                    set1.intersection(&set2).cloned().collect()
                })
            }).unwrap();

        let chr = common.iter().next().unwrap();

        let prio = get_item_prio(*chr);

        println!("{}, {}", chr, prio);

        prio_sum += prio;
    }

    println!("Priority sum: {}", prio_sum);
}

fn main()
{
    // part_one();
    part_two();
}