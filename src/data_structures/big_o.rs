/**
 * Currently this is a roughed out file to be used as a reference
 * for Big O notation.
 *
 * Each of these will be placed into a separate file and with a test.
 * 
 */


/** O(log n) Logarithmic Time Complexity 
 * 
 * Example - Binary Search
 * Binary search is used to find the position of a target value
 * within a sorted array.
 */
pub fn binary_search(array: &[i32], target: i32) -> Result<usize, usize> {
    array.binary_search(&target)
}

/** O(n) Linear Time Complexity
 * 
 * Example - Basic Search Algorithm
 * For unordered data, the most basic search is walking an array.
 */
pub fn basic_search(array: &[i32], target: i32) -> bool {
    for n in array {
        if *n == target {
            return true;
        }
    }
    false
}

/** O(n logn) Example - Heap Sort
 * 
 * Heap sort is another divide and conquer algorithm that has a time complexity of O(n log n).
 */
pub fn heap_sort(array: &mut [i32]) {
    let len = array.len();
    // Build heap (rearrange array)
    for i in (0..len / 2).rev() {
        heapify(array, len, i);
    }
    // One by one extract an element from heap
    for i in (1..len).rev() {
        // Move current root to end
        array.swap(0, i);
        // call max heapify on the reduced heap
        heapify(array, i, 0);
    }
}

/**
 * To heapify a subtree rooted with node i which is an index in array[]
 * n is size of heap
 */
fn heapify(array: &mut [i32], len: usize, idx: usize) {
    let mut largest = idx; // Initialize largest as root
    let left = 2 * idx + 1; // left = 2*i + 1
    let right = 2 * idx + 2; // right = 2*i + 2

    // If left child is larger than root
    if left < len && array[left] > array[largest] {
        largest = left;
    }

    // If right child is larger than largest so far
    if right < len && array[right] > array[largest] {
        largest = right;
    }
}

/** O(n^2) Example - Bubble Sort
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

/** O(N^3) Cubic Time Complexity
 * 
 * Sum of Character Codes
*/

/** 
 * O(2^n) Exponential Time Complexity
 * 
 * Example - Fibonacci Sequence
 * Recursive calculation of Fibonacci sequence exhibits exponential time complexity.
 */
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/** 
 * O(n!) Factorial Time Complexity 
 * 
 * Example - Permutations
 * Generating all permutations of an array has a factorial time complexity.
 */
pub fn permutations(arr: &mut [i32], k: usize, result: &mut Vec<Vec<i32>>) { 
    if k == 1 {
        result.push(arr.to_vec());
    } else {
        for i in 0..k {
            permutations(arr, k - 1, result);
            if k % 2 == 0 {
                arr.swap(i, k - 1);
            } else {
                arr.swap(0, k - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_element() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(access_element(&array, 0), Some(1));
        assert_eq!(access_element(&array, 6), None);
    }

    #[test]
    fn test_binary_search() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&array, 5), Ok(4));
        assert_eq!(binary_search(&array, 6), Err(5));
    }

    #[test]
    fn test_basic_search() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(basic_search(&array, 5), true);
        assert_eq!(basic_search(&array, 6), false);
    }

    #[test]
    fn test_merge_sort() {
        let mut array = [1, 5, 2, 4, 3];
        merge_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heap_sort() {
        let mut array = [1, 5, 2, 4, 3];
        heap_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut array = [1, 5, 2, 4, 3];
        bubble_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_permutations() {
        let mut arr:   [i32; 3] = [1, 2, 3];
        let mut result = Vec::new();
        let len = arr.len();
        permutations(&mut arr, len, &mut result);
        assert_eq!(result.len(), 6);
    }
}   