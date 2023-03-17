use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'maximumToys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY prices
 *  2. INTEGER k
 */

fn maximum_toys(prices: &[i32], k: i32) -> i32 {
    let mut sorted_prices = prices.to_vec();
    sorted_prices.sort_unstable(); // Sort with O(n*log(n)) worst case
    sorted_prices
        .iter()
        .scan(0, |acc, x| {
            // Cumulative sum
            *acc += x;
            Some(*acc)
        })
        .position(|x| x > k) // Find position that exceeds budget
        .unwrap_or(0) as i32
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

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let prices: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = maximum_toys(&prices, k);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(maximum_toys(&[1, 2, 3, 4], 7), 3);
    }

    #[test]
    fn sample_0() {
        assert_eq!(maximum_toys(&[1, 12, 5, 111, 200, 1000, 10], 50), 4);
    }
}
