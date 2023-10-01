mod aoc;

fn is_fully_unique(s: &str) -> bool
{
    let mut count = 0;
    for a in s.chars()
    {
        for b in s.chars()
        {
            if a == b
            {
                count += 1;
            }
        }
    }
    return count == s.len();
}

fn index_of_first_unique_substr(s: &str, target_len: usize) -> Option<usize>
{
    for i in target_len..s.len()
    {
        let substr = &s[i-target_len..i];
        if is_fully_unique(&substr)
        {
            return Some(i);
        }
    }
    return None;
}

fn main()
{
    let lines = aoc::read_lines();

    for line in lines
    {
        println!("{}", line);
        println!("Index of first packet marker: {:?}", index_of_first_unique_substr(&line, 4));
        println!("Index of first message marker: {:?}", index_of_first_unique_substr(&line, 14));
    }
}
