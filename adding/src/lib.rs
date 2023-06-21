pub fn add_curry(initial: i32) -> impl Fn(i32) -> i32 {
    move |x| initial + x
}