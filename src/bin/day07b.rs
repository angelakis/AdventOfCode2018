use std::collections::HashMap;

pub fn get_duration(step: char) -> usize {
    61 + step as usize - 'A' as usize
}

pub fn main() {
    let input = include_str!("../../input/day07a.txt");
    let nodes: Vec<(char, char)> = input.lines()
                                         .map(|p| p.split(' ')
                                                   .collect::<Vec<&str>>())
                                         .map(|x| (x[1].chars().next().unwrap(),
                                                   x[7].chars().next().unwrap()))
                                         .collect();
    let mut done = HashMap::new();
    let mut to_be_done = HashMap::new();
    let mut possible_next = HashMap::new();
    for &(parent, node) in nodes.iter() {
        done.insert(parent, false);
        done.insert(node, false);
        to_be_done.insert(parent, false);
        to_be_done.insert(node, false);
    }
    let node_num = done.len();
    let mut cur_sec = 0;
    let mut workers = vec!((0, '}', false); 5);
    for _ in 0..node_num {
        workers.sort_by(|&(x, _, _), &(y, _, _)| x.cmp(&y));
        for worker in workers.iter_mut() {
            if worker.2 && worker.0 <= cur_sec {
                done.insert(worker.1, true);
                worker.2 = false;
            }
        }
        for &(parent, node) in nodes.iter() {
            let node_done = done.get(&node).unwrap();
            if *node_done {
                continue;
            }
            let parent_done = done.get(&parent).unwrap();
            if *parent_done {
                possible_next.entry(node).or_insert(true);
                possible_next.insert(parent, false);
            } else {
                possible_next.insert(node, false);
                possible_next.entry(parent).or_insert(true);
            }
        }
        let mut possibles = vec!();
        for (&node, &possible) in possible_next.iter() {
            if possible && !to_be_done.get(&node).unwrap() {
                possibles.push(node);
            }
        }
        possibles.sort();
        let mut possibles_iter = possibles.iter();
        let mut next_sec = usize::max_value();
        for worker in workers.iter_mut() {
            if worker.2 {
                continue;
            }
            if let Some(step) = possibles_iter.next() {
                worker.0 = cur_sec + get_duration(*step);
                worker.1 = *step;
                worker.2 = true;
                to_be_done.insert(worker.1, true);
            } else {
                break;
            }
        }
        for worker in workers.iter() {
            if worker.2 && worker.0 < next_sec {
                next_sec = worker.0;
            }
        }
        cur_sec = next_sec;
        possible_next.clear();
    }
    println!("Last one finished at {}", cur_sec);
}
