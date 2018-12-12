pub fn main() {
    let input = include_str!("../../input/day08a.txt.simple");
    let input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut metadata_sum = 0;
    let mut input_iter = input.iter();
    let root_children = input_iter.next().unwrap();
    let root_metadata = input_iter.next().unwrap();
    let mut children = vec!(0; *root_children);
    let mut metadata = vec!(root_metadata);
    println!("Node has {} children and {} metadata", root_children, root_metadata);
    loop {
        println!("Metadata sum is {}", metadata_sum);
        while let Some(_) = children.pop() {
            let cur_children = input_iter.next().unwrap();
            let cur_metadata = input_iter.next().unwrap();
            metadata.push(cur_metadata);
            children.extend(vec!(0; *cur_children));
            println!("Node has {} children and {} metadata", cur_children, cur_metadata);
            if *cur_children == 0 {
                break;
            }
        }
        for _ in 0..*metadata.pop().unwrap() {
            let lala = input_iter.next().unwrap();
            println!("Adding metadata {}", lala);
            metadata_sum += lala;
        }
        if children.is_empty() && metadata.is_empty() {
            break;
        }
    }
    println!("Metadata sum is {}", metadata_sum);
}
