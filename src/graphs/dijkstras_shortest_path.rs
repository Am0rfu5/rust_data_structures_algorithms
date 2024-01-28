/**
 * Dijkstra's Shortest Path Algorithm
 * 
 * Returns the shortest path from the start node to the needle node, if it exists.
 * 
 * Greedy Algorithm
 * 
 * Time Complexity: O(|V|^2)
 * Space Complexity: O(|V|)
 * 
 * @param graph The graph to search
 * @param start The starting node
 * @param needle The node to search for in the graph
 * @return A path from the start to the needle, if it exists 
 */
use std::collections::{HashMap, HashSet};

type Graph = HashMap<char, HashMap<char, u32>>;

fn dijkstras_shortest_path(graph: &Graph, start: char, needle: char) -> Option<Vec<char>> {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut predecessors = HashMap::new();
    
    distances.insert(start, 0); // Distance from start to start is 0
    predecessors.insert(start, '\0'); // Use a dummy char to indicate the start

    while let Some(current) = get_closest_unvisited_node(&distances, &visited) {
        // base case
        if current == needle {
            let mut path = vec![needle]; // Start with the needle
            let mut prev = predecessors[&needle]; // Get the predecessor of the needle
            while prev != '\0' {
                path.push(prev); // Add the predecessor to the path
                prev = predecessors[&prev]; // Get the predecessor of the predecessor
            }
            path.reverse(); // Reverse the path so it goes from start to needle
            return Some(path); 
        }

        visited.insert(current); // Mark the current node as visited
        
        // Update the distances and predecessors of the neighbors of the current node
        if let Some(neighbors) = graph.get(&current) {
            for (&neighbor, &weight) in neighbors {
                if !visited.contains(&neighbor) {
                    let new_distance = distances[&current] + weight;
                    if !distances.contains_key(&neighbor) || new_distance < distances[&neighbor] {
                        distances.insert(neighbor, new_distance);
                        predecessors.insert(neighbor, current);
                    }
                }
            }
        }
    }

    None
}

fn get_closest_unvisited_node(
    distances: &HashMap<char, u32>,
    visited: &HashSet<char>,
) -> Option<char> {
    let mut closest_node = None;
    let mut closest_distance = std::u32::MAX;

    for (&node, &distance) in distances {
        if !visited.contains(&node) && distance < closest_distance {
            closest_node = Some(node);
            closest_distance = distance;
        }
    }

    closest_node
}

// Dijkstra's Shortest Path Algorithm - Recursive
fn dijkstras_shortest_path_recursive(
    graph: &Graph,
    visited: &mut HashSet<char>,
    distances: &mut HashMap<char, u32>,
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

    visited.insert(current);

    if let Some(neighbors) = graph.get(&current) {
        for (&neighbor, &weight) in neighbors {
            if !visited.contains(&neighbor) {
                let new_distance = distances[&current] + weight;
                if !distances.contains_key(&neighbor) || new_distance < distances[&neighbor] {
                    distances.insert(neighbor, new_distance);
                    predecessors.insert(neighbor, current);
                }
            }
        }
    }

    if let Some(next) = get_closest_unvisited_node(&distances, &visited) {
        dijkstras_shortest_path_recursive(
            graph,
            visited,
            distances,
            predecessors,
            next,
            needle,
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstras_shortest_path() {
        let mut graph = HashMap::new();
        graph.insert('A', HashMap::new());
        graph.insert('B', HashMap::new());
        graph.insert('C', HashMap::new());
        graph.insert('D', HashMap::new());
        graph.insert('E', HashMap::new());
        graph.insert('F', HashMap::new());
        graph.insert('G', HashMap::new());
        graph.insert('H', HashMap::new());

        graph.get_mut(&'A').unwrap().insert('B', 3);
        graph.get_mut(&'A').unwrap().insert('C', 6);
        graph.get_mut(&'A').unwrap().insert('D', 1);

        graph.get_mut(&'B').unwrap().insert('A', 3);
        graph.get_mut(&'B').unwrap().insert('C', 2);
        graph.get_mut(&'B').unwrap().insert('D', 4);
        graph.get_mut(&'B').unwrap().insert('E', 7);

        graph.get_mut(&'C').unwrap().insert('A', 6);
        graph.get_mut(&'C').unwrap().insert('B', 2);
        graph.get_mut(&'C').unwrap().insert('E', 6);
        graph.get_mut(&'C').unwrap().insert('F', 2);

        graph.get_mut(&'D').unwrap().insert('A', 1);
        graph.get_mut(&'D').unwrap().insert('B', 4);
        graph.get_mut(&'D').unwrap().insert('E', 3);
        graph.get_mut(&'D').unwrap().insert('G', 5);

        graph.get_mut(&'E').unwrap().insert('B', 7);
        graph.get_mut(&'E').unwrap().insert('C', 6);
        graph.get_mut(&'E').unwrap().insert('D', 3);
        graph.get_mut(&'E').unwrap().insert('F', 8);
        graph.get_mut(&'E').unwrap().insert('G', 3);
        graph.get_mut(&'E').unwrap().insert('H', 4);

        graph.get_mut(&'F').unwrap().insert('C', 2);
        graph.get_mut(&'F').unwrap().insert('E', 8);
        graph.get_mut(&'F').unwrap().insert('H', 7);
        graph.get_mut(&'F').unwrap().insert('G', 9);
        
        graph.get_mut(&'G').unwrap().insert('D', 5);
        graph.get_mut(&'G').unwrap().insert('E', 3);
        graph.get_mut(&'G').unwrap().insert('F', 9);
        graph.get_mut(&'G').unwrap().insert('H', 2);
        
        graph.get_mut(&'H').unwrap().insert('E', 4);
        graph.get_mut(&'H').unwrap().insert('F', 7);
        graph.get_mut(&'H').unwrap().insert('G', 2);
        
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'H'), Some(vec!['A', 'D', 'E', 'H']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'G'), Some(vec!['A', 'D', 'G']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'F'), Some(vec!['A', 'D', 'E', 'F']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'E'), Some(vec!['A', 'D', 'E']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'D'), Some(vec!['A', 'D']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'C'), Some(vec!['A', 'C']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'B'), Some(vec!['A', 'B']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'A'), Some(vec!['A']));
        assert_eq!(dijkstras_shortest_path(&graph, 'A', 'Z'), None);
        
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'H'), Some(vec!['A', 'D', 'E', 'H']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'G'), Some(vec!['A', 'D', 'G']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'F'), Some(vec!['A', 'D', 'E', 'F']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'E'), Some(vec!['A', 'D', 'E']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'D'), Some(vec!['A', 'D']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'C'), Some(vec!['A', 'C']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'B'), Some(vec!['A', 'B']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'A'), Some(vec!['A']));
        assert_eq!(dijkstras_shortest_path_recursive(&graph, &mut HashSet::new(), &mut HashMap::new(), &mut HashMap::new(), 'A', 'Z'), None);
    }
}