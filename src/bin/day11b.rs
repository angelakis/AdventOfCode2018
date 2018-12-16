use std::cmp;

pub fn cell_power(serial: usize, x: usize, y: usize) -> isize {
    (((x+10) * (((x+10)*y) + serial) / 100) % 10) as isize - 5isize
}

pub fn mini_grid_power(grid: &Vec<Vec<isize>>, top_left_x: usize,
                       top_left_y: usize, square_size: usize) -> isize {
    let mut power: isize = 0;
    for x in 0..square_size {
        for y in 0..square_size {
            power += grid[top_left_x + x][top_left_y + y];
        }
    }
    power
}

pub fn main() {
    let input = include_str!("../../input/day11a.txt");
    let serial: usize = input.trim().parse().unwrap();
    let grid_size = 301;
    let mut fuel_grid: Vec<Vec<isize>> = vec!(vec!(0; grid_size); grid_size);
    for x in 1..grid_size {
        for y in 1..grid_size {
            fuel_grid[x][y] = cell_power(serial, x, y);
        }
    }
    let mut best_power = ((0, 0, 0), 0isize);
    for x in 1..grid_size - 2 {
        for y in 1..grid_size - 2 {
            for w in 1..cmp::min(grid_size - x - 1, grid_size - y - 1) {
                let cur_power = mini_grid_power(&fuel_grid, x, y, w);
                if cur_power > best_power.1 {
                    best_power = ((x, y, w), cur_power);
                }
            }
        }
    }
    println!("Best power is {:?} with {}", best_power.0, best_power.1);
}
