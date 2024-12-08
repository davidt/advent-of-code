use itertools::Itertools;
use std::fs::read_to_string;

fn is_safe(gaps: Vec<i32>) -> bool {
    if gaps.iter().all(|&x| x > 0) || gaps.iter().all(|&x| x < 0) {
        let max = gaps.iter().map(|x| x.abs()).max().unwrap();

        return max >= 1 && max <= 3;
    } else {
        return false;
    }
}

fn skip_nth(v: &Vec<i32>, n: usize) -> Vec<i32> {
    return v
        .iter()
        .enumerate()
        .filter_map(|(i, e)| if i != n { Some(*e) } else { None })
        .collect();
}

fn main() {
    let mut safe = 0;
    let mut kinda_safe = 0;

    for line in read_to_string("input").unwrap().lines() {
        let v: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

        let mut gaps = v.iter().tuple_windows().map(|(a, b)| b - a).collect();

        if is_safe(gaps) {
            safe += 1;
        } else {
            for i in 0..v.len() {
                let new_v = skip_nth(&v, i);

                gaps = new_v.iter().tuple_windows().map(|(a, b)| b - a).collect();

                if is_safe(gaps) {
                    kinda_safe += 1;
                    break;
                }
            }
        }
    }

    println!("Safe: {}", safe);
    println!("Kinda safe: {}", safe + kinda_safe);
}
