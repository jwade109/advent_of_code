#![allow(unused)]

use std::env;
use std::fs::read_to_string;

pub fn read_file(filename: &str) -> Vec<String>
{
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines()
    {
        result.push(line.to_string());
    }

    result
}

pub fn split<'a>(s: &'a str, delim: &str) -> Vec<&'a str>
{
    return s.split(delim).collect::<Vec<&str>>();
}
