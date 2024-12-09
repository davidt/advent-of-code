use itertools::enumerate;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn in_bounds(position: (i32, i32), w: i32, h: i32) -> bool {
    let (x, y) = position;

    x >= 0 && x < w && y >= 0 && y < h
}

fn phase1(antennae: &HashMap<char, Vec<(i32, i32)>>, w: i32, h: i32) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in antennae.iter() {
        for pair in positions.iter().permutations(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            let dx = x2 - x1;
            let dy = y2 - y1;

            let p1 = (x1 - dx, y1 - dy);
            let p2 = (x2 + dx, y2 + dy);

            if in_bounds(p1, w, h) {
                antinodes.insert(p1);
            }

            if in_bounds(p2, w, h) {
                antinodes.insert(p2);
            }
        }
    }

    println!("Phase 1: {} unique antinode positions", antinodes.len());
}

fn phase2(antennae: &HashMap<char, Vec<(i32, i32)>>, w: i32, h: i32) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in antennae.iter() {
        if positions.len() <= 1 {
            continue;
        }

        for pair in positions.iter().permutations(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            let dx = x2 - x1;
            let dy = y2 - y1;

            let mut i = 0;

            loop {
                let p = (x1 - (dx * i), y1 - (dy * i));

                if in_bounds(p, w, h) {
                    antinodes.insert(p);
                    i += 1;
                } else {
                    break;
                }
            }

            i = 0;

            loop {
                let p = (x2 + (dx * i), y2 + (dy * i));

                if in_bounds(p, w, h) {
                    antinodes.insert(p);
                    i += 1;
                } else {
                    break;
                }
            }
        }
    }

    println!("Phase 2: {} unique antinode positions", antinodes.len());
}

fn main() {
    let board: Vec<Vec<char>> = read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| -> Vec<char> { line.chars().collect() })
        .collect();

    let w = board[0].len() as i32;
    let h = board.len() as i32;

    let mut antennae: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (x, line) in enumerate(board.iter()) {
        for (y, c) in enumerate(line.iter()) {
            if *c != '.' {
                antennae
                    .entry(*c)
                    .and_modify(|v| v.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        }
    }

    phase1(&antennae, w, h);
    phase2(&antennae, w, h);
}
