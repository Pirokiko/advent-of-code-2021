/// Define types to ensure only the expected values are used,
/// instead of mixing the usage of edges and vertices
type VertexIndex = usize;
type EdgeIndex = usize;

/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
pub struct Graph {
    /// Maps a vertex id to the first edge in its adjacency list.
    first: Vec<Option<EdgeIndex>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    next: Vec<Option<EdgeIndex>>,
    /// Maps an edge id to the vertex that it points to.
    endp: Vec<VertexIndex>,
}

impl Graph {
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, emax_hint should be close to the number of
    /// edges that will be inserted.
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            first: vec![None; vmax],
            next: Vec::with_capacity(emax_hint),
            endp: Vec::with_capacity(emax_hint),
        }
    }

    /// Returns the number of vertices.
    pub fn num_v(&self) -> usize {
        self.first.len()
    }

    /// Returns the number of edges, double-counting undirected edges.
    pub fn num_e(&self) -> usize {
        self.endp.len()
    }

    /// Adds a directed edge from u to v. Returns the edge id
    pub fn add_edge(&mut self, u: VertexIndex, v: VertexIndex) -> EdgeIndex {
        let edge_id = self.num_e();
        self.next.push(self.first[u]);
        self.first[u] = Some(edge_id);
        self.endp.push(v);
        edge_id
    }

    /// An undirected edge is two directed edges. If edges are added only via
    /// this function, the reverse of any edge e can be found at e^1.
    pub fn add_undirected_edge(
        &mut self,
        u: VertexIndex,
        v: VertexIndex,
    ) -> (EdgeIndex, EdgeIndex) {
        (self.add_edge(u, v), self.add_edge(v, u))
    }

    /// If we think of each even-numbered vertex as a variable, and its
    /// odd-numbered successor as its negation, then we can build the
    /// implication graph corresponding to any 2-CNF formula.
    /// Note that u||v == !u -> v == !v -> u.
    pub fn add_two_sat_clause(&mut self, u: VertexIndex, v: VertexIndex) {
        self.add_edge(u ^ 1, v);
        self.add_edge(v ^ 1, u);
    }

    /// Gets vertex u's adjacency list.
    pub fn adj_list(&self, u: VertexIndex) -> AdjListIterator {
        AdjListIterator {
            graph: self,
            next_e: self.first[u],
        }
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct AdjListIterator<'a> {
    graph: &'a Graph,
    next_e: Option<EdgeIndex>,
}

impl<'a> Iterator for AdjListIterator<'a> {
    type Item = (EdgeIndex, VertexIndex);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_e.map(|e| {
            let v = self.graph.endp[e];
            self.next_e = self.graph.next[e];
            (e, v)
        })
    }
}
