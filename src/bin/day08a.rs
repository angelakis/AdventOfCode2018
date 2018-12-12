pub fn main() {
    let input = include_str!("../../input/day08a.txt");
    let input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut metadata_sum = 0;
    let mut input_iter = input.iter();
    let root_children = input_iter.next().unwrap();
    let root_metadata = input_iter.next().unwrap();
    let mut children = vec!(*root_children);
    let mut metadata = vec!(*root_metadata);
    loop {
        while let Some(cur_children) = children.pop() {
            if cur_children == 0 {
                break;
            }
            children.push(cur_children - 1);
            let child_children = input_iter.next().unwrap();
            let child_metadata = input_iter.next().unwrap();
            children.push(*child_children);
            metadata.push(*child_metadata);
        }
        for _ in 0..metadata.pop().unwrap() {
            let lala = input_iter.next().unwrap();
            metadata_sum += lala;
        }
        if children.is_empty() && metadata.is_empty() {
            break;
        }
    }
    println!("Metadata sum is {}", metadata_sum);
}
