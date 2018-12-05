use std::collections::HashMap;

extern crate aoc2018;

use aoc2018::files::read_from_file;

pub fn change_format(input: String) -> Vec<(usize, usize, String, String)> {
    let mut infos = Vec::new();
    for line in input.lines() {
        let simpler = line.replace("[", "")
                          .replace("]", "")
                          .replace("-", "")
                          .replace("#", "")
                          .replace(":", "");
        let words: Vec<&str> = simpler.split(' ').collect();
        let date: usize = words[0].parse().unwrap();
        let time: usize = words[1].parse().unwrap();
        infos.push((date, time, words[2].to_string(), words[3].to_string()));
    }
    infos.sort();
    infos
}

pub fn main() {
    let input = read_from_file("input/day04a.txt");
    let infos = change_format(input);
    let mut cur_guard = 0usize;
    let mut fell_asleep = 0;
    let mut best_freq_guard = 0;
    let mut best_freq = 0;
    let mut best_freq_minute = 0;
    let mut freq_map = HashMap::new();
    for info in infos.iter() {
        match *info {
            (_, _, ref s, ref g) if s == "Guard" => cur_guard = g.parse().unwrap(),
            (_, t, ref s, _) if s == "falls" => fell_asleep = t,
            (_, woke, ref s, _) if s == "wakes" => {
                let minute_freqs = freq_map.entry(cur_guard).or_insert(vec!(0; 60));
                for m in fell_asleep..woke {
                    minute_freqs[m] += 1;
                    if minute_freqs[m] >= best_freq {
                        best_freq_guard = cur_guard;
                        best_freq_minute = m;
                        best_freq = minute_freqs[m];
                    }
                }
            },
            _ => panic!("Something went terribly wrong"),
        }
    }
    println!("Result is = {}", best_freq_guard * best_freq_minute);
}
