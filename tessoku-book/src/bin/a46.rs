use proconio::input;

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

fn play_greedy(n: usize, points: Vec<Point>) -> Vec<usize> {
    let mut order = vec![0; n];

    let mut current_place = 1;
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
        order[i] = min_id;
        current_place = min_id;
    }

    order
}

fn main() {
    input! {
        n: usize,
        xys: [(i64, i64); n],
    }
    let mut points = Vec::new();
    for (x, y) in xys {
        points.push(Point { x, y });
    }
    let order = play_greedy(n, points);
    for i in order {
        println!("{}", i + 1);
    }
    println!("1");
}
