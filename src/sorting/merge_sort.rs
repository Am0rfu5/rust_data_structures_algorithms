/**
 * Merge Sort Algorithm Example
 * 
 * Merge sort is a sorting algorithm that uses the divide and conquer paradigm.
 * 
 * Merge sort is a divide and conquer algorithm that was invented by John von Neumann in 1945.
 * 
 * Merge sort has a time complexity of O(n log n).
 * 
 */
pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

/**
 * Merge two sorted arrays into one sorted array.
 * 
 * In Rust we utilize the Generic Trait Bound syntax to specify that the type T must implement the Ord and Copy traits.
 */
fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    let mut left = Vec::with_capacity(mid);
    let mut right = Vec::with_capacity(arr.len() - mid);
    left.extend_from_slice(&arr[..mid]);
    right.extend_from_slice(&arr[mid..]);
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [1, 5, 2, 4, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}