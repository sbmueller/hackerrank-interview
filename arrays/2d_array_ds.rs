use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'hourglassSum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

use std::slice::Iter;

fn row_sum(it: &mut Iter<i32>, column: usize) -> i32 {
    // This should be `advance_by` which is currently a nightly feature
    for _ in 0..column {
        it.next();
    }
    it.take(3).sum::<i32>()
}

fn hourglassSum(arr: &[Vec<i32>]) -> i32 {
    let mut max_val = std::i32::MIN;
    for c in 0..4 {
        for r in 0..4 {
            let mut acc = 0;

            // Sum first row
            acc += row_sum(&mut arr[r].iter(), c);
            // Sum second row
            acc += arr[r + 1].iter().nth(c + 1).unwrap();
            // Sum third row
            acc += row_sum(&mut arr[r + 2].iter(), c);

            max_val = std::cmp::max(max_val, acc);
        }
    }
    max_val
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(6_usize);

    for i in 0..6_usize {
        arr.push(Vec::with_capacity(6_usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = hourglassSum(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
