/** 
 * Breadth First Search in Rust
 * 
 * Breadth First Search is a graph traversal algorithm that traverses a graph level by level.
 * 
 * Notice in the tests that there are multiple paths to the same node but it fails to find one path.
 * This is because the algorithm uses .get() on the HashMap which returns the last inserted value, similar to .pop().
 */
use std::collections::{HashMap, VecDeque, HashSet};

type Graph = HashMap<char, Vec<char>>;

fn breadth_first_search(graph: &Graph, start: char, needle: char) -> Option<Vec<char>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut predecessors = HashMap::new();

    queue.push_back(start);
    visited.insert(start);
    predecessors.insert(start, '\0'); // Use a dummy char to indicate the start

    while let Some(current) = queue.pop_front() {
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
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                    predecessors.insert(neighbor, current);
                }
            }
        }
    }

    None // Needle not found
}

/**
 * Recursive implementation of Breadth First Search
 */
fn breadth_first_search_recursive(
    graph: &Graph, 
    queue: &mut VecDeque<char>, 
    visited: &mut HashSet<char>, 
    predecessors: &mut HashMap<char, char>, 
    needle: char
) -> Option<Vec<char>> {
    if let Some(current) = queue.pop_front() {
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
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                    predecessors.insert(neighbor, current);
                }
            }
        }
        breadth_first_search_recursive(graph, queue, visited, predecessors, needle)
    } else {
        None // Needle not found
    }
}

fn bfs_wrapper(graph: &Graph, start: char, needle: char) -> Option<Vec<char>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut predecessors = HashMap::new();

    queue.push_back(start);
    visited.insert(start);
    predecessors.insert(start, '\0'); // Use a dummy char to indicate the start

    breadth_first_search_recursive(graph, &mut queue, &mut visited, &mut predecessors, needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();
        graph.insert('A', vec!['B', 'C']);
        graph.insert('B', vec!['D']);
        graph.insert('C', vec!['E']);
        graph.insert('D', vec!['E', 'F']);
        graph.insert('E', vec!['G']);
        graph.insert('F', vec!['C']);
        graph.insert('G', vec![]);
        graph
    }

    
    fn create_test_graph2() -> Graph {
        let mut graph = Graph::new();
        graph.insert('A', vec!['B', 'C']);
        graph.insert('B', vec!['D']);
        graph.insert('C', vec![]);
        graph.insert('D', vec!['E', 'F']);
        graph.insert('E', vec!['G']);
        graph.insert('F', vec!['C']);
        graph.insert('G', vec![]);
        graph
    }
    
    #[test]
    fn test_breadth_first_search_path_found() {
        let graph = create_test_graph();
        assert_eq!(breadth_first_search(&graph, 'A', 'G'), Some(vec!['A', 'C', 'E', 'G']));
    }

    #[test]
    fn test_breadth_first_search_path_not_found() {
        let graph = create_test_graph();
        assert_eq!(breadth_first_search(&graph, 'A', 'H'), None);
    }

    // Recursive tests
    #[test]
    fn test_breadth_first_search_recursive_path_found() {
        let graph = create_test_graph();
        assert_eq!(bfs_wrapper(&graph, 'A', 'G'), Some(vec!['A', 'C', 'E', 'G']));
    }
    
    #[test]
    fn test_breadth_first_search_recursive_path2_found() {
        let graph = create_test_graph2();
        assert_eq!(bfs_wrapper(&graph, 'A', 'G'), Some(vec!['A', 'B', 'D','E', 'G']));
    }
    
    #[test]
    fn test_breadth_first_search_recursive_path2_not_found() {
        let graph = create_test_graph2();
        assert_eq!(bfs_wrapper(&graph, 'A', 'H'), None);
    }
    
    #[test]
    fn test_breadth_first_search_recursive_path_not_found() {
        let graph = create_test_graph2();
        assert_eq!(bfs_wrapper(&graph, 'A', 'H'), None);
    }
}