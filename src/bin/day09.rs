pub fn main() {
    let input = include_str!("../../input/day09a.txt");
    let input: Vec<&str> = input
        .trim()
        .split(' ')
        .collect();
    let (player_num, marbles_num) = (input[0].parse().unwrap(),
                                     input[6].parse().unwrap());
    let mut field = vec!(0, 4, 2, 1, 3);
    let mut scores = vec!(0; player_num);
    let mut designated = 1;
    for marble in 5..marbles_num {
        println!("{:?}", field);
        if marble % 23 != 0 {
            let new_position = designated + 2;
            if new_position == marble {
                field.push(marble);
            } else if new_position == marble + 1 {
                field.insert(1, marble);
            } else {
                field.insert(new_position, marble);
            }
            designated = new_position;
        } else {
        }
    }
    println!("Winner scored {}", scores.iter().max().unwrap());
}
