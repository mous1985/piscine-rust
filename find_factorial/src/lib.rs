pub fn factorial(num: u64) -> u64 {
    if num==0{
        return 1;
    }else if num==1{
        return 1;
    }
    let mut result: u64 = 1;
    for i in 1..=num {
        result *= i;
    }
    result
}
