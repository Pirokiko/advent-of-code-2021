use crate::lib::graph::NodeIndex;
use crate::lib::graph2::Graph;

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

fn parse(content: &str) -> (Graph, Vec<usize>, NodeIndex) {
    let nr_of_lines = content.lines().count();
    let nr_of_vertices = nr_of_lines * nr_of_lines;

    let mapped_weights: Vec<Vec<usize>> = content
        .lines()
        .map(|line| line.chars().map(char_to_usize).collect())
        .collect();

    let nr_of_edges = 4 * nr_of_vertices - 4 * nr_of_lines;

    let mut graph = Graph::new(nr_of_vertices, nr_of_edges);
    let mut weights: Vec<usize> = vec![usize::MAX; nr_of_edges];

    for (row_idx, row) in mapped_weights.iter().enumerate() {
        for (column_idx, weight) in row.iter().enumerate() {
            let vertex_idx = vertex(row_idx, column_idx, nr_of_lines);

            for (row, column) in edges(row_idx, column_idx, nr_of_lines) {
                let edge = graph.add_edge(vertex(row, column, nr_of_lines), vertex_idx);
                weights[edge] = *weight;
            }
        }
    }

    (graph, weights, nr_of_vertices - 1)
}

pub fn part1(content: &str) -> usize {
    let (graph, weights, end_index) = parse(content);

    let distances = graph.dijkstra(weights.as_slice(), end_index);

    distances[0]
}

#[cfg(test)]
mod tests {
    use crate::puzzle15::part1::part1;

    static TEST_CONTENT: &'static str = include_str!("test.txt");

    #[test]
    fn part1_works() {
        assert_eq!(40, part1(TEST_CONTENT));
    }
}
