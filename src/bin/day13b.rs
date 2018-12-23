use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Clone)]
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

#[derive(Clone)]
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
    loop {
        let mut positions = HashMap::new();
        carts.sort_by(|(xy1, ..), (xy2, ..)| xy1.cmp(xy2));
        let temp_carts = carts.clone();
        for i in 0..temp_carts.len() {
            positions.insert(carts[i].0, i);
        }
        let mut to_be_removed = vec!();
        for (cart_ind, cart) in temp_carts.iter().enumerate() {
            let ((x, y), dir, next_turn) = cart;
            positions.remove(&(*x, *y));
            let (nextx, nexty) = match dir {
                Direction::Down => (*x+1, *y),
                Direction::Up => (*x-1, *y),
                Direction::Left => (*x, *y-1),
                Direction::Right => (*x, *y+1),
            };
            if let Entry::Occupied(e) = positions.entry((nextx, nexty)) {
                to_be_removed.push(cart_ind);
                to_be_removed.push(*e.get());
                e.remove_entry();
                continue;
            }
            positions.insert((nextx, nexty), cart_ind);
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
            carts[cart_ind] = ((nextx, nexty), new_dir, new_next_turn);
        }
        let mut removed_counter = 0;
        to_be_removed.sort();
        to_be_removed.dedup();
        for crashed in to_be_removed.iter() {
            carts.remove(crashed - removed_counter);
            removed_counter += 1;
        }
        if carts.len() == 1 {
            break;
        } else if carts.len() == 0 {
            panic!("No carts left");
        }
    }
    println!("Last cart will be @ {:?}", ((carts[0].0).1, (carts[0].0).0));
}
