/** 
 * Quick Sort Algorithm Example
 * 
 * Quick sort is a divide and conquer algorithm that has a time complexity that in the worst case scenario is O(n^2) and memory complexity O(log n)
 * 
 * It is not considered a stable sort.
 * 
 * It is a recursive algorithm that works by partitioning an array into two halves, and then recursively sorting the two halves independently.
 */
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let pivot = partition(arr);
    // println!("Pivot: {}", pivot);
    // println!("Left Before: {:?}", &arr[0..pivot]);
    quick_sort(&mut arr[0..pivot]);
    // println!("Left After: {:?}", &arr[0..pivot]);
    // println!("Right Before: {:?}", &arr[pivot + 1..len]);
    quick_sort(&mut arr[pivot + 1..len]);
    // println!("Right: {:?}", &arr[pivot + 1..len]);
}

/**
 * Partition the array into two halves.
 * 
 * The pivot is the last element in the array.
 * 
 * The left half contains all elements less than or equal to the pivot.
 * 
 * The right half contains all elements greater than the pivot.
 */
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = len / 2;
    arr.swap(pivot, len - 1);
    let mut store = 0;
    for i in 0..len - 1 {
        if arr[i] <= arr[len - 1] {
            arr.swap(i, store);
            store += 1;
        }
    }
    arr.swap(store, len - 1);
    store
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        // test with an odd number of elements
        // let mut arr = [2,3,1];
        
        // test with an even number of elements
        // let mut arr = [2,3,1,4];
    
        // test a larger number of elements
        let mut arr = [1, 5, 15, 4, 3, 9, 12, 6, 8, 7, 10, 11, 13, 2, 14, 16    ];
        println!("Before: {:?}", arr);
        quick_sort(&mut arr);
        println!("After: {:?}", arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }
}