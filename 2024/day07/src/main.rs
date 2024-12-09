use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    result: i64,
    values: Vec<i64>,
}

fn compute1(result: i64, acc: i64, remaining: &[i64]) -> bool {
    let next = remaining[0];
    let a = acc + next;
    let b = acc * next;

    if remaining.len() == 1 {
        return (a == result) || (b == result);
    } else {
        return compute1(result, a, &remaining[1..]) || compute1(result, b, &remaining[1..]);
    }
}

fn compute2(result: i64, acc: i64, remaining: &[i64]) -> bool {
    let next = remaining[0];
    let a = acc + next;
    let b = acc * next;
    let c = format!("{}{}", acc, next).parse::<i64>().unwrap();

    if remaining.len() == 1 {
        return (a == result) || (b == result) || (c == result);
    } else {
        return compute2(result, a, &remaining[1..])
            || compute2(result, b, &remaining[1..])
            || compute2(result, c, &remaining[1..]);
    }
}

fn main() {
    let mut equations: Vec<Equation> = Vec::new();

    for line in read_to_string("input").unwrap().lines() {
        let (result_str, values_str) = line.split_once(":").unwrap();

        equations.push(Equation {
            result: result_str.parse::<i64>().unwrap(),
            values: values_str
                .trim()
                .split(" ")
                .map(|value| value.parse::<i64>().unwrap())
                .collect(),
        });
    }

    let mut phase1 = 0;
    let mut phase2 = 0;

    for equation in equations.iter() {
        if compute1(equation.result, equation.values[0], &equation.values[1..]) {
            phase1 += equation.result;
        }

        if compute2(equation.result, equation.values[0], &equation.values[1..]) {
            phase2 += equation.result;
        }
    }

    println!("{}", phase1);
    println!("{}", phase2);
}
