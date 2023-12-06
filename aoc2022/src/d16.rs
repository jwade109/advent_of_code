mod aoc;

#[derive(Debug, Clone)]
struct Valve
{
    name: String,
    rate: i32,
    dst: Vec<String>,
    open: bool
}

#[derive(Debug, Clone, Copy)]
enum Action
{
    Move(usize),
    Open
}

#[derive(Debug, Clone)]
struct SimState
{
    current: usize,
    valves: Vec<Valve>,
    score: i32,
    actions: Vec<Action>
}

fn parse_line(line: &str) -> Valve
{
    let tokens = aoc::split(&line, " ");

    let name = tokens[1];
    let rate : i32 = tokens[4][5..tokens[4].len()-1].parse().unwrap();

    let mut dst = vec![];

    for i in 9..tokens.len()
    {
        dst.push(tokens[i][0..2].to_string());
    }

    return Valve
    {
        name: name.to_string(),
        rate: rate,
        dst: dst,
        open: false
    };
}

fn index_of(valves: &Vec<Valve>, name: &str) -> usize
{
    return valves.iter().position(|v| v.name == name).unwrap();
}

fn generate_actions(sim: &SimState) -> Vec<Action>
{
    let count : i32 = sim.valves.iter().map(|v| (!v.open) as i32).sum();
    // println!("{:?} currently closed", count);
    if count == 0
    {
        return vec![];
    }

    let current = &sim.valves[sim.current];
    let mut moves : Vec<_> = current.dst.iter()
        .map(|d| Action::Move(index_of(&sim.valves, d))).collect();
    if !current.open
    {
        moves.push(Action::Open);
    }
    return moves;
}

fn execute_action(sim: &mut SimState, action: &Action)
{
    // println!("Executing action #{}: {:?} (current {})",
    //     sim.actions.len(), &action, sim.current);
    sim.score += sum_of_pressure(&sim);
    sim.actions.push(*action);
    match action
    {
        Action::Open =>
        {
            sim.valves[sim.current].open = true;
        },
        Action::Move(x) =>
        {
            sim.current = *x;
        }
    }
}

fn sum_of_pressure(sim: &SimState) -> i32
{
    let open_valves : Vec<_> = sim.valves.iter().filter(|v| v.open).collect();
    let open_names : Vec<&String> = open_valves.iter().map(|v| &v.name).collect();
    let sum : i32 = open_valves.iter().map(|v| v.rate).sum();
    println!("Valves {:?} are open, releasing {} pressure.", &open_names, sum);
    return sum;
}

fn recursive_entrypoint(sim: &SimState, minutes: usize) -> SimState
{
    if sim.actions.len() == minutes
    {
        return sim.clone();
    }

    let all_actions = generate_actions(&sim);
    let mut best_sim : Option<SimState> = None;
    let mut best_score = 0;
    for action in all_actions
    {
        let mut copy = sim.clone();
        execute_action(&mut copy, &action);
        let s = recursive_entrypoint(&copy, minutes);
        if s.score >= best_score
        {
            best_score = s.score;
            best_sim = Some(s);
        }
    }
    return best_sim.unwrap();
}

fn main()
{
    let lines = aoc::read_lines();

    let mut sim = SimState
    {
        current: 0,
        valves: lines.iter().map(|l| parse_line(l)).collect(),
        score: 0,
        actions: vec![]
    };

    let best_sim = recursive_entrypoint(&sim, 7);

    for (i, action) in best_sim.actions.iter().enumerate()
    {
        println!("== Minute {} ==", i + 1);
        match action
        {
            Action::Move(x) =>
            {
                println!("You move to valve {:?}", &best_sim.valves[*x].name);
            },
            Action::Open =>
            {
                println!("You open the current valve.");
            }
        }
        execute_action(&mut sim, &action);
    }

    println!("Final score: {}", best_sim.score);
}
