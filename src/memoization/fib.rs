use std::collections::HashMap;

pub fn fib_example() {
    let n = 50;
    println!("Fib {}: {}", n, fib(n, &mut HashMap::new()));
}

fn fib(n: usize, map: &mut HashMap<usize, usize>) -> usize {
    if n <= 2 {
        return 1
    }

    // // Standard Recursion Answer
    // fib(n - 1, None) + fib(n - 2, None)

    // "Memoization"
    match map.get(&n) {
        Some(value) => *value,
        None => {
            let calc = fib(n - 1, map) + fib(n - 2, map);
            map.insert(n, calc);
            calc
        }
    }
}