use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'repeatedString' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. LONG_INTEGER n
 */

fn repeatedString(s: &str, n: i64) -> i64 {
    let as_in_str = s.chars().filter(|&x| x == 'a').count();
    let last_rep_size = n as usize % s.len();
    let as_in_last_rep = s.chars().take(last_rep_size).filter(|&x| x == 'a').count();
    as_in_str as i64 * (n / s.len() as i64) + as_in_last_rep as i64
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    let result = repeatedString(&s, n);

    writeln!(&mut fptr, "{}", result).ok();
}
