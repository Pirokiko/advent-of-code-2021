use crate::lib::graph2::Graph;
use crate::puzzle15::ShortestPath;

fn char_to_usize(char: char) -> usize {
    usize::from_str_radix(&char.to_string(), 10).expect("char to be a digit")
}

fn vertex(row: usize, column: usize, size: usize) -> usize {
    row * size + column
}

fn edges(row: usize, column: usize, size: usize) -> Vec<(usize, usize)> {
    let mut edge_vertices = vec![];
    if row > 0 {
        edge_vertices.push((row - 1, column));
    }

    if row < size - 1 {
        edge_vertices.push((row + 1, column));
    }

    if column > 0 {
        edge_vertices.push((row, column - 1));
    }

    if column < size - 1 {
        edge_vertices.push((row, column + 1));
    }

    edge_vertices
}

pub fn parse_weight_map(content: &str) -> Vec<Vec<usize>> {
    content
        .lines()
        .map(|line| line.chars().map(char_to_usize).collect())
        .collect()
}

pub fn parse_graph_and_weights(weight_map: Vec<Vec<usize>>) -> (Graph, Vec<usize>) {
    let nr_of_lines = weight_map.len();
    let nr_of_vertices = nr_of_lines * nr_of_lines;

    let nr_of_edges = 4 * nr_of_vertices - 4 * nr_of_lines;

    let mut graph = Graph::new(nr_of_vertices, nr_of_edges);
    let mut weights: Vec<usize> = vec![usize::MAX; nr_of_edges];

    for (row_idx, row) in weight_map.iter().enumerate() {
        for (column_idx, weight) in row.iter().enumerate() {
            let vertex_idx = vertex(row_idx, column_idx, nr_of_lines);

            for (row, column) in edges(row_idx, column_idx, nr_of_lines) {
                let edge = graph.add_edge(vertex(row, column, nr_of_lines), vertex_idx);
                weights[edge] = *weight;
            }
        }
    }

    (graph, weights)
}

pub fn build_shortest_path(graph: Graph, weights: Vec<usize>) -> ShortestPath {
    let end = graph.num_v() - 1;
    ShortestPath {
        graph,
        start: 0,
        end,
        weights,
    }
}
