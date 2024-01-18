use std::collections::BinaryHeap;

/**
 * Heap Sort
 * 
 * Time Complexity: O(n log n) Linearithmic Time Complexity
 * Space Complexity: O(1) Constant Space Complexity
 * 
 * Heap sort is a comparison based sorting technique based on Binary Heap data structure. 
 * It is similar to selection sort where we first find the maximum element and place the
 * maximum element at the end. We repeat the same process for remaining element.
 * 
 * Heap sort is an in-place algorithm. 
 * It does not require any extra space (space complexity O(1)
 */
pub fn heap_sort<T: Ord>(mut vec: Vec<T>) -> Vec<T> {
    // Create a new max heap and add all the elements from the vector to it.
    let mut heap = BinaryHeap::new();
    for item in vec.drain(..) {
        heap.push(item);
    }

    // Now, retrieve the elements in sorted order.
    let mut sorted_vec = Vec::with_capacity(heap.len());
    while let Some(item) = heap.pop() {
        // As it's a max heap, we push each element at the front to get the elements in ascending order.
        sorted_vec.insert(0, item);
    }

    sorted_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = [1, 5, 2, 4, 3];
        let sorted_arr = heap_sort(arr.to_vec());
        arr.sort();
        assert_eq!(sorted_arr, arr);
    }
}