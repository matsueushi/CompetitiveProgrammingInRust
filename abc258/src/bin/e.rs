// use core::cmp::Ordering::{Greater, Less};
// use itertools_num::ItertoolsNum;
// use proconio::{fastout, input};

// // naive
// // fn main() {
// //     let n = 10;
// //     let q = 5;
// //     let x = 20;
// //     let ws = vec![5, 8, 5, 9, 8, 7, 4, 4, 8, 2];

// //     let mut k = 100;

// //     let mut i = 0;
// //     let mut weight = 0;
// //     let mut res = vec![];
// //     while k > 0 {
// //         weight += ws[i % n];
// //         if weight >= x {
// //             res.push(i + 1);
// //             weight = 0;
// //             k -= 1;
// //         }
// //         i += 1;
// //     }

// //     let mut ns = vec![res[0]];
// //     for i in 0..res.len() - 1 {
// //         ns.push(res[i + 1] - res[i]);
// //     }
// //     println!("{:?}", res.iter().map(|&x| x % n).collect::<Vec<_>>());
// //     println!("{:?}", ns);
// // }

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         q: usize,
//         x: usize,
//         ws: [usize; n],
//         ks: [usize; q],
//     }

//     // let n = 10;
//     // let q = 5;
//     // let x = 20;
//     // let ws = vec![5, 8, 5, 9, 8, 7, 4, 4, 8, 2];
//     // let ks = vec![100];

//     // sum of ws
//     let wsum: usize = ws.iter().sum();
//     // println!("{}", wsum);

//     // cumsum
//     // 0, w0, w0 + w1, ...
//     let csum = std::iter::once(0usize)
//         .chain(ws.clone().into_iter())
//         .chain(ws.into_iter())
//         .cumsum::<usize>()
//         .collect::<Vec<_>>();
//     // println!("{:?}", csum);

//     let cyc = x / wsum;
//     let rem = x % wsum;

//     let mut i = 0;
//     let mut used = vec![n + 1; n];
//     let mut cs = vec![];
//     used[0] = 0;
//     let mut st = 0;
//     let mut en = 0;
//     for k in 1..n + 1 {
//         // searchsortedfirst
//         let c = csum
//             .binary_search_by(|&y| if y < csum[i] + rem { Less } else { Greater })
//             .unwrap_or_else(|i| i);
//         // println!("i {}", i);
//         // println!("c - i {}", c - i);
//         cs.push(wsum * cyc + c - i);
//         i = c % n;
//         if used[i] <= n {
//             st = used[i];
//             en = k;
//             // println!("{} {}", st, en);
//             break;
//         }
//         used[i] = k;
//     }

//     // println!("{:?}", cs);
//     for k in ks {
//         if k <= st {
//             println!("{}", cs[k - 1]);
//         } else {
//             let mut c = (k - st) % (en - st);
//             if c == 0 {
//                 c = en - st;
//             }
//             // println!("{} {}", st, c);
//             println!("{}", cs[st + c - 1]);
//         }
//     }
// }
