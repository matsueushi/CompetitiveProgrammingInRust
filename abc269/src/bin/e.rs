// use proconio::{fastout, input, source::line::LineSource};

// #[fastout]
// fn main() {
//     let stdin = std::io::stdin();
//     let mut source = LineSource::new(stdin.lock());

//     input! {
//         from &mut source,
//         n: usize,
//     }
//     drop(source);

//     let mut l = 1;
//     let mut r = 10;
//     for _ in 0..10 {
//         let mut source = LineSource::new(stdin.lock());
//         let m = (l + r) / 2;
//         println!("? {} {} 1 {}", l, m, n);
//         input! {
//             from &mut source,
//             count: usize,
//         }
//         drop(source);

//         if count == l - m + 1 {
//             l = m + 1;
//         } else {
//             r = m;
//         }
//     }
//     let x = l;

//     l = 1;
//     r = 10;
//     for _ in 0..10 {
//         let mut source = LineSource::new(stdin.lock());
//         let m = (l + r) / 2;
//         println!("? {} {} {} {}", x, x, l, m);
//         input! {
//             from &mut source,
//             count: usize,
//         }
//         drop(source);
//         if count == l - m + 1 {
//             l = m + 1;
//         } else {
//             r = m;
//         }
//     }
//     let y = l;

//     println!("! {} {}", x, y);
//     drop(source);
// }
