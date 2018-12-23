pub fn main() {
    let mut input = include_str!("../../input/day14a.txt").to_string();
    input.pop();
    let mut recipes = vec!(3, 7);
    let mut first = 0;
    let mut second = 1;
    let mut recipes_len = 2;
    let needed_recipes;
    loop {
        let next_recipe = recipes[first] + recipes[second];
        if next_recipe < 10 {
            recipes.push(next_recipe);
            recipes_len += 1;
        } else {
            recipes.push(1);
            recipes.push(next_recipe - 10);
            recipes_len += 2;
        }
        first = (first + 1 + recipes[first]) % recipes_len;
        second = (second + 1 + recipes[second]) % recipes_len; 
        if recipes_len <= input.len() {
            continue
        }
        let score1 = recipes[recipes_len - input.len()..]
             .into_iter().map(|i| i.to_string()).collect::<String>();
        let score2 = recipes[recipes_len - 1 - input.len()..recipes_len - 1]
             .into_iter().map(|i| i.to_string()).collect::<String>();
        if score1 == input {
            needed_recipes = recipes_len - input.len();
            break;
        } else if score2 == input {
            needed_recipes = recipes_len - 1 - input.len();
            break;
        }
    }
    println!("The pattern is matched after {} recipes", needed_recipes); 
}
