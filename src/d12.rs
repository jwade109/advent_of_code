use std::collections::{HashMap, HashSet};

mod aoc;

type Coord = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell
{
    L(i32),
    S,
    E
}

fn char_to_cell(c: char) -> Cell
{
    return match c
    {
        'S' => Cell::S,
        'E' => Cell::E,
        _   => Cell::L((c as i32) - ('a' as i32) + 1)
    };
}

fn generate_neighbor_coords(r: usize, c: usize, rmax: usize, cmax: usize) -> Vec<Coord>
{
    return match (r, c)
    {
        (0, 0) => vec![(r + 1, c), (r, c + 1)],
        (0, _) => vec![(r + 1, c), (r, c + 1), (r, c - 1)],
        (_, 0) => vec![(r + 1, c), (r, c + 1), (r - 1, c)],
        (_, _) => vec![(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)]
    }.into_iter()
     .filter(|(r, c)| r < &rmax && c < &cmax).collect()
}

fn print_current_path(path: &Vec<Coord>, cells: &Vec<Vec<Cell>>)
{
    for r in 0..cells.len()
    {
        for c in 0..cells[r].len()
        {
            let index = path.iter().position(|&e| e == (r, c));
            if index.is_some()
            {
                let c = match cells[r][c]
                {
                    Cell::S => 'S',
                    Cell::E => 'E',
                    Cell::L(x) => (x as u8 + ('a' as u8) - 1) as char
                };
                print!("{}", c);
            }
            else
            {
                print!(" ");
            }

            // let index = path.iter().position(|&e| e == (r, c));
            // if index.is_some()
            // {
            //     let i = index.unwrap();
            //     if i + 1 == path.len()
            //     {
            //         print!("#");
            //     }
            //     else
            //     {
            //         let cur = path[i];
            //         let next = path[i+1];
            //         let diff = (
            //             next.0 as i32 - cur.0 as i32 ,
            //             next.1 as i32 - cur.1 as i32
            //         );
            //         let c = match diff
            //         {
            //             (0, 1)  => '>',
            //             (1, 0)  => 'v',
            //             (-1, 0) => '^',
            //             (0, -1) => '<',
            //             _       => '?'
            //         };
            //         print!("{}", c);
            //     }
            // }
            // else
            // {
            //     print!(" ");
            // }
        }
        println!();
    }
    println!("Steps: {}", path.len() - 1);
}

fn dist_heuristic(test: &Coord, end: &Coord) -> f64
{
    let dx : f64 = (test.0 as f64 - end.0 as f64).abs();
    let dy : f64 = (test.1 as f64 - end.1 as f64).abs();

    return dx*dx + dy*dy;
}

fn find_lowest_score_node(open_set: &HashSet<Coord>, fscore: &HashMap<Coord, f64>) -> Option<Coord>
{
    let mut lowest_score = f64::INFINITY;
    let mut best_coord : Option<Coord> = None;
    for c in open_set
    {
        let score = *fscore.get(c).unwrap_or(&f64::INFINITY);
        if score < lowest_score
        {
            lowest_score = score;
            best_coord = Some(*c);
        }
    }
    return best_coord;
}

fn cell_to_i32(c: &Cell) -> i32
{
    return match c
    {
        Cell::S => 0,
        Cell::E => 26,
        Cell::L(x) => *x
    };
}

fn reconstruct_path(came_from: &HashMap<Coord, Coord>, mut end: Coord) -> Vec<Coord>
{
    let mut path = vec![end];
    while came_from.contains_key(&end)
    {
        end = *came_from.get(&end).unwrap();
        path.push(end);
    }
    path.reverse();
    return path;
}

fn astar(start: Coord, end: Coord, cells: &Vec<Vec<Cell>>) -> Option<Vec<Coord>>
{
    let rmax = cells.len();
    let cmax = cells[0].len();

    let mut open_set : HashSet<Coord> = HashSet::new();
    let mut came_from = HashMap::new();
    let mut gscore : HashMap<Coord, f64> = HashMap::new();
    let mut fscore : HashMap<Coord, f64> = HashMap::new();

    open_set.insert(start);

    gscore.insert(start, 0.0);
    fscore.insert(start, dist_heuristic(&start, &end));

    while open_set.len() > 0
    {
        let current = find_lowest_score_node(&open_set, &fscore).expect("Failed to find current!");
        // println!("Next: {:?}", current);
        open_set.remove(&current);
        let current_cell = cells[current.0][current.1];
        if current_cell == Cell::E
        {
            // println!("We did it!");
            return Some(reconstruct_path(&came_from, end));
        }

        let current_level = cell_to_i32(&current_cell);
        let neighbors = generate_neighbor_coords(current.0, current.1, rmax, cmax);

        // println!("Neighbors of {:?}: {:?}", &current, &neighbors);

        for neigh in neighbors
        {
            let neighbor_cell = cells[neigh.0][neigh.1];
            let next_level = cell_to_i32(&neighbor_cell);

            let mut move_score = f64::INFINITY;
            if next_level <= current_level + 1
            {
                move_score = 1.0;
            }

            let tentative_g = gscore.get(&current).unwrap_or(&f64::INFINITY) + move_score;
            if tentative_g < *gscore.get(&neigh).unwrap_or(&f64::INFINITY)
            {
                came_from.insert(neigh, current);
                gscore.insert(neigh, tentative_g);
                fscore.insert(neigh, tentative_g);
                open_set.insert(neigh);
            }
        }
    }

    return None;
}

fn find_element(cells: &Vec<Vec<Cell>>, search: &Cell) -> Option<Coord>
{
    for r in 0..cells.len()
    {
        for c in 0..cells[r].len()
        {
            if cells[r][c] == *search
            {
                return Some((r, c));
            }
        }
    }
    return None;
}

fn part_one()
{
    let lines = aoc::read_lines();

    let cells : Vec<Vec<Cell>> = lines.iter()
        .map(|line| line.chars().map(|c| char_to_cell(c)).collect()).collect();

    let sc = find_element(&cells, &Cell::S).expect("Can't find start!");
    let ec = find_element(&cells, &Cell::E).expect("Can't find end!");

    let path = astar(sc, ec, &cells).expect("No path!");

    print_current_path(&path, &cells);
}

fn find_all_starting(cells: &Vec<Vec<Cell>>) -> Vec<Coord>
{
    let mut ret = vec![];

    for r in 0..cells.len()
    {
        for c in 0..cells[r].len()
        {
            if cells[r][c] == Cell::L(1) || cells[r][c] == Cell::S
            {
                ret.push((r, c));
            }
        }
    }
    return ret;
}

fn part_two()
{
    let lines = aoc::read_lines();

    let cells : Vec<Vec<Cell>> = lines.iter()
        .map(|line| line.chars().map(|c| char_to_cell(c)).collect()).collect();

    let starts = find_all_starting(&cells);
    let ec = find_element(&cells, &Cell::E).expect("Can't find end!");

    let mut best_path = vec![];
    let mut lowest = 100000;

    for sc in starts
    {
        let path = astar(sc, ec, &cells);
        if path.is_some()
        {
            let p = path.unwrap();
            let len = p.len();
            if len < lowest
            {
                lowest = len;
                best_path = p;
            }
        }
    }

    print_current_path(&best_path, &cells);
}

fn main()
{
    part_one();
    part_two();
}