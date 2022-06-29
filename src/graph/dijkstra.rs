#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    ops::Add,
};

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: &V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new(); // 最終的な(スタートから)各ノードへの最短距離
    let mut priority = BinaryHeap::new(); // 探索対象について優先度つきキューで管理

    ans.insert(*start, None);

    for (new, weight) in &graph[start] {
        ans.insert(*new, Some((*start, *weight)));
        priority.push(Reverse((*weight, new, start))); // weightが小さいほうが優先度が高いためReverseでラップして入れる
    }

    // Reverseでラップしているのでその形で取り出す
    while let Some(Reverse((dist_new, new, prev))) = priority.pop() {
        match ans[new] {
            // what we popped is what is in ans, we'll compute it
            Some((p, d)) if p == *prev && d == dist_new => {} // go to next process
            _ => continue,                                    // skip next process
        }
        for (next, weight) in &graph[new] {
            match ans.get(next) {
                // すでに格納されている距離のほうが短ければ何もしない
                Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
                // 何も格納されていなければstartなので何もしない
                Some(None) => {}
                _ => {
                    ans.insert(*next, Some((*new, dist_new + *weight)));
                    priority.push(Reverse((dist_new + *weight, next, new))); // 探索対象に入れる
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::{dijkstra, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
        graph.entry(v2).or_insert_with(BTreeMap::new);
    }

    // #[test]
    // fn single_edge() {
    //     let mut graph = BTreeMap::new();
    //     add_edge(&mut graph, 0, 1, 2);
    //     dbg!(&graph);

    //     let mut dists_0 = BTreeMap::new();
    //     dists_0.insert(0, None);
    //     dists_0.insert(1, Some((0, 2)));

    //     assert_eq!(dijkstra(&graph, &0), dists_0);
    // }

    #[test]
    fn graph_1() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 'a', 'c', 12);
        add_edge(&mut graph, 'a', 'd', 60);
        add_edge(&mut graph, 'b', 'a', 10);
        add_edge(&mut graph, 'c', 'b', 20);
        add_edge(&mut graph, 'c', 'd', 32);
        add_edge(&mut graph, 'e', 'a', 7);

        let mut dists_a = BTreeMap::new();
        dists_a.insert('a', None);
        dists_a.insert('c', Some(('a', 12)));
        dists_a.insert('d', Some(('c', 44)));
        dists_a.insert('b', Some(('c', 32)));
        assert_eq!(dijkstra(&graph, &'a'), dists_a);

        let mut dists_b = BTreeMap::new();
        dists_b.insert('b', None);
        dists_b.insert('a', Some(('b', 10)));
        dists_b.insert('c', Some(('a', 22)));
        dists_b.insert('d', Some(('c', 54)));
        assert_eq!(dijkstra(&graph, &'b'), dists_b);

        let mut dists_c = BTreeMap::new();
        dists_c.insert('c', None);
        dists_c.insert('b', Some(('c', 20)));
        dists_c.insert('d', Some(('c', 32)));
        dists_c.insert('a', Some(('b', 30)));
        assert_eq!(dijkstra(&graph, &'c'), dists_c);

        let mut dists_d = BTreeMap::new();
        dists_d.insert('d', None);
        assert_eq!(dijkstra(&graph, &'d'), dists_d);

        let mut dists_e = BTreeMap::new();
        dists_e.insert('e', None);
        dists_e.insert('a', Some(('e', 7)));
        dists_e.insert('c', Some(('a', 19)));
        dists_e.insert('d', Some(('c', 51)));
        dists_e.insert('b', Some(('c', 39)));
        assert_eq!(dijkstra(&graph, &'e'), dists_e);
    }
}
