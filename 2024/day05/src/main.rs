use itertools::enumerate;
use std::collections::HashMap;
use std::fs::read_to_string;

fn is_valid(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, page) in enumerate(update.iter()) {
        let Some(dependencies) = rules.get(page) else {
            continue;
        };

        for dependency in dependencies {
            let Some(j) = update.iter().position(|x| x == dependency) else {
                continue;
            };

            if j > i {
                return false;
            }
        }
    }

    return true;
}

fn fix_invalid(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut c = update.clone();

    loop {
        'pages: for (i, page) in enumerate(c.clone().iter()) {
            let Some(dependencies) = rules.get(page) else {
                continue;
            };

            for dependency in dependencies {
                let Some(j) = c.iter().position(|x| x == dependency) else {
                    continue;
                };

                if j > i {
                    let s = c.as_mut_slice();
                    s[i] = *dependency;
                    s[j] = page.clone();

                    break 'pages;
                }
            }
        }

        if is_valid(&c, rules) {
            return c;
        }
    }
}

fn main() {
    let mut rules = HashMap::<i32, Vec<i32>>::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut phase1 = true;

    for line in read_to_string("input").unwrap().lines() {
        if line.is_empty() {
            phase1 = false;
            continue;
        }

        if phase1 {
            let Some((a, b)) = line.split_once('|') else {
                panic!()
            };

            let ai = a.parse::<i32>().unwrap();
            let bi = b.parse::<i32>().unwrap();

            rules
                .entry(bi)
                .and_modify(|x| x.push(ai))
                .or_insert(vec![ai]);
        } else {
            updates.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
        }
    }

    let mut total_valid = 0;
    let mut total_invalid = 0;
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        if is_valid(&update, &rules) {
            let middle = update.get((update.len() - 1) / 2).unwrap();
            total_valid += middle;
        } else {
            invalid_updates.push(update.clone())
        }
    }

    for update in invalid_updates.iter() {
        let fixed = fix_invalid(update, &rules);
        let middle = fixed.get((fixed.len() - 1) / 2).unwrap();
        total_invalid += middle;
    }

    println!("Valid middles: {}", total_valid);
    println!("Invalid middles: {}", total_invalid);
}
