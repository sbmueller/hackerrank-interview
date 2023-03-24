use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'activityNotifications' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY expenditure
 *  2. INTEGER d
 */

/// Assumes a sorted input
fn median(numbers: &[i32]) -> f32 {
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        return (numbers[mid - 1] + numbers[mid]) as f32 * 0.5;
    } else {
        return numbers[mid] as f32;
    }
}

fn activity_notifications(expenditure: &[i32], d: i32) -> i32 {
    let mut notifications = 0;
    let d_u = d as usize;
    let mut trailing = expenditure[0..d_u].to_vec();
    trailing.sort_unstable(); // sort the first chunk of the past days
    for i in d_u..expenditure.len() {
        let val = expenditure[i];
        if 2.0 * median(&trailing) <= val as f32 {
            notifications += 1;
        }
        // Remove oldest spending from trailing days
        trailing.remove(trailing.binary_search(&expenditure[i - d_u]).unwrap());
        // Add today's spending to trailing days
        trailing.insert(
            trailing
                .binary_search(&expenditure[i])
                .unwrap_or_else(|e| e),
            expenditure[i],
        );
    }
    notifications
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

    let d = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let expenditure: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = activity_notifications(&expenditure, d);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_0() {
        assert_eq!(activity_notifications(&[2, 3, 4, 2, 3, 6, 8, 4, 5], 5), 2);
    }

    #[test]
    fn sample_1() {
        assert_eq!(activity_notifications(&[1, 2, 3, 4, 4], 4), 0);
    }

    #[test]
    fn sample_2() {
        assert_eq!(activity_notifications(&[10, 20, 30, 40, 50], 3), 1);
    }
}
