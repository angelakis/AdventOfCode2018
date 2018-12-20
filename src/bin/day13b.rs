use std::collections::HashMap;

#[derive(Debug, Clone)]
enum NextTurn {
    Left,
    Straight,
    Right,
}

impl NextTurn {
    fn new() -> NextTurn {
        NextTurn::Left
    }

    fn next(&self) -> NextTurn {
        match &self {
            NextTurn::Left => NextTurn::Straight,
            NextTurn::Straight => NextTurn::Right,
            NextTurn::Right => NextTurn::Left,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn new(symbol: char) -> Option<Direction> {
        match symbol {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }

    fn turn(&self, next_turn: NextTurn) -> Direction {
        match next_turn {
            NextTurn::Straight => match &self {
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
            }
            NextTurn::Left => match &self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            NextTurn::Right => match &self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        }
    }
}

pub fn main() {
    let input = include_str!("../../input/day13a.txt");
    let rail_map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let (xmax, ymax) = (rail_map.len(), rail_map[0].len());
    let mut carts = vec!();
    for x in 0..xmax {
        for y in 0..ymax {
            if let Some(dir) = Direction::new(rail_map[x][y]) {
                carts.push(((x, y), dir, NextTurn::new()));
            }
        }
    }
    for _ in 0..100 {
        let temp_carts = carts.clone();
        let mut new_positions = HashMap::new();
        let mut to_be_removed = vec!();
        for (cart_ind, cart) in temp_carts.iter().enumerate() {
            let ((x, y), dir, next_turn) = cart;
            let (nextx, nexty) = match dir {
                Direction::Down => (*x+1, *y),
                Direction::Up => (*x-1, *y),
                Direction::Left => (*x, *y-1),
                Direction::Right => (*x, *y+1),
            };
            if let Some(crashed) = new_positions.get(&(nextx, nexty)) {
                println!("carts len {} Crashed {:?} {:?}", carts.len(), cart_ind, *crashed);
                to_be_removed.push(cart_ind);
                to_be_removed.push(*crashed);
                continue;
            }
            new_positions.insert((nextx, nexty), cart_ind);
            let mut new_next_turn = next_turn.clone();
            let new_dir = match (rail_map[nextx][nexty], dir) {
                ('\\', Direction::Right) => Direction::Down,
                ('\\', Direction::Down) => Direction::Right,
                ('\\', Direction::Left) => Direction::Up,
                ('\\', Direction::Up) => Direction::Left,
                ('/', Direction::Left) => Direction::Down,
                ('/', Direction::Down) => Direction::Left,
                ('/', Direction::Up) => Direction::Right,
                ('/', Direction::Right) => Direction::Up,
                ('+', dir) => {
                    new_next_turn = next_turn.next();
                    dir.turn(next_turn.clone())
                }
                (_, Direction::Up) => Direction::Up,
                (_, Direction::Left) => Direction::Left,
                (_, Direction::Right) => Direction::Right,
                (_, Direction::Down) => Direction::Down,
            };
            println!("Was at {:?} with dir {:?}, found {}, went at {:?}, with dir{:?}\nAlso previous {:?}, now {:?}", (*x, *y), dir, rail_map[nextx][nexty], (nextx, nexty), new_dir, next_turn, new_next_turn);
            carts[cart_ind] = ((nextx, nexty), new_dir, new_next_turn);
        }
        if to_be_removed.len() > 1 {
        println!("carts {:?}, to_be_removed {:?}", carts, to_be_removed);
        };
        let mut removed_counter = 0;
        to_be_removed.sort();
        to_be_removed.dedup();
        if to_be_removed.len() > 1 {
        println!("to_be_removed {:?}", to_be_removed);
        }
        for crashed in to_be_removed.iter() {
            carts.remove(crashed - removed_counter);
            removed_counter += 1;
        }
        if to_be_removed.len() > 1 {
        println!("carts {:?}", carts);
        }
        if carts.len() == 1 {
            break;
        }
    }
    println!("Last cart will be @ {:?}", ((carts[0].0).1, (carts[0].0).0));
}
