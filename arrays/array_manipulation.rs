use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'arrayManipulation' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY queries
 */

fn arrayManipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut arr: Vec<i64> = vec![0; n as usize];
    for op in queries {
        match op[..] {
            [a, b, k] => {
                arr[(a - 1) as usize] += k as i64;
                if b < n {
                    arr[b as usize] -= k as i64;
                }
            }
            _ => panic!("Unexpected operation!"),
        }
    }
    let cumsum: Vec<i64> = arr
        .into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    *cumsum.iter().max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

    for i in 0..m as usize {
        queries.push(Vec::with_capacity(3_usize));

        queries[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = arrayManipulation(n, &queries);

    writeln!(&mut fptr, "{}", result).ok();
}
