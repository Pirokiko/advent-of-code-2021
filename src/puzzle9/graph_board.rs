use crate::puzzle9::graph::{Graph, NodeData, NodeIndex};

pub struct GraphBoard {
    graph: Graph<usize>,
}

impl GraphBoard {
    fn from(graph: Graph<usize>) -> GraphBoard {
        GraphBoard { graph }
    }

    pub fn low_points(&self) -> Vec<&NodeData<usize>> {
        self.graph
            .nodes()
            .filter(|index| self.is_low_point(index))
            .map(|index| self.graph.get_node(index))
            .collect()
    }

    fn is_low_point(&self, source: &NodeIndex) -> bool {
        let source_val = self.graph.get_node(*source).value();
        println!("source_val: {}", source_val);
        self.graph.successors(*source).all(|index| {
            let index_value = self.graph.get_node(index).value();
            println!("source_val: {}, index_value: {}", *source_val, *index_value);
            index_value.gt(source_val)
        })
    }
}

pub fn parse(content: &str) -> GraphBoard {
    let mut graph: Graph<usize> = Graph::default();

    let mut indices: Vec<Vec<NodeIndex>> = vec![];
    for (row, line) in content.lines().enumerate() {
        let mut row_indices: Vec<NodeIndex> = vec![];
        for (column, digit) in line.chars().enumerate() {
            let node_index =
                graph.add_node(usize::from_str_radix(&format!("{}", digit), 10).unwrap());
            row_indices.push(node_index);

            if row > 0 {
                let up_index = indices[row - 1][column];
                graph.add_edge(node_index, up_index);
                graph.add_edge(up_index, node_index);
            }
            if column > 0 {
                let left_index = row_indices[column - 1];
                graph.add_edge(node_index, left_index);
                graph.add_edge(left_index, node_index);
            }
        }
        indices.push(row_indices);
    }

    GraphBoard::from(graph)
}
