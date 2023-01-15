use proconio::input;
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn get_distance(a: &Point, b: &Point) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    ((dx * dx + dy * dy) as f64).sqrt()
}

fn get_score(points: &Vec<Point>, orders: &Vec<usize>) -> f64 {
    let score = orders
        .windows(2)
        .map(|x| get_distance(&points[x[0]], &points[x[1]]))
        .sum();
    score
}

/*
貪欲法
*/
#[allow(dead_code)]
fn play_greedy(points: Vec<Point>) -> Vec<usize> {
    let n = points.len();
    let mut orders = vec![0; n + 1];

    let mut current_place = 0;
    let mut visited = vec![false; n];

    visited[0] = true;

    // 貪欲法スタート
    for i in 1..n {
        let mut min_dist = 10000.00; // 現時点での距離の最小
        let mut min_id = 0; // 次はどの年に移動すれば良いか

        // 距離が最小となる都市を探す
        for j in 0..n {
            if visited[j] {
                continue;
            }
            let new_dist = get_distance(&points[current_place], &points[j]);
            if min_dist > new_dist {
                min_dist = new_dist;
                min_id = j;
            }
        }

        // 現在位置の更新
        visited[min_id] = true;
        orders[i] = min_id;
        current_place = min_id;
    }

    orders[n] = 0;
    orders
}

/*
局所探索法
*/
#[allow(dead_code)]
const MAX_CLIMBING: usize = 200000;

#[allow(dead_code)]
fn hill_climbming(points: Vec<Point>) -> Vec<usize> {
    let n = points.len();
    let mut orders = vec![0; n + 1];

    // 初期解作成
    for i in 0..n {
        orders[i] = i;
    }

    let mut rng = thread_rng();
    let mut score = get_score(&points, &orders);
    for _ in 0..MAX_CLIMBING {
        let mut l: usize = rng.gen_range(1, n);
        let mut r: usize = rng.gen_range(1, n);
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        orders[l..=r].reverse();
        let new_score = get_score(&points, &orders);
        if score > new_score {
            score = new_score;
            eprintln!("{}", score);
        } else {
            orders[l..=r].reverse();
        }
    }

    orders
}

/*
焼きなまし法
*/
const MAX_ANNEALING: usize = 200000;

fn simulated_annealing(points: Vec<Point>) -> Vec<usize> {
    let n = points.len();
    let mut orders = vec![0; n + 1];

    // 初期解作成
    for i in 0..n {
        orders[i] = i;
    }

    let mut rng = thread_rng();
    let mut score = get_score(&points, &orders);
    for i in 0..MAX_ANNEALING {
        let mut l: usize = rng.gen_range(1, n);
        let mut r: usize = rng.gen_range(1, n);
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }

        orders[l..=r].reverse();
        let new_score = get_score(&points, &orders);

        let t = 30.0 - 28.00 * (i as f64) / 200000.0;
        let prob = ((score - new_score) / t).exp().min(1.0);
        let p: f64 = rng.gen();
        if p < prob {
            score = new_score;
        } else {
            orders[l..=r].reverse();
        }
    }

    orders
}
fn main() {
    // cargo run --bin tessoku-book-a46 < src/bin/a46_input.txt > src/bin/a46_output.txt
    input! {
        n: usize,
        xys: [(i64, i64); n],
    }
    let mut points = Vec::new();
    for (x, y) in xys {
        points.push(Point { x, y });
    }

    // 貪欲法
    // let orders = play_greedy(points);

    // 局所探索法
    // let orders = hill_climbming(points);

    // 焼きなまし法
    let orders = simulated_annealing(points);

    for i in orders {
        println!("{}", i + 1);
    }
}
