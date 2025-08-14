use std::collections::{HashMap, HashSet};

pub trait Graph {
    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn add_edge(&mut self, w: usize, v: usize);
    /// get All nodes that are at most one edge away from w
    fn adjacent(&self, w: usize) -> HashSet<usize>;
}

/// undirected graph backed by an nxn matrix, that is pre allocated.
pub struct UndirectedDenseGraph {
    num_vertices: usize,
    num_edges: usize,
    data: Vec<Vec<bool>>,
}

impl UndirectedDenseGraph {
    pub fn new(num_vertices: usize) -> Self {
        let mut data: Vec<Vec<bool>> = Vec::new();
        data.resize_with(num_vertices, || {
            std::iter::repeat(false)
                .take(num_vertices)
                .collect::<Vec<bool>>()
        });

        Self {
            num_vertices,
            num_edges: 0,
            data,
        }
    }
}

impl Graph for UndirectedDenseGraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn add_edge(&mut self, w: usize, v: usize) {
        self.data[w][v] = true;
        self.data[v][w] = true;
        self.num_edges += 1;
    }

    fn adjacent(&self, w: usize) -> HashSet<usize> {
        self.data[w]
            .iter()
            .enumerate()
            .filter_map(
                |(v, &is_connected)| {
                    if is_connected {
                        Some(v)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }
}

/// undirected graph backed by a adjency list data structure
pub struct UndirectedSparseGraph {
    data: HashMap<usize, HashSet<usize>>,
}

impl UndirectedSparseGraph {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Graph for UndirectedSparseGraph {
    fn num_vertices(&self) -> usize {
        self.data.len()
    }

    fn num_edges(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            let temp: HashSet<&usize> =
                HashSet::from_iter(self.data.values().flat_map(|h| h.iter()));

            temp.len() - 1
        }
    }

    fn add_edge(&mut self, w: usize, v: usize) {
        self.data
            .entry(w)
            .and_modify(|neighbors| {
                neighbors.insert(v);
            })
            .or_insert(HashSet::from([v]));
        self.data
            .entry(v)
            .and_modify(|neighbors| {
                neighbors.insert(w);
            })
            .or_insert(HashSet::from([w]));
    }

    fn adjacent(&self, w: usize) -> HashSet<usize> {
        self.data
            .get(&w)
            .map(|v| v.clone())
            .unwrap_or(HashSet::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn undirected_dense_add_edge() {
        let mut g = UndirectedDenseGraph::new(2);
        assert_eq!(g.data.len(), 2);
        assert_eq!(g.data[0].len(), 2);

        assert!(!g.data[0][1]);
        g.add_edge(0, 1);
        assert!(g.data[0][1]);
    }

    #[test]
    fn undirected_dense_adjacent() {
        let mut g = UndirectedDenseGraph::new(5);
        assert_eq!(g.num_edges(), 0);

        g.add_edge(2, 1);
        g.add_edge(2, 3);

        assert_eq!(g.num_edges(), 2);
        assert_eq!(g.adjacent(2), HashSet::from([1, 3]));
    }

    #[test]
    fn undirected_sparse_add_edge() {
        let mut g = UndirectedSparseGraph::new();
        assert_eq!(g.data.len(), 0);

        g.add_edge(0, 1);
        assert_eq!(g.data.get(&0).unwrap().clone(), HashSet::from([1]));
    }

    #[test]
    fn undirected_sparse_adjacent() {
        let mut g = UndirectedSparseGraph::new();
        assert_eq!(g.num_edges(), 0);

        g.add_edge(2, 1);
        g.add_edge(2, 3);

        assert_eq!(g.num_edges(), 2);
        let expected: HashSet<usize> = HashSet::from([1, 3]);
        assert_eq!(g.adjacent(2), expected);
    }
}
