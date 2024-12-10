use std::collections::HashSet;
use std::fs::read_to_string;

type Map = Vec<Vec<usize>>;
type Position = (usize, usize);

fn next_positions(w: usize, h: usize, position: &Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let (x, y) = *position;

    if y > 0 { positions.push((x, y - 1)); /* North */ }
    if x + 1 < w { positions.push((x + 1, y)); /* East */ }
    if y + 1 < h { positions.push((x, y + 1)); /* South */ }
    if x > 0 { positions.push((x - 1, y)); /* West */ }

    positions
}

fn walk(
    map: &Map,
    w: usize,
    h: usize,
    elevation: usize,
    position: &Position,
) -> (HashSet<Position>, usize) {
    let mut endpoints: HashSet<Position> = HashSet::new();
    let mut paths: usize = 0;

    for step in next_positions(w, h, position).iter() {
        let (x, y) = *step;
        let next_elevation = map[y][x];

        if elevation == 8 && next_elevation == 9 {
            endpoints.insert(*step);
            paths += 1;
        } else if elevation + 1 == next_elevation {
            let (a, b) = walk(map, w, h, next_elevation, step);
            for point in a.iter() {
                endpoints.insert(*point);
            }

            paths += b;
        }
    }

    (endpoints, paths)
}

fn main() {
    let map: Map = read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| -> Vec<usize> {
            line.chars()
                .map(|c| String::from(c).parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let h = map.len();
    let w = map[0].len();

    let mut endpoints = 0;
    let mut paths = 0;

    for y in 0..h {
        for x in 0..w {
            if map[y][x] == 0 {
                let (a, b) = walk(&map, w, h, 0, &(x, y));
                endpoints += a.len();
                paths += b;
            }
        }
    }

    println!("{} total endpoints", endpoints);
    println!("{} total unique paths", paths);
}
