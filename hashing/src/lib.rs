pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum(); 
    let count = list.len() as f64; 
    let mean = sum as f64 / count; 

    mean
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut list_final = list.clone(); 
    list_final.sort();
    let length = list_final.len();
    if length % 2 == 0 {
        let x = length / 2;
        (list_final[x - 1] + list_final[x]) / 2
    } else {
        
        list_final[length / 2]
    }
}
use std::collections::HashMap;
pub fn mode(list: &Vec<i32>) -> i32 {
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for &num in list {
        let count = count_map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (&num, &count) in count_map.iter() {
        if count > max_count {
            mode = num;
            max_count = count;
        }
    }

    mode
}
