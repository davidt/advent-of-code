use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let data = read_to_string("input").unwrap();
    let re = Regex::new(r"(?<op>do|don\'t|mul)\(((?P<a>\d+)\,(?P<b>\d+))?\)").unwrap();
    let mut on = true;
    let mut total = 0;

    let _: Vec<_> = re.captures_iter(&data)
        .map(|c| {
            let op = c.name("op").unwrap().as_str();
            let a = c.name("a");
            let b = c.name("b");

            match op {
                "do" => {
                    if a == None && b == None {
                        on = true;
                    }
                },
                "don't" => {
                    if a == None && b == None {
                        on = false;
                    }
                },
                "mul" => {
                    if a != None && b != None && on {
                        let ai = a.unwrap().as_str().parse::<i32>().unwrap();
                        let bi = b.unwrap().as_str().parse::<i32>().unwrap();
                        total += ai * bi;
                    }
                }
                _ => panic!(),
            }
        })
        .collect();

    println!("{}", total);
}
