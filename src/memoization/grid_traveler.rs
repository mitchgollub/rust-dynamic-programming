use std::collections::HashMap;

pub fn grid_traveler_example() {
    println!("grid_traveler(1, 1): {}", grid_traveler(1, 1, &mut HashMap::new()));
    println!("grid_traveler(2, 3): {}", grid_traveler(2, 3, &mut HashMap::new()));
    println!("grid_traveler(3, 2): {}", grid_traveler(3, 2, &mut HashMap::new()));
    println!("grid_traveler(3, 3): {}", grid_traveler(3, 3, &mut HashMap::new()));
    println!("grid_traveler(10, 10): {}", grid_traveler(10, 10, &mut HashMap::new()));
    println!("grid_traveler(18, 18): {}", grid_traveler(18, 18, &mut HashMap::new()));
}

fn grid_traveler(m: usize, n: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if m == 1 && n == 1 {
        1
    }
    else if m == 0 || n == 0 {
        0
    }
    else {
        match map.get(&(m, n)) {
            Some(value) => *value,
            None => {
                let result = grid_traveler(m - 1, n, map) + grid_traveler(m, n - 1, map);
                map.insert((m, n), result);
                result
            }
        }
    }
}