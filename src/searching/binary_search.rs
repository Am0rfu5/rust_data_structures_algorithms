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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_works() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &5), Some(4));
        assert_eq!(binary_search(&arr, &6), None);
    }
}