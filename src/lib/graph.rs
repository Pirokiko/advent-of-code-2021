#[derive(Default, Debug)]
pub struct Graph<Val> {
    nodes: Vec<NodeData<Val>>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;

#[derive(Debug)]
pub struct NodeData<Val> {
    first_outgoing_edge: Option<EdgeIndex>,
    value: Val,
}

impl<Val> NodeData<Val> {
    pub fn value(&self) -> &Val {
        &self.value
    }
    pub fn value_mut(&mut self) -> &mut Val {
        &mut self.value
    }
}

pub type EdgeIndex = usize;

#[derive(Debug)]
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl<Val> Graph<Val> {
    pub fn add_node(&mut self, value: Val) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing_edge: None,
            value,
        });
        index
    }

    pub fn get_node(&self, index: NodeIndex) -> &NodeData<Val> {
        &self.nodes[index]
    }

    pub fn get_node_mut(&mut self, index: NodeIndex) -> &mut NodeData<Val> {
        &mut self.nodes[index]
    }
}

impl<Val> Graph<Val> {
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }
}

impl<Val> Graph<Val> {
    pub fn successors(&self, source: NodeIndex) -> Successors<Val> {
        Successors {
            graph: self,
            current_edge_index: self.nodes[source].first_outgoing_edge,
        }
    }

    pub fn nodes(&self) -> AllNodes<Val> {
        AllNodes {
            graph: self,
            current_index: if self.nodes.len() > 0 { Some(0) } else { None },
        }
    }
}

pub struct Successors<'graph, Val> {
    graph: &'graph Graph<Val>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph, Val> Iterator for Successors<'graph, Val> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

pub struct AllNodes<'graph, Val> {
    graph: &'graph Graph<Val>,
    current_index: Option<NodeIndex>,
}

impl<'graph, Val> Iterator for AllNodes<'graph, Val> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_index {
            None => None,
            Some(node_index) => {
                let next = node_index + 1;
                self.current_index = if next >= self.graph.nodes.len() {
                    None
                } else {
                    Some(next)
                };
                Some(node_index)
            }
        }
    }
}
