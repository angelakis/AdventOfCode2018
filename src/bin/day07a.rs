use std::collections::HashMap;
pub fn main() {
    let input = include_str!("../../input/day07a.txt");
    let nodes: Vec<(char, char)> = input.lines()
                                         .map(|p| p.split(' ')
                                                   .collect::<Vec<&str>>())
                                         .map(|x| (x[1].chars().next().unwrap(),
                                                   x[7].chars().next().unwrap()))
                                         .collect();
    let mut visited = HashMap::new();
    let mut possible_next = HashMap::new();
    let mut order = vec!();
    for &(parent, node) in nodes.iter() {
        visited.insert(parent, false);
        visited.insert(node, false);
    }
    let node_num = visited.len();
    while order.len() != node_num {
        for &(parent, node) in nodes.iter() {
            let node_visited = visited.get(&node).unwrap();
            if *node_visited {
                continue;
            }
            let parent_visited = visited.get(&parent).unwrap();
            if *parent_visited {
                possible_next.entry(node).or_insert(true);
                possible_next.insert(parent, false);
            } else {
                possible_next.insert(node, false);
                possible_next.entry(parent).or_insert(true);
            }
        }
        let mut cur = '{';
        for (&node, &possible) in possible_next.iter() {
            if possible && node < cur {
                cur = node;
            }
        }
        order.push(cur);
        visited.insert(cur, true);
        possible_next.clear();
    }
    println!("Order is = {}", order.iter().collect::<String>());
}
