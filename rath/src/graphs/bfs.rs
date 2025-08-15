/// Breath first search algorithms
///
///
use super::graph::Graph;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Get the path from source to destination node if it exist using Breath first search
pub fn path_to<T: Graph>(graph: &T, source: usize, destination: usize) -> Option<Vec<usize>> {
    let mut queue: BinaryHeap<usize> = BinaryHeap::new();

    // define the edges into each vertex as itself for now
    let mut incident_edge: HashMap<usize, usize> = HashMap::new();
    let mut seen: HashSet<usize> = HashSet::new();

    queue.push(source);

    // build edges: TODO we should prob cache this later
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        seen.insert(current);

        // TODO:  G is undirected, to maintain separate set of seen vertices
        for next in graph.adjacent(current) {
            if !seen.contains(&next) {
                incident_edge.insert(next, current);
                queue.push(next);
            }
        }
    }

    // no edges to destination from source
    if !incident_edge.contains_key(&destination) {
        None
    } else {
        let mut path: Vec<usize> = Vec::new();

        // walk the graph backwards from destination via the incident edges to build path to
        // source
        let mut x = destination;
        while x != source {
            path.push(x);
            x = incident_edge.get(&x).unwrap().clone();
        }

        path.push(source);
        path.reverse();

        Some(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graphs::graph::UndirectedSparseGraph;

    #[test]
    fn bfs_path_to() {
        let mut graph = UndirectedSparseGraph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let actual = path_to(&graph, 0, 2);
        assert_eq!(actual, Some(vec![0, 1, 2]));
    }

    #[test]
    fn bfs_path_to_returns_none_for_node_with_no_paths() {
        let mut graph = UndirectedSparseGraph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(5, 5);
        graph.add_edge(5, 6);

        assert_eq!(path_to(&graph, 0, 5), None);
    }
}
