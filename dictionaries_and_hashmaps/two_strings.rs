use std::collections::hash_set::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'twoStrings' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s1
 *  2. STRING s2
 */

fn two_strings(s1: &str, s2: &str) -> String {
    let h1 = HashSet::from(s1.chars().collect::<HashSet<char>>());
    let h2 = HashSet::from(s2.chars().collect::<HashSet<char>>());
    if h1.intersection(&h2).count() > 0 {
        return "YES".to_string();
    }
    return "NO".to_string();
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..q {
        let s1 = stdin_iterator.next().unwrap().unwrap();

        let s2 = stdin_iterator.next().unwrap().unwrap();

        let result = two_strings(&s1, &s2);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
