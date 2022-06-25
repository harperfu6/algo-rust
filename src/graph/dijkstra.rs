use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    fmt::Debug,
};

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

fn dijkstra<V: Ord + Copy + Debug, E: Ord + Copy + Debug>(
    graph: &Graph<V, E>,
    start: &V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut priority = BinaryHeap::new();

    ans.insert(*start, None);

    for (new, weight) in &graph[start] {
        ans.insert(*new, Some((*start, *weight)));
        priority.push(Reverse((*weight, new, start))); // weightが小さいほうが優先度が高いためReverseでラップして入れる
    }
    dbg!(&ans);
    println!("=========");
    dbg!(&priority);

    while let Some(Reverse((dist_new, new, prev))) = priority.pop() {
        match ans[new] {
            // what we popped is what is in ans, we'll compute it
            Some((p, d)) if p == *prev && d == dist_new => {} // go to next process
            _ => continue,                                    // skip next process
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

        dijkstra(&graph, &'a');
    }
}
