pub trait Graph {
    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn add_edge(&mut self, w: usize, v: usize);
    /// get All nodes that are at most one edge away from w
    fn adjacent(&self, w: usize) -> Vec<usize>;
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
    }

    fn adjacent(&self, w: usize) -> Vec<usize> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_edge() {
        let mut g = UndirectedDenseGraph::new(2);
        assert_eq!(g.data.len(), 2);
        assert_eq!(g.data[0].len(), 2);

        assert!(!g.data[0][1]);
        g.add_edge(0, 1);
        assert!(g.data[0][1]);
    }

    #[test]
    fn adjacent() {
        let mut g = UndirectedDenseGraph::new(5);

        g.add_edge(2, 1);
        g.add_edge(2, 3);

        assert_eq!(g.adjacent(2), vec![1, 3]);
    }
}
