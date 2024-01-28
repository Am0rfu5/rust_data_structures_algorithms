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
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

type Graph = HashMap<char, HashMap<char, u32>>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: char,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Notice that the we flip the ordering here
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstras_shortest_path(graph: &Graph, start: char, needle: char) -> Option<Vec<char>> {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut predecessors = HashMap::new();
    let mut visited = HashSet::new();

    distances.insert(start, 0);
    heap.push(Reverse(State { cost: 0, position: start }));
    predecessors.insert(start, '\0'); // Use a dummy char to indicate the start

    while let Some(Reverse(State { cost, position })) = heap.pop() {
        if position == needle {
            let mut path = vec![needle];
            let mut prev = predecessors[&needle];
            while prev != '\0' {
                path.push(prev);
                prev = predecessors[&prev];
            }
            path.reverse();
            return Some(path);
        }

        // Important: skip if this node is already visited
        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        if let Some(neighbors) = graph.get(&position) {
            for (&neighbor, &weight) in neighbors {
                if visited.contains(&neighbor) {
                    continue;
                }

                let next = State {
                    cost: cost + weight,
                    position: neighbor,
                };

                if !distances.contains_key(&neighbor) || next.cost < distances[&neighbor] {
                    heap.push(Reverse(next));
                    distances.insert(neighbor, next.cost);
                    predecessors.insert(neighbor, position);
                }
            }
        }
    }

    None
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