pub fn main() {
    let input = include_str!("../../input/day14a.txt");
    let goal: usize = input.trim().parse().unwrap();
    let mut recipes = vec!(3, 7);
    let mut first = 0;
    let mut second = 1;
    while recipes.len() < goal + 10 {
        let next_recipe = recipes[first] + recipes[second];
        if next_recipe < 10 {
            recipes.push(next_recipe);
        } else {
            recipes.push(1);
            recipes.push(next_recipe - 10);
        }
        first = (first + 1 + recipes[first]) % recipes.len(); 
        second = (second + 1 + recipes[second]) % recipes.len(); 
    }
    println!("The next ten recipes are = {}", 
             &recipes[goal..goal+10]
             .into_iter().map(|i| i.to_string()).collect::<String>());
}
