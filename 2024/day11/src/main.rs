use std::collections::HashMap;
use std::fs::read_to_string;

fn rules(stone: &u64) -> Vec<u64> {
    if *stone == 0 {
        return vec![1];
    }

    let n_digits = stone.ilog10() + 1;

    if n_digits % 2 == 0 {
        let power = 10_u64.pow(n_digits / 2);
        let a = stone / power;
        let b = stone - (a * power);

        return vec![a, b];
    }

    return vec![stone * 2024];
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones.iter().map(|stone| rules(stone)).flatten().collect()
}

type Cache = HashMap<(u64, i32), u64>;

fn advance(stone: u64, blinks: i32, total: i32, cache: &mut Cache) -> u64 {
    if blinks == total {
        return 1;
    }

    if let Some(n) = cache.get(&(stone, blinks)) {
        return *n;
    }

    /*
     * Single digit stones will cycle back to single digits in a set number of
     * blinks. We can therefore short-circuit a lot of steps.
     */
    struct SingleDigitStep {
        blinks: i32,
        next: Vec<u64>,
    }
    let steps = [
        /* 0 */ SingleDigitStep {
            blinks: 1,
            next: vec![1],
        },
        /* 1 */
        SingleDigitStep {
            blinks: 3,
            next: vec![2, 0, 2, 4],
        },
        /* 2 */
        SingleDigitStep {
            blinks: 3,
            next: vec![4, 0, 4, 8],
        },
        /* 3 */
        SingleDigitStep {
            blinks: 3,
            next: vec![6, 0, 7, 2],
        },
        /* 4 */
        SingleDigitStep {
            blinks: 3,
            next: vec![8, 0, 9, 6],
        },
        /* 5 */
        SingleDigitStep {
            blinks: 5,
            next: vec![2, 0, 4, 8, 2, 8, 8, 0],
        },
        /* 6 */
        SingleDigitStep {
            blinks: 5,
            next: vec![2, 4, 5, 7, 9, 4, 5, 6],
        },
        /* 7 */
        SingleDigitStep {
            blinks: 5,
            next: vec![2, 8, 6, 7, 6, 0, 3, 2],
        },
        /* 8 */
        SingleDigitStep {
            blinks: 5,
            next: vec![3, 2, 7, 7, 2, 6, 16192],
        },
        /* 9 */
        SingleDigitStep {
            blinks: 5,
            next: vec![3, 6, 8, 6, 9, 1, 8, 4],
        },
    ];

    if stone < 10 {
        let step = &steps[stone as usize];
        let next_blinks = blinks + step.blinks;

        if next_blinks <= total {
            let n = step
                .next
                .iter()
                .map(|next| advance(*next, next_blinks, total, cache))
                .sum();

            cache.insert((stone, blinks), n);

            return n;
        }
    }

    let next = rules(&stone);
    let n = next
        .iter()
        .map(|s| advance(*s, blinks + 1, total, cache))
        .sum();
    cache.insert((stone, blinks), n);
    return n;
}

fn main() {
    let stones: Vec<u64> = read_to_string("input")
        .unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    /* 25 blinks is small enough that we can just simulate everything. */
    {
        let mut stones_25 = stones.clone();

        for _i in 0..25 {
            stones_25 = blink(&stones_25);
        }

        println!("{} stones after 25 blinks", stones_25.len());
    }

    /* 75 needs to be much more clever. */
    {
        let mut cache: Cache = HashMap::new();
        let result: u64 = stones
            .iter()
            .map(|stone| advance(*stone, 0, 75, &mut cache))
            .sum();

        println!("{} stones after 75 blinks", result);
    }
}
