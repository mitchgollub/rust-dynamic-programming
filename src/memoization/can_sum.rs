use std::collections::HashMap;

pub fn can_sum_example() {
    println!("can_sum(7, &vec![2, 3]): {}", can_sum(7, &vec![2, 3], &mut HashMap::new()));
    println!("can_sum(7, &vec![5, 3, 4, 7]): {}", can_sum(7, &vec![5, 3, 4, 7], &mut HashMap::new()));
    println!("can_sum(5, &vec![2, 4]): {}", can_sum(5, &vec![2, 4], &mut HashMap::new()));
    println!("can_sum(8, &vec![2, 3, 5]): {}", can_sum(8, &vec![2, 3, 5], &mut HashMap::new()));
    println!("can_sum(300, &vec![7, 14]): {}", can_sum(300, &vec![7, 14], &mut HashMap::new()));
}

fn can_sum(target_sum: i32, numbers: &Vec<i32>, map: &mut HashMap<i32, bool>) -> bool {
    if target_sum == 0 {
        true
    }
    else if target_sum < 0 {
        false
    }
    else {
        match map.get(&target_sum) {
            Some(value) => *value,
            None => {
                let mut result = false;
                for number in numbers {
                    result = result || can_sum(target_sum - number, &numbers, map)
                }
                map.insert(target_sum, result);
                result
            }
        }
    }
}