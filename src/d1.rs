use std::cmp::max;

mod aoc;

fn main()
{
    let lines = aoc::read_lines();

    let mut current_count = 0;
    let mut max_count = 0;

    let mut all_counts = Vec::new();

    for line in lines
    {
        if line.is_empty()
        {
            max_count = max(max_count, current_count);
            all_counts.push(current_count);
            current_count = 0;
            continue;
        }

        let i: i32 = line.trim().parse().expect("Expected a number");

        current_count += i;
    }

    println!("Max count: {}", max_count);
    all_counts.sort();
    all_counts.reverse();
    println!("Sum of top three: {}",
        all_counts[0] + all_counts[1] + all_counts[2]);
}

