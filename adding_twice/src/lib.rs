pub fn twice<F, T>(f: F) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
    T: Copy,
{
    move |x| f(f(x))
}

pub fn add_curry(initial: i32) -> impl Fn(i32) -> i32 {
    move |x| initial + x
}