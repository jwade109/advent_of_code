use std::env;
use std::fs::read_to_string;

pub fn read_lines() -> Vec<String>
{
    let args : Vec<String> = env::args().collect();

    if args.len() < 2
    {
        panic!("Requires filename as argument.");
    }

    let filename = &args[1];

    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines()
    {
        result.push(line.to_string());
    }

    result
}
