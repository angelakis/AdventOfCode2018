use std::cmp;
pub struct Point(usize, usize);

pub fn distance(p1: &Point, p2: &Point) -> usize {
    ((p1.0 as isize - p2.0 as isize).abs() +
     (p1.1 as isize - p2.1 as isize).abs()) as usize
}

pub fn main() {
    let input = include_str!("../../input/day06a.txt");
    let threshold = 10000;
    let points: Vec<Point> = input.lines()
                                       .map(|p| p.split(',')
                                                 .map(|x| x.trim().parse::<usize>().unwrap())
                                                 .collect::<Vec<usize>>())
                                       .map(|x| Point(x[0], x[1]))
                                       .collect();
    let (mut xmax, mut ymax) = (points[0].0, points[0].1);
    for p in points.iter() {
        xmax = cmp::max(p.0 + 1, xmax);
        ymax = cmp::max(p.1 + 1, ymax);
    }
    let mut safe_spaces = 0;
    for x in 0..xmax {
        for y in 0..ymax {
            let mut total_dist = 0;
            let cur_point = Point(x, y);
            for p in points.iter() {
                total_dist += distance(p, &cur_point);
            }
            if total_dist < threshold {
                safe_spaces += 1;
            }
        }
    }
    println!("Safe region has size {}", safe_spaces);
}
