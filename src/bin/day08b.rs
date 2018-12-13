use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../../input/day08a.txt");
    let input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut input_iter = input.iter();
    let mut nodes = vec!();
    let mut metadata = vec!();
    let mut node_values: HashMap<usize, usize> = HashMap::new();
    let mut node_children: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut current_node = 1;
    nodes.push((current_node, *input_iter.next().unwrap()));
    metadata.push(*input_iter.next().unwrap());
    while let Some((node, cur_children)) = nodes.pop() {
        if cur_children == 0 {
            let mut cur_value = 0;
            for _ in 0..metadata.pop().unwrap() {
                let metadatum = input_iter.next().unwrap();
                if node_children.get(&node) == None {
                    cur_value += metadatum;
                } else {
                    let my_children = node_children.get(&node).unwrap();
                    if metadatum > &my_children.len() {
                        continue;
                    }
                    cur_value += node_values.get(&my_children[metadatum - 1]).unwrap();
                }
            }
            node_values.insert(node, cur_value);
            continue;
        }
        nodes.push((node, cur_children - 1));
        current_node += 1;
        let nc = node_children.entry(node).or_insert(vec!());
        nc.push(current_node);
        nodes.push((current_node, *input_iter.next().unwrap()));
        metadata.push(*input_iter.next().unwrap());
    }
    println!("Root node value is {:?}", node_values.get(&1).unwrap());
}
