use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Edge {
    node: usize,
    cost: usize,
}

pub fn dijkstra(adj_list: &Vec<Vec<Edge>>, start: usize) -> Vec<Option<usize>> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    dist.into_iter()
        .map(|x| if x == usize::MAX { None } else { Some(x) })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![
                Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 },
            ],
            // Node 3
            vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
            // Node 4
            vec![],
        ];

        let res = dijkstra(&graph, 0);
        assert_eq!(res, vec![Some(0), Some(1), Some(10), Some(3), Some(5)]);
        let res = dijkstra(&graph, 4);
        assert_eq!(res, vec![None, None, None, None, Some(0)]);
    }
}
