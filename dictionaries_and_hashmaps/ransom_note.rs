use std::io::{self, BufRead};

/*
 * Complete the 'checkMagazine' function below.
 *
 * The function accepts following parameters:
 *  1. STRING_ARRAY magazine
 *  2. STRING_ARRAY note
 */

fn checkMagazine(magazine: &[String], note: &[String]) {
    let mut magazin_copy = magazine.to_vec();
    for word in note {
        if let Some(index) = magazin_copy.iter().position(|value| value == word) {
            magazin_copy.remove(index);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let n = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let magazine: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let note: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    checkMagazine(&magazine, &note);
}
