use std::io::{self, BufRead};

/*
 * Complete the 'minimumBribes' function below.
 *
 * The function accepts INTEGER_ARRAY q as parameter.
 */

fn move_element(arr: &mut [i32], old_index: usize, new_index: usize) {
    if old_index < new_index {
        arr[old_index..=new_index].rotate_left(1);
    } else {
        arr[new_index..=old_index].rotate_right(1);
    }
}

fn minimumBribes(q: &[i32]) {
    let mut bribes = 0;
    // Working copy of input
    let mut q_work = q.to_vec();
    let mut idx = q_work.len() - 1;
    while idx < q_work.len() {
        let v = q_work[idx];
        let diff = v - (idx + 1) as i32;
        if diff > 2 {
            println!("Too chaotic");
            return;
        } else if diff > 0 {
            bribes += diff;
            move_element(&mut q_work, idx, v as usize - 1);
        } else {
            idx -= 1;
        }
    }
    println!("{bribes}");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let q: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        minimumBribes(&q);
    }
}
