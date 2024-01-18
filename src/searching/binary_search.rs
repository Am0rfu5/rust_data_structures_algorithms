/** O(log n) Logarithmic Time Complexity 
 * 
* Binary search is used to find the position of a target value within a sorted array.
*/
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if arr[mid] > *target {
            high = mid - 1;
        } else if arr[mid] < *target {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

/**
 * Example2 - Using the Binary Search method from the standard library
 */
pub fn binary_search2(array: &[i32], target: i32) -> Result<usize, usize> {
    array.binary_search(&target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &5), Some(4));
        assert_eq!(binary_search(&arr, &6), None);
    }
}