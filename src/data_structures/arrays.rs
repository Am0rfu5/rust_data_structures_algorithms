/** O(1) Constant Time Complexity
 *  
 * Example - Accessing an Array Element
 * Accessing an element at a specific index in an array is a
 * constant time operation.
 */
pub fn access_array_element(array: &[i32], index: usize) -> Option<i32> {
    array.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_array_element() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(access_array_element(&array, 0), Some(1));
        assert_eq!(access_array_element(&array, 6), None);
    }
}