pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nbrs: Vec<u32> = s.split_whitespace().map(|nbr_to_str| {
            if nbr_to_str.ends_with('k') {
                nbr_to_str[..nbr_to_str.len() - 1].parse::<f32>().unwrap() * 1000.0
            } else {
                nbr_to_str.parse::<f32>().unwrap()
            }
        }).map(|f| f as u32).collect();
    Box::new(nbrs)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}