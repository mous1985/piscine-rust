pub fn fibonacci(n : u32 )-> u32 {
    if n == 0  {
        return 0;
    }else if n == 1 {
        return 1;
    }
    let mut a :u32=0;
    let mut b :u32=1;
    let mut result :u32=0;
    for _ in 2..=n {
        result=a+b;
        a=b;
        b=result;
    }
    result
}
