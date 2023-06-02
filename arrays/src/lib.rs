pub fn sum(a:&[i32] ) -> i32 {
	let mut sum = 0;
    for &num in a {
        sum += num;
    }
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
   [10;32]
} 