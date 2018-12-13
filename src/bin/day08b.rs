use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../../input/day08a.txt");
    let input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut cur_node = 1;
    let mut input_iter = input.iter();
    let root_children = input_iter.next().unwrap();
    let root_metadata = input_iter.next().unwrap();
    let mut children = vec!((cur_node, *root_children, *root_children));
    let mut metadata = vec!(*root_metadata);
    let mut last_parent = 1;
    let mut children_values: HashMap<usize, Vec<usize>> = HashMap::new();
    loop {
        let mut has_value = true;
        while let Some((node, orig_children, cur_children)) = children.pop() {
            if cur_children == 0 {
                if orig_children != 0 {
                    has_value = false;
                }
                break;
            }
            children.push((node, orig_children, cur_children - 1));
            last_parent = node;
            let child_children = input_iter.next().unwrap();
            let child_metadata = input_iter.next().unwrap();
            cur_node += 1;
            children.push((cur_node, *child_children, *child_children));
            metadata.push(*child_metadata);
        }
        if children.is_empty() {
            last_parent = 1;
        }
        let mut cur_value = 0;
        for _ in 0..metadata.pop().unwrap() {
            let metadatum = input_iter.next().unwrap();
            if has_value {
                cur_value += metadatum;
            } else {
                println!("Requesting a value for {}", last_parent);
                let my_cv = children_values.get(&last_parent).unwrap();
                if metadatum > &my_cv.len() {
                    continue;
                }
                cur_value += my_cv[metadatum - 1];
            }
        }
        println!("Pushing a value for {}", last_parent);
        let cv = children_values.entry(last_parent).or_insert(vec!());
        cv.push(cur_value);
        if children.is_empty() && metadata.is_empty() {
            break;
        }
    }
    println!("Root node value is {:?}", children_values.get(&1).unwrap().last().unwrap());
}
