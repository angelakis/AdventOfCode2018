use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    marble: usize,
    previous: usize,
    next: usize,
}

impl Node {
    pub fn blank() -> Node {
        Node {marble: 0, previous: 0, next: 0}
    }
    pub fn new(marble: usize, previous: usize, next: usize) -> Node {
        Node {marble: marble, previous: previous, next: next}
    }
}

pub fn insert_after(nodes: &mut HashMap<usize, Node>, spot: usize, marble: usize) {
    let before = nodes.get(&spot).unwrap().clone();
    let after = nodes.get(&before.next).unwrap().clone();
    nodes.insert(marble, Node::new(marble, before.marble, after.marble));
    nodes.insert(before.marble, Node::new(before.marble, before.previous, marble));
    nodes.insert(after.marble, Node::new(after.marble, marble, after.next));
}

pub fn remove_at(nodes: &mut HashMap<usize, Node>, spot: usize) {
    let node = nodes.get(&spot).unwrap().clone();
    let previous = nodes.get(&node.previous).unwrap().clone();
    let next = nodes.get(&node.next).unwrap().clone();
    nodes.remove(&spot);
    nodes.insert(previous.marble, Node::new(previous.marble, previous.previous, node.next));
    nodes.insert(next.marble, Node::new(next.marble, node.previous, next.next));
}

pub fn main() {
    let input = include_str!("../../input/day09a.txt");
    let input: Vec<&str> = input
        .trim()
        .split(' ')
        .collect();
    let (player_num, marbles_num): (usize, usize) =
        (input[0].parse().unwrap(), input[6].parse().unwrap());
    let mut nodes: HashMap<usize, Node> = HashMap::new();
    nodes.insert(0, Node::new(0, 1, 1));
    nodes.insert(1, Node::new(1, 0, 0));
    let mut scores = vec!(0; player_num);
    let mut designated = 1;
    for marble in 2..100 * marbles_num {
        if marble % 23 != 0 {
            let designated_node = nodes.get(&designated).unwrap().clone();
            let spot = designated_node.next;
            insert_after(&mut nodes, spot, marble);
            designated = marble;
        } else {
            let mut to_remove = designated;
            for _ in 0..7 {
                let current = nodes.get(&to_remove).unwrap();
                to_remove = current.previous;
            }
            let node_to_remove = nodes.get(&to_remove).unwrap().clone();
            remove_at(&mut nodes, to_remove);
            let cur_player = marble % player_num;
            scores[cur_player] += marble + to_remove;
            designated = node_to_remove.next;
        }
    }
    println!("Winner scored {}", scores.iter().max().unwrap());
}
