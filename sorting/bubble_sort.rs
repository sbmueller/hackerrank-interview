use std::io::{self, BufRead};

/*
 * Complete the 'countSwaps' function below.
 *
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn business_logic(a: &[i32]) -> (usize, i32, i32) {
    let mut swaps = 0;
    let mut a_vec = a.to_vec();
    for _ in 0..a_vec.len() {
        for j in 0..a_vec.len() - 1 {
            if a_vec[j] > a_vec[j + 1] {
                a_vec.swap(j, j + 1);
                swaps += 1;
            }
        }
    }
    (swaps, *a_vec.first().unwrap(), *a_vec.last().unwrap())
}

fn count_swaps(a: &[i32]) {
    let (swaps, min, max) = business_logic(a);
    println!("Array is sorted in {swaps} swaps.");
    println!("First Element: {min}");
    println!("Last Element: {max}");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    count_swaps(&a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_case_0() {
        assert_eq!(business_logic(&[1, 2, 3]), (0, 1, 3));
    }

    #[test]
    fn sample_test_case_1() {
        assert_eq!(business_logic(&[3, 2, 1]), (3, 1, 3));
    }

    #[test]
    fn sample_test_case_2() {
        assert_eq!(business_logic(&[4, 2, 3, 1]), (5, 1, 4));
    }
}
