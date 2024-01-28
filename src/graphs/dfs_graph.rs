/**
 * Depth First Search
 *
 * Returns a path from the start to the needle, if it exists.
 * 
 * There is another example of DFS in the `recursion` directory.
 * 
 * Time Complexity: O(V + E)
 * Space Complexity: O(V)
 * 
 * @param graph The graph to search
 * @param start The starting node
 * @param needle The node to search for in the graph
 * @return A path from the start to the needle, if it exists 
 */
use std::collections::{HashMap, HashSet};
  
type Graph = HashMap<char, Vec<char>>;

fn depth_first_search_recursive(
    graph: &Graph,
    visited: &mut HashSet<char>,
    predecessors: &mut HashMap<char, char>,
    current: char,
    needle: char,
) -> Option<Vec<char>> {
    if current == needle {
        let mut path = vec![needle];
        let mut prev = predecessors[&needle];
        while prev != '\0' {
            path.push(prev);
            prev = predecessors[&prev];
        }
        path.reverse();
        return Some(path);
    }

    if let Some(neighbors) = graph.get(&current) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                predecessors.insert(neighbor, current);
                if let Some(path) = depth_first_search_recursive(
                    graph,
                    visited,
                    predecessors,
                    neighbor,
                    needle,
                ) {
                    return Some(path);
                }
            }
        }
    }

    None
}
 
fn depth_first_search(graph: &Graph, start: char, needle: char) -> Option<Vec<char>> {
    let mut visited = HashSet::new();
    let mut predecessors = HashMap::new();
    visited.insert(start);
    predecessors.insert(start, '\0'); // Use a dummy char to indicate the start

    depth_first_search_recursive(
        graph,
        &mut visited,
        &mut predecessors,
        start,
        needle,
    )
}   
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_depth_first_search() {
        let mut graph = HashMap::new();
        graph.insert('A', vec!['B', 'C']);
        graph.insert('B', vec!['D', 'E']);
        graph.insert('C', vec!['F']);
        graph.insert('D', vec!['G', 'H']);
        graph.insert('E', vec!['I']);
        graph.insert('F', vec!['J']);
        graph.insert('G', vec!['K']);
        graph.insert('H', vec!['L']);
        graph.insert('I', vec!['M','N']);
        graph.insert('J', vec!['N']);
        graph.insert('K', vec!['O']);
        graph.insert('L', vec!['P']);
        graph.insert('M', vec!['Q']);
        graph.insert('N', vec!['R']);
        graph.insert('O', vec!['S']);
        graph.insert('P', vec!['T']);
        graph.insert('Q', vec!['U']);
        graph.insert('R', vec!['V']);
        graph.insert('S', vec!['W']);
        graph.insert('T', vec!['X']);
        graph.insert('U', vec!['Y']);
        graph.insert('V', vec!['Z']);
        graph.insert('W', vec![]);
        graph.insert('X', vec![]);
        graph.insert('Y', vec!['R']);
        graph.insert('Z', vec![]);
 
        assert_eq!(
            depth_first_search(&graph, 'A', 'Z'),
            Some(vec!['A', 'B', 'E', 'I', 'N', 'R', 'V', 'Z'])
        );
        assert_eq!(
            depth_first_search(&graph, 'A', 'W'),
            Some(vec!['A', 'B', 'D', 'G', 'K', 'O', 'S', 'W'])
        );
        assert_eq!(
            depth_first_search(&graph, 'A', 'A'),
            Some(vec!['A'])
        );
        assert_eq!(
            depth_first_search(&graph, 'A', 'X'),
            None
        );
    }
}