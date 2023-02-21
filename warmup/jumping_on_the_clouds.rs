use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'jumpingOnClouds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY c as parameter.
 */

fn jumpingOnClouds(c: &[i32]) -> i32 {
    let mut jumps = 0;
    let mut index = 0;
    loop {
        if (index + 2) < c.len() && c[index + 2] == 0 {
            index += 2;
            jumps += 1;
        } else {
            index += 1;
            jumps += 1;
        }
        if index == c.len() - 1 {
            break;
        }
    }

    jumps
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let c: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = jumpingOnClouds(&c);

    writeln!(&mut fptr, "{}", result).ok();
}
