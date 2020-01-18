/**
 * References GEN-1
 */
// use std::vec::Vec;
use std::collections::HashMap;

pub fn get_mean(list: &[u32]) -> f64 {
    // could have used vector : &Vec<u32>
    println!("mod called");
    // let mut sum = 0;

    // for i in 0..list.len() {
    //     sum += list[i];
    // }
    let sum: u32 = list.iter().sum(); //<- Iterator::sum
    sum as f64 / list.len() as f64
}

pub fn get_median(list: &[u32]) -> f64 {
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        get_mean(&list[(mid - 1)..(mid + 1)])
    } else {
        list[mid] as f64
    }
}

pub fn get_mode(list: &[u32]) -> u32 {
    let mut hash_map: HashMap<u32, u32> = HashMap::new();
    let mut max: (u32, u32) = (0, 0);

    for key in list {
        // get entry for number or make a new one with 0, then increament by one
        let entry = hash_map.entry(*key).or_insert(0);
        *entry += 1;
    }
    // hash_map.insert(&1001, 2002);
    for (key, val) in &hash_map {
        if max.1 < *val {
            max = (*key, *val);
        }
    }
    max.0
}
