use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn get_input(file_contents: &str) -> (Vec<(isize, isize)>, Vec<(isize, isize)>) {
    let data = file_contents.lines()
        .map(|x| x.replace("<", " ")
             .replace(">", " ")
             .replace(",", ""))
        .map(|x| x.split_whitespace()
             .map(|y| y.to_string())
             .collect::<Vec<String>>());
    let (mut points, mut velocities) = (vec!(), vec!());
    for x in data {
        points.push((x[1].parse().unwrap(), x[2].parse().unwrap()));
        velocities.push((x[x.len() - 2].parse().unwrap(),
                         x[x.len() - 1].parse().unwrap()));
    }
    (points, velocities)
}

pub fn are_close(p1: (isize, isize), p2: (isize, isize)) -> bool {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() <= 3
}

pub fn main() {
    let input = include_str!("../../input/day10a.txt");
    let (mut points, velocities) = get_input(input);
    let points_num = points.len();
    let mut possible_moments = HashMap::new();
    for moment in 1..15000 {
        for point in 0..points_num {
            let point_data = points[point];
            let speed = velocities[point];
            points[point] = (point_data.0 + speed.0, point_data.1 + speed.1);
        }
        let mut close_points = vec!();
        let mut points_clone = points.clone();
        points_clone.sort();
        let mut last = points_clone[0];
        for i in 1..points_num {
            if are_close(last, points_clone[i]) {
                close_points.push(points_clone[i]);
                close_points.push(last);
            }
            last = points_clone[i];
        }
        points_clone.sort_by(|&(x1, y1), &(x2, y2)|
                             match y1.cmp(&y2) {
                                 Ordering::Equal => x1.cmp(&x2),
                                 _ => y1.cmp(&y2),
                             });
        let mut last = points_clone[0];
        for i in 1..points_num {
            if are_close(last, points_clone[i]) {
                close_points.push(points_clone[i]);
                close_points.push(last);
            }
            last = points_clone[i];
        }
        if possible_moments.get(&close_points.len()) == None {
            possible_moments.insert(close_points.len(), (close_points, moment));
        }
    }
    for (arr, moment) in possible_moments.values() {
        if arr.len() < points.len() / 2  {
            continue;
        }
        let mut arr = arr.clone();
        let arrangement: Vec<(isize, isize)> = arr.iter().map(|&(x, y)| (y, x)).collect();
        let mut minx = arrangement[0].0;
        let mut maxx = arrangement[0].0;
        let mut miny = arrangement[0].1;
        let mut maxy = arrangement[0].1;
        let mut points = HashMap::new();
        for p in arrangement.iter() {
            minx = cmp::min(minx, p.0);
            maxx = cmp::max(maxx, p.0);
            miny = cmp::min(miny, p.1);
            maxy = cmp::max(maxy, p.1);
            points.insert(*p, true);
        }
        if maxy - miny > 500 {
            continue;
        }
        if maxx - minx > 20 {
            continue;
        }
        println!("Moment: {} - A possible message:", moment);
        for x in minx..maxx + 1 {
            for y in miny..maxy + 1 {
                if points.get(&(x, y)) == None {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}
