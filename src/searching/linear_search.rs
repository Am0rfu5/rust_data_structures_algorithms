/** O(n) Linear Time Complexity
 * 
 * Example - Linear Search Algorithm
 * 
 * A Linear search is also called a Basic Search.
 * For unordered data, the most basic search is walking an array.
 */
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr = [1, 5, 2, 4, 3];
        assert_eq!(linear_search(&arr, &5), Some(1));
        assert_eq!(linear_search(&arr, &6), None);
    }
}