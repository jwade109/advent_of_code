mod aoc;

fn ab_contains_xy(a: i32, b: i32, x: i32, y: i32) -> bool
{
    return a <= x && b >= y;
}

fn overlap_at_all(a: i32, b: i32, x: i32, y: i32) -> bool
{
    return (x <= a && a <= y) ||
           (x <= b && b <= y) ||
           (a <= x && x <= b) ||
           (a <= y && y <= b);
}

fn main()
{
    let lines = aoc::read_lines();

    let mut total_overlap = 0;
    let mut partial_overlap = 0;

    for line in lines
    {
        let split = aoc::split(&line, ",");
        let left  = aoc::split(&split[0], "-");
        let right = aoc::split(&split[1], "-");

        let a : i32 = left[0].parse().unwrap();
        let b : i32 = left[1].parse().unwrap();
        let x : i32 = right[0].parse().unwrap();
        let y : i32 = right[1].parse().unwrap();

        if ab_contains_xy(a, b, x, y) || ab_contains_xy(x, y, a, b)
        {
            total_overlap += 1;
        }

        if overlap_at_all(a, b, x, y)
        {
            partial_overlap += 1;
        }
    }

    println!("Total: {}", total_overlap);
    println!("Partial: {}", partial_overlap);
}