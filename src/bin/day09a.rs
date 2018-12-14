pub fn main() {
    let input = include_str!("../../input/day09a.txt");
    let input: Vec<&str> = input
        .trim()
        .split(' ')
        .collect();
    let (player_num, marbles_num): (usize, usize) = 
        (input[0].parse().unwrap(), input[6].parse().unwrap());
    let mut field = vec!(0, 4, 2, 1, 3);
    let mut field_len = field.len();
    let mut scores = vec!(0; player_num);
    let mut designated = 1;
    for marble in 5..marbles_num {
        if marble % 23 != 0 {
            let new_position = designated + 2;
            if new_position == field_len {
                field.push(marble);
                designated = field_len;
            } else if new_position == field_len + 1 {
                field.insert(1, marble);
                designated = 1;
            } else {
                field.insert(new_position, marble);
                designated = new_position;
            }
            field_len += 1;
        } else {
            let cur_player = marble % player_num;
            let to_remove = {
                if designated >= 7 {
                    designated - 7
                } else {
                    field_len + designated - 7
                }
            };
            scores[cur_player] += marble + field[to_remove];
            field.remove(to_remove);
            designated = to_remove;
            field_len -= 1;
        }
        if marble % (marbles_num) == 0 {
            println!("{}% done", marble / (marbles_num));
        }
    }
    println!("Winner scored {}", scores.iter().max().unwrap());
}
