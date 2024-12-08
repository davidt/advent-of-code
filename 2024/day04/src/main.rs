use std::fs::read_to_string;

fn get_char_at(data: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if x < 0 || y < 0 {
        return '.';
    }

    match data.get(y as usize) {
        Some(line) => match line.get(x as usize) {
            Some(c) => *c,
            None => '.',
        },
        None => '.',
    }
}

fn check_xmas(data: &Vec<Vec<char>>, x: i32, y: i32, xd: i32, yd: i32) -> i32 {
    let mut chars: Vec<char> = Vec::new();

    for i in 0..4 {
        chars.push(get_char_at(data, x + xd * i, y + yd * i));
    }

    let s: String = chars.into_iter().collect();

    if s == "XMAS" {
        1
    } else {
        0
    }
}

fn check_xmas_space(data: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut total = 0;

    total += check_xmas(data, x, y, -1, -1);
    total += check_xmas(data, x, y, 0, -1);
    total += check_xmas(data, x, y, 1, -1);
    total += check_xmas(data, x, y, -1, 0);
    total += check_xmas(data, x, y, 1, 0);
    total += check_xmas(data, x, y, -1, 1);
    total += check_xmas(data, x, y, 0, 1);
    total += check_xmas(data, x, y, 1, 1);

    total
}

fn check_x_mas_space(data: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let c1 = get_char_at(data, x - 1, y - 1);
    let c2 = get_char_at(data, x + 1, y - 1);
    let c3 = get_char_at(data, x, y);
    let c4 = get_char_at(data, x - 1, y + 1);
    let c5 = get_char_at(data, x + 1, y + 1);

    let d1: [char; 3] = [c1, c3, c5];
    let d2: [char; 3] = [c2, c3, c4];

    let s1: String = d1.iter().collect();
    let s2: String = d2.iter().collect();

    if (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM") {
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in read_to_string("input").unwrap().lines() {
        data.push(line.chars().collect());
    }

    let h = data.len();
    let w = data[0].len();

    let mut total_xmas = 0;

    for y in 0..h {
        for x in 0..w {
            total_xmas += check_xmas_space(&data, x as i32, y as i32);
        }
    }

    println!("XMAS: {}", total_xmas);

    let mut total_x_mas = 0;

    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            total_x_mas += check_x_mas_space(&data, x as i32, y as i32);
        }
    }

    println!("X-MAS: {}", total_x_mas);
}
