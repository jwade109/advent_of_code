use std::collections::HashSet;

mod aoc;

#[derive(Debug)]
enum ParseResult
{
    ChangeDir(String),
    ListDir,
    File(FileInfo)
}

#[derive(Debug)]
struct FileInfo
{
    size: usize,
    name: String
}

fn parse_line(line: &str) -> Option<ParseResult>
{
    if line.is_empty()
    {
        return None;
    }

    let splits = aoc::split(&line, " ");

    if splits[0] == "$"
    {
        let cmd = match splits[1]
        {
            "cd" => ParseResult::ChangeDir(splits[2].to_string()),
            "ls" => ParseResult::ListDir,
            _ => return None
        };

        return Some(cmd);
    }

    let fi = FileInfo
    {
        size: splits[0].parse().ok()?,
        name: splits.get(1)?.to_string()
    };

    return Some(ParseResult::File(fi));
}

fn dir_contains_file(dir: &str, file: &str) -> bool
{
    if dir == "/"
    {
        return true;
    }
    let d_with_slash = dir.to_owned() + "/";
    return file.starts_with(&d_with_slash);
}

fn size_of_directory(dir: &str, files: &Vec<FileInfo>) -> usize
{
    let mut s = 0;
    for file in files
    {
        if dir_contains_file(&dir, &file.name)
        {
            s += file.size;
        }
    }
    return s;
}

fn main()
{
    let lines = aoc::read_lines();

    let mut path : Vec<String> = vec![];

    let mut all_dirs  : HashSet<String> = HashSet::new();
    let mut all_files : Vec<FileInfo> = vec![];

    for line in lines
    {
        let x = parse_line(&line);

        match x
        {
            Some(ParseResult::ChangeDir(c)) =>
            {
                // println!("Change dir: {:?}", c);

                match c.as_str()
                {
                    ".." => { path.pop(); },
                    "/"  => path = vec![],
                    _    => path.push(c)
                }

                all_dirs.insert("/".to_owned() + &path.join("/"));

                // println!("Current path is now {:?}", path);
            },
            Some(ParseResult::ListDir) =>
            {
                // println!("List dir");
            },
            Some(ParseResult::File(f)) =>
            {
                path.push(f.name.clone());

                // println!("/{} {}", path.join("/"), f.size);

                let fi = FileInfo {
                    name: "/".to_owned() + &path.join("/"),
                    size: f.size
                };

                path.pop();

                all_files.push(fi);
            },
            None => ()
        }
    }

    let mut sum_sizes = 0;

    for dir in &all_dirs
    {
        let s = size_of_directory(&dir, &all_files);
        println!("{}: {}", dir, s);
        if s <= 100000
        {
            sum_sizes += s;
        }
    }
    println!("Sum of sizes: {}", sum_sizes);

    let total_disk_size = 70000000;
    let req_unused = 30000000;

    let current_usage = size_of_directory("/", &all_files);
    let current_unused = total_disk_size - current_usage;

    let must_free_up = req_unused - current_unused;

    let mut best_size = current_usage;
    let mut best_dir: String = "/".to_string();

    for dir in all_dirs
    {
        let s = size_of_directory(&dir, &all_files);
        if s >= must_free_up && s < best_size
        {
            best_size = s;
            best_dir = dir.clone();
        }
    }

    println!("Unused: {}, need to free up {}", current_unused, must_free_up);
    println!("Deleting {} will free up {}", best_dir, best_size);
}
