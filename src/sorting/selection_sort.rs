// Selection Sort example
// Path: src/sorting/selection_sort.rs
use std::cmp::Ordering;

/** Selection Sort Algorithm Example 
 * 
 * Time Complexity: O(n^2)
 * Memory Complexity: O(1)
 *  
 * Selection sort is a simple sorting algorithm with a time complexity of O(n^2).
 */
pub fn selection_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    for i in 0..len {
        let mut min = i;
        for j in i + 1..len {
            if array[j] < array[min] {
                min = j;
            }
        }
        if min != i {
            array.swap(i, min);
        }
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [1, 5, 2, 4, 3];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}