use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sherlockAndAnagrams' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

/// Sort a string by its characters
fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn sherlock_and_anagrams(s: &str) -> i32 {
    let mut anagrams: HashMap<String, i32> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    for l in 1..s.len() {
        chars
            .windows(l) // windows of size l over the input string
            .map(|v| v.iter().collect::<String>()) // rebuild substrings from char arrays
            .for_each(|substring| {
                *anagrams.entry(sort_string(&substring)).or_insert(0) += 1; // increase substring
                                                                            // counter
            });
    }
    anagrams.values().map(|&v| (0..v).sum::<i32>()).sum() // Sum over all anagram possibilities
                                                          // and all anagrams
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
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlock_and_anagrams(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_1() {
        assert_eq!(sherlock_and_anagrams("abba"), 4);
        assert_eq!(sherlock_and_anagrams("abcd"), 0);
    }

    #[test]
    fn sample_input_2() {
        assert_eq!(sherlock_and_anagrams("ifailuhkqq"), 3);
        assert_eq!(sherlock_and_anagrams("kkkk"), 10);
    }

    #[test]
    fn sample_input_3() {
        assert_eq!(sherlock_and_anagrams("cdcd"), 5);
    }
}
