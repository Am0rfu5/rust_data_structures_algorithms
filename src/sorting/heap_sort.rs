use std::collections::BinaryHeap;

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