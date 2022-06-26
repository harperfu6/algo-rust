use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node(u32);

struct Edge(u32, u32);

struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }
}

impl From<u32> for Node {
    fn from(item: u32) -> Self {
        Node(item)
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}

impl Node {
    fn value(&self) -> u32 {
        self.0
    }

    fn neighbors(&self, graph: &Graph) -> Vec<Node> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

/// Perform a breadth-first search on Graph `graph`.
///
/// # Parameters
///
/// - `graph`: The graph to search.
/// - `root`: The starting node of the graph from which to begin searching.
/// - `target`: The target node for the search.
///
/// # Returns
///
/// If the target is found, an Optional vector is returned with the history
/// of nodes visited as its contents.
///
/// If the target is not found or there is no path from the root,
/// `None` is returned.
///
fn breadth_first_search(graph: &Graph, root: Node, target: Node) -> Option<Vec<u32>> {
    let mut queue = VecDeque::new(); // BFSの実態(双方向のキュー)
    let mut visited: HashSet<Node> = HashSet::new(); // 探索済のノード管理用
    let mut history: Vec<u32> = Vec::new(); // 探索順がわかるように探索履歴を残す（返り値になる）

    queue.push_back(root);
    visited.insert(root);
    // キューに入れられたノードを順に処理していく(キューには最初からすべてのノードをいれるのではな
    // く、上のノードから順にエッジが張られているものを処理中に入れていくのがポイント)
    while let Some(currentnode) = queue.pop_front() {
        history.push(currentnode.value());

        if currentnode == target {
            return Some(history);
        }

        for neighbor in currentnode.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor); // 今検索していたノードとエッジがはられていたノードをキューに入れる
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{breadth_first_search, Graph};

    fn graph1() -> Graph {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7), (5, 8)];
        Graph::new(
            nodes.into_iter().map(|x| x.into()).collect(),
            edges.into_iter().map(|x| x.into()).collect(),
        )
    }

    #[test]
    fn not_found_node() {
        let graph1 = graph1();
        let root = 1;
        let target = 10;

        assert_eq!(
            breadth_first_search(&graph1, root.into(), target.into()),
            None
        );
    }

    #[test]
    fn all_node() {
        let graph1 = graph1();
        let root = 1;
        let target = 8;

        let expected_history = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(
            breadth_first_search(&graph1, root.into(), target.into()),
            Some(expected_history)
        );
    }
}
