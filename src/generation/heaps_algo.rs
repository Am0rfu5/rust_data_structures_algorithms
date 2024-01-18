/**
 * Heap's Algorithm
 *  
 * Time Complexity: O(n!) Factorial Time Complexity
 * Memory Complexity: O(n) Linear Space Complexity
 * 
 * Generating all permutations of an array has a factorial time complexity.
 * 
 * Heap's algorithm is used to generate all permutations of n objects. It was first 
 * proposed by B. R. Heap in 1963. It generates each permutation from the previous one
 * by choosing a pair of elements to interchange.
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