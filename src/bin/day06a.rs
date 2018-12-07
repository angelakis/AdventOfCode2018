use std::cmp;
use std::collections::HashMap;

pub struct Point(usize, usize);

pub fn distance(p1: &Point, p2: &Point) -> usize {
    ((p1.0 as isize - p2.0 as isize).abs() +
     (p1.1 as isize - p2.1 as isize).abs()) as usize
}

pub fn main() {
    let input = include_str!("../../input/day06a.txt");
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
    let mut counts = HashMap::new();
    let mut infinite = Vec::new();
    for x in 0..xmax {
        for y in 0..ymax {
            let (mut closest, mut min_dist, mut tie) = (0, xmax + ymax, false);
            let cur_point = Point(x, y);
            for (i, p) in points.iter().enumerate() {
                if distance(p, &cur_point) < min_dist {
                    closest = i;
                    tie = false;
                    min_dist = distance(p, &cur_point);
                } else if distance(p, &cur_point) == min_dist {
                    tie = true;
                }
            }
            if tie {
                continue;
            }
            let count = counts.entry(closest).or_insert(0);
            *count += 1;
            if x == 0 || y == 0 || x == xmax - 1 || y == ymax - 1 {
                infinite.push(closest);
            }
        }
    }
    for p in infinite {
        counts.remove(&p);
    }
    let mut largest_area = 0;
    for count in counts {
        largest_area = cmp::max(largest_area, count.1);
    }
    println!("Largest area is = {}", largest_area);
}
