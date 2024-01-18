/**
 * Recursion Example
 * 
 * Path Finding: A Maze Solver
 * 
 * Given a maze, find a path from the start to the end.
 * 
 * The maze is represented as a 2D array of characters, where:
 * # represents a wall and cannot be crossed
 * * represents the end of the maze
 * ' ' represents an open space or corridor that can be walked through
 * 
 * The maze is guaranteed to have a start and an end.
 * 
 * The path is represented as a list of coordinates, where each coordinate is a tuple of (row, column).
 */
pub fn walk(
    maze: &Vec<String>,
    curr: (isize, isize),
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<(isize, isize)>
) -> bool {
    // Out of bounds check
    if curr.0 < 0 || curr.0 >= maze.len() as isize || curr.1 < 0 || curr.1 >= maze[0].len() as isize {
        return false;
    }
    // Check for walls or seen position
    let current_pos = maze[curr.0 as usize].chars().nth(curr.1 as usize).unwrap_or('#');
    if current_pos == '#' || seen[curr.0 as usize][curr.1 as usize] {
        return false;
    }
    // If it is the end of the maze
    if current_pos == '*' {
        path.push(curr);
        return true;
    }
    // Mark the current position as part of the seen path
    seen[curr.0 as usize][curr.1 as usize] = true;
    path.push(curr);

    // Define the directions to move
    let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    
    // Check all directions
    for dir in directions.iter() {
        let next = (curr.0 + dir.0, curr.1 + dir.1);
        if walk(maze, next, end, seen, path) {
            return true;
        }
    }

    // Remove the current position from the path if no path found
    path.pop();
    seen[curr.0 as usize][curr.1 as usize] = false; // Unmark the current position
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_solver() {
        let maze = vec![
            "##########".to_string(),
            "#        #".to_string(),
            "# #### ###".to_string(),
            "# #    # #".to_string(),
            "# # #### #".to_string(),
            "# #      #".to_string(),
            "### ######".to_string(),
            "# #      #".to_string(),
            "# ## #####".to_string(),
            "#    #   #".to_string(),
            "# ###### #".to_string(),
            "#        #".to_string(),
            "##### ## #".to_string(),
            "#     #* #".to_string(), // * for the end
            "##########".to_string(),
        ];
        let mut seen = vec![vec![false; maze[0].len()]; maze.len()];
        let mut path = Vec::new();

        // Define start and end, assuming the opening at the start is (1, 0) and the end is (13, 9)
        let start = (1, 0);
//      Our end is currently at (13, 7) but you can move it wherever you want
        assert!(walk(&maze, start, &mut seen, &mut path));
        println!("Path: {:?}", path);
        println!("# of moves: {:?}", path.len());
        assert!(!path.is_empty());
        assert_eq!(path.first(), Some(&start), "Path does not start at the beginning");
        assert_eq!(path.last(), Some(&end), "Path does not lead to the end");
    }
}
