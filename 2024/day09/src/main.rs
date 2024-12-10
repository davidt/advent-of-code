use itertools::interleave;
use std::iter;
use std::fs::read_to_string;

#[derive(Copy, Clone, PartialEq)]
enum ElementType {
    FILE,
    GAP,
}

#[derive(Copy, Clone)]
struct Element {
    etype: ElementType,
    id: Option<usize>,
    size: usize,
}

fn print_disk(disk: &Vec<Element>) {
    let printable = disk.iter()
        .map(|el| match el.etype {
            ElementType::FILE => iter::repeat_n(format!("{}", el.id.unwrap() as u32), el.size),
            ElementType::GAP => iter::repeat_n(String::from("."), el.size),
        })
        .flatten()
        .collect::<Vec<_>>()
        .join("");

    println!("{}", printable);
}

fn to_sectors(disk: &Vec<Element>) -> Vec<Option<usize>> {
    disk.iter()
        .map(|el| match el.etype {
            ElementType::FILE => iter::repeat_n(Some(el.id.unwrap()), el.size),
            ElementType::GAP => iter::repeat_n(None, el.size),
        })
        .flatten()
        .collect()
}

fn defrag1(disk: Vec<Element>) {
    let mut sectors = to_sectors(&disk);
    let mut i = 0;
    let mut j = sectors.len() - 1;

    loop {
        if i >= j {
            break;
        }

        let block = sectors[i];
        if block.is_some() {
            i += 1;
            continue
        } else {
            sectors[i] = sectors[j];
            sectors[j] = None;

            i += 1;

            loop {
                j -= 1;

                if i >= j || sectors[j].is_some() {
                    break;
                }
            }
        }
    }

    let checksum: u64 = sectors.iter()
        .enumerate()
        .map(|(i, file)| {
            match file {
                Some(f) => Some((i as u64) * (*f as u64)),
                None => None,
            }
        })
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
        .sum();

    println!("Defrag method 1, checksum: {}", checksum);
}

fn defrag2(mut disk: Vec<Element>) {
    let disk_len = disk.len();

    for i in 0..disk_len {
        let gap = disk[i];
        if gap.etype != ElementType::GAP {
            continue;
        }

        let gap_size = gap.size;

        for j in (i..disk_len).rev() {
            let file = disk[j];
            if file.etype != ElementType::FILE {
                continue;
            }

            let file_size = file.size;

            if file_size > gap_size {
                continue;
            } else if file_size == gap_size {
                // File size matches gap size exactly, just swap.
                disk[i] = file;
                disk[j] = gap;
                break;
            } else {
                // Move the file and add an additional gap.
                disk[i] = file;
                disk.insert(i + 1, Element {
                    etype: ElementType::GAP,
                    id: None,
                    size: gap_size - file_size,
                });
                disk[j + 1] = Element {
                    etype: ElementType::GAP,
                    id: None,
                    size: file_size
                };
                break;
            }
        }
    }

    let sectors = to_sectors(&disk);

    let checksum: u64 = sectors.iter()
        .enumerate()
        .map(|(i, file)| {
            match file {
                Some(f) => Some((i as u64) * (*f as u64)),
                None => None,
            }
        })
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
        .sum();

    println!("Defrag method 2, checksum: {}", checksum);
}

fn main() {
    let disk_map: Vec<usize> = read_to_string("input")
        .unwrap()
        .trim()
        .chars()
        .map(|c| String::from(c).parse::<usize>().unwrap())
        .collect();

    let file_sizes: Vec<&usize> = disk_map.iter().step_by(2).collect();
    let gap_sizes: Vec<&usize> = disk_map.iter().skip(1).step_by(2).collect();

    let disk: Vec<Element> = interleave(
            file_sizes.iter()
                .enumerate()
                .map(|(i, size)| Element {
                    etype: ElementType::FILE,
                    id: Some(i),
                    size: **size,
                }),
            gap_sizes.iter()
                .map(|size| Element {
                    etype: ElementType::GAP,
                    id: None,
                    size: **size,
                })
        )
        .collect();

    defrag1(disk.clone());
    defrag2(disk.clone());
}
