pub fn stars(n: u32) -> String {
// let mut s=String::new();
// for i in 0..=n{
//     let  char='*';
//     // let mut x: u32=2;
//     // x*=2;
//     s.push(char);
// }
// s
"*".to_string().repeat(u32::pow(2,n) as usize)

}
