/** Bubble Sort Algorithm Example 
 * 
 * Time Complexity: O(n^2)
 * Memory Complexity: O(1)
 *  
 * Bubble sort is a simple sorting algorithm with a time complexity of O(n^2).
 */
pub fn bubble_sort(array: &mut [i32]) {
    let len = array.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [1, 5, 2, 4, 3];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}