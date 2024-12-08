use std::collections::HashMap;
use std::iter::zip;
use std::fs::read_to_string;


fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for line in read_to_string("input").unwrap().lines() {
        let (a, b) = line.trim().split_once("   ").unwrap();
        let ai = a.parse::<i32>().unwrap();
        let bi = b.parse::<i32>().unwrap();

        list1.push(ai);
        list2.push(bi);

        counts.entry(bi)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    list1.sort();
    list2.sort();

    let distance: i32 = zip(list1.iter(), list2.iter())
        .map(|(a, b)| (b - a).abs())
        .sum();

    println!("Distance: {}", distance);

    let similarity: i32 = list1.iter()
        .map(|x| {
            let count = counts.get(x).unwrap_or(&0);
            x * count
        })
        .sum();

    println!("Similarity: {}", similarity);
}
