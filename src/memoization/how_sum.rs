use std::collections::HashMap;

pub fn how_sum_example() {
    println!(
        "how_sum(7, &vec![2, 3]): {:?}",
        how_sum(7, &vec![2, 3], &mut HashMap::new())
    );
    println!(
        "how_sum(7, &vec![5, 3, 4, 7]): {:?}",
        how_sum(7, &vec![5, 3, 4, 7], &mut HashMap::new())
    );
    println!(
        "how_sum(5, &vec![2, 4]): {:?}",
        how_sum(5, &vec![2, 4], &mut HashMap::new())
    );
    println!(
        "how_sum(8, &vec![2, 3, 5]): {:?}",
        how_sum(8, &vec![2, 3, 5], &mut HashMap::new())
    );
    println!(
        "how_sum(300, &vec![7, 14]): {:?}",
        how_sum(300, &vec![7, 14], &mut HashMap::new())
    );
}

fn how_sum(
    target_sum: i32,
    numbers: &Vec<i32>,
    map: &mut HashMap<i32, Option<Vec<i32>>>,
) -> Option<Vec<i32>> {
    if target_sum == 0 {
        Some(vec![])
    } else if target_sum < 0 {
        None
    } else {
        match map.get(&target_sum) {
            Some(value) => match value {
                Some(value) => Some(value.to_vec()),
                None => None,
            },
            None => {
                for number in numbers {
                    let result = how_sum(target_sum - number, &numbers, map);
                    if result.is_some() {
                        let mut result_vec = result.unwrap();
                        result_vec.push(*number);
                        map.insert(target_sum, Some(result_vec.to_vec()));
                        return Some(result_vec);
                    };
                }

                map.insert(target_sum, None);
                None
            }
        }
    }
}
