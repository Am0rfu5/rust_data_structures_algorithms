# Algorithms

!!!NOTE This is entire repo is still in early development. These categorization and list items may or may not be included.

## Resources

There is a great visualization at [VisuAlgo](https://visualgo.net/en) that shows how each of these data structures work.  I highly recommend checking it out.

## Jargon

- Turtles: Small values near the end of the list.
- Rabbits: Large values around the beginning of the list.
- Pigeonholes: Buckets used in pigeonhole sort.
- Bitonic Sequence: A sequence that increases and then decreases, or vice versa.

## Sorting

- **Bubble Sort**: A simple sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. It's easy to understand but inefficient for large datasets.
- **Selection Sort**: Works by repeatedly finding the minimum element from the unsorted part of the array and putting it at the beginning. Simple, but not suitable for large lists.
- **Insertion Sort**: Builds the final sorted array one item at a time, inserting each element into its correct position. Efficient for small and mostly-sorted datasets.
- **Merge Sort**: A divide and conquer algorithm that divides the input array into two halves, recursively sorts them, and then merges the sorted halves. Efficient and stable but requires extra space.
- **Quick Sort**: An efficient, divide-and-conquer, and in-place sorting algorithm that works by selecting a 'pivot' element and partitioning the other elements into two sub-arrays according to whether they are less than or greater than the pivot.
- **Heap Sort**: Converts the array into a heap, then repeatedly extracts the maximum element from the heap and rebuilds the heap until the array is sorted. Efficient and works in-place but not stable.
- **Radix Sort**: A non-comparative integer sorting algorithm that sorts data with integer keys by grouping the keys by individual digits that share the same significant position and value.
- **Counting Sort**: An integer sorting algorithm that operates by counting the number of objects that have distinct key values, then doing some arithmetic to calculate the position of each object in the output sequence.
- **Bucket Sort**: Distributes elements into a number of buckets, sorts these buckets individually, and then concatenates them. Effective for uniformly distributed data.
- **Shell Sort**: An in-place comparison sort which generalizes insertion sort by allowing the exchange of far-off elements, then progressively reducing the gap between elements to be compared.
- **Cocktail Sort**: A variation of bubble sort that sorts in both directions on each pass through the list, reducing turtles (small values near the end of the list) in early stages.
- **Comb Sort**: An improvement on bubble sort, that eliminates turtles by using a larger gap starting with the total list length, then shrinking the gap until it becomes 1 like in bubble sort.
- **Gnome Sort**: Similar to insertion sort but moves elements to their proper place by a series of swaps, like a gnome sorting his flower pots.
- **Odd-Even Sort**: A variation of bubble sort for parallel processing environments; it performs a sequence of odd-even transpositions which is similar to bubble sort.
- **Cycle Sort**: An in-place and unstable sorting algorithm theoretically optimal in terms of total number of writes to the original array, making it useful for write-heavy memory.
- **Stooge Sort**: A recursive sorting algorithm with a very poor time complexity, primarily of theoretical interest and often used as an example of a highly inefficient algorithm.
- **Pigeonhole Sort**: Also known as bucket sort or counting sort, pigeonhole sort is used when the range of key values is small; it works by assigning elements to 'pigeonholes' based on their key values and then listing the elements of non-empty pigeonholes in order.
- **Bitonic Sort**: A parallel algorithm for sorting, it works by recursively sorting bitonic sequences, which are sequences that increase and then decrease, or vice versa.
- **Pancake Sort**: Involves flipping sub-arrays of a given array to sort the values, similar to sorting pancakes in size order with a spatula.
- **Bogo Sort**: A highly inefficient sorting algorithm based on the generate and test paradigm; it randomly permutes its input until it finds a sorted sequence.
- **Sleep Sort**: An unconventional sorting algorithm that creates a new thread for each item to be sorted, where each thread sleeps for an interval corresponding to the item's value, then emits the item.
- **Bead Sort**: Also known as gravity sort, this natural sorting algorithm both distributes beads into columns and then counts the beads in each colum.
- **Binary Tree Sort**: Builds a binary search tree from the keys of the input elements and then traverses the tree in-order to produce a sorted output.
- **Batcher's Odd-Even Mergesort**: A merge sort variant that sorts the array by recursively dividing it into smaller arrays, then sorting and merging them using an odd-even merge.
- **Smoothsort**: A comparison-based sorting algorithm that combines aspects of heap sort and insertion sort to sort the array while maintaining a heap within the array.
- **Tournament Sort**: Builds a tournament tree (a form of min/max heap) to sort the elements. It's similar to heap sort but uses a binary tree structure.
- **Cocktail Shaker Sort**: Also known as bidirectional bubble sort, it's a variation of bubble sort that sorts in both directions in each pass through the list.
- **Strand Sort**: Works by repeatedly pulling sorted sublists out of the list to be sorted and merging them with the resulting

Below is a table of sorting algorithms with their details. For some algorithms, there are no direct equivalents in the Rust standard library or commonly used crates.

| Name | Rust Implementation | Common Use Cases | Big O Worst Case Time Complexity | Big O Worst Case Memory Complexity |
|------|---------------------|------------------|----------------------------------|------------------------------------|
| Bubble Sort | [Custom Example](./src/sorting/bubble_sort.rs) | Small datasets, educational purposes | O(n²) | O(1) |
| Selection Sort | Custom implementation | Small datasets, educational purposes | O(n²) | O(1) |
| Insertion Sort | Custom implementation | Small or mostly sorted datasets | O(n²) | O(1) |
| Merge Sort | `std::vec::Vec::sort` | General purpose | O(n log n) | O(n) |
| Quick Sort | `std::vec::Vec::sort_unstable` | General purpose | O(n²) | O(log n) |
| Heap Sort | `std::collections::BinaryHeap` | General purpose | O(n log n) | O(n) |
| Radix Sort | `radix_sort` crate | Sorting integers | O(nk) | O(n + k) |
| Counting Sort | Custom implementation | Integer sorting with small range | O(n + k) | O(n + k) |
| Bucket Sort | Custom implementation | Uniformly distributed data | O(n²) | O(n) |
| Shell Sort | Custom implementation | General purpose | O(n(log n)²) | O(1) |
| Cocktail Sort | Custom implementation | Small datasets, educational purposes | O(n²) | O(1) |
| Comb Sort | Custom implementation | Improvement over bubble sort | O(n²) | O(1) |
| Gnome Sort | Custom implementation | Small datasets, similar to insertion sort | O(n²) | O(1) |
| Odd-Even Sort | Custom implementation | Parallel processing environments | O(n²) | O(1) |
| Cycle Sort | Custom implementation | Minimizing writes to memory | O(n²) | O(1) |
| Stooge Sort | Custom implementation | Educational, example of inefficiency | O(n^(log 3/log 1.5)) | O(n) |
| Pigeonhole Sort | Custom implementation | Small range of key values | O(n + k) | O(n + k) |
| Bitonic Sort | Custom implementation | Parallel sorting | O(log²n) | O(n log n) |
| Pancake Sort | Custom implementation | Educational, novelty use | O(n²) | O(1) |
| Bogo Sort | Custom implementation | Educational, example of inefficiency | O((n+1)!) | O(1) |
| Sleep Sort | Custom implementation | Novelty use, unconventional | Unpredictable | O(n) |
| Bead Sort | Custom implementation | Positive integers, novelty | O(n²) | O(n²) |
| Binary Tree Sort | `std::collections::BTreeMap` (indirectly) | General purpose | O(n log n) | O(n) |
| Batcher's Odd-Even Mergesort | Custom implementation | Parallel sorting | O(log²n) | O(n) |
| Smoothsort | Custom implementation | Variation of heapsort | O(n log n) | O(1) |
| Tournament Sort | Custom implementation | Heap sort variation | O(n log n) | O(n) |
| Cocktail Shaker Sort | Custom implementation | Variation of bubble sort | O(n²) | O(1) |
| Strand Sort | Custom implementation | Sorting linked lists | O(n²) | O(1) |

Note: The Big O complexities are for the worst-case scenarios. Some algorithms may have better average-case complexities. Memory complexity often refers to additional memory needed besides the input data. For sorting small arrays or lists, simpler algorithms like Insertion Sort or Bubble Sort might be more efficient due to their lower overhead despite higher theoretical complexities. In contrast, algorithms like Quick Sort and Merge Sort are better suited for larger datasets.

## Searching

- **Linear Search**: A simple search algorithm that checks every element in a list or array until it finds the target value. It's straightforward but inefficient for large datasets.
- **Binary Search**: An efficient algorithm for finding an item in a sorted array by repeatedly dividing the search interval in half. It's much faster than linear search for large datasets.
- **Jump Search**: An algorithm for sorted arrays that jumps ahead a fixed number of steps and then performs a linear search backwards. It's a balance between linear and binary search, offering better performance than linear search on sorted arrays.
- **Interpolation Search**: Similar to binary search but estimates the position of the target value using the values of the endpoints. It's particularly efficient for uniformly distributed datasets.
- **Exponential Search**: Starts with a subarray of size 1 and exponentially grows the size until the boundary of the target is found, then uses binary search within this boundary. It's useful for unbounded or infinite arrays.
- **Ternary Search**: Divides the array into three parts and discards one or two parts based on comparisons, similar to binary search but with three sections. It can be slightly more efficient than binary search in some cases.
- **Sublist Search**: A search algorithm to find a sublist within another list. It checks if the elements of one list appear in the same order in another list.
- **Recursive Fibonacci Search**: A variant of binary search that uses Fibonacci numbers to divide the array and can be implemented recursively.
- **Unbounded Binary Search**: Useful for searching in a sorted array of unknown or infinite size, typically starting with a bounded search and expanding as needed.
- **Interpolation Search with duplicates**: Similar to binary search but calculates the probable position of the searched value. Efficiency can decrease when the array contains duplicate elements.

### Recursive Searching

- **Recursive Binary Search**: A binary search algorithm implemented using recursion, dividing the search interval in half each time.
- **Recursive Ternary Search**: Divides the array into three parts and discards one or two parts each step, based on comparisons. It's a recursive implementation of ternary search.
- **Recursive Fibonacci Search**: Implements Fibonacci search recursively. It divides the array at positions calculated based on Fibonacci numbers.
- **Recursive Interpolation Search**: An implementation of interpolation search using recursion. It tries to guess the position of the searched value based on the values at the array's endpoints.
- **Recursive Exponential Search**: Useful for unbounded searches. It first finds a range where the element might be by growing the bounds exponentially, then uses binary search within that range.
- **Recursive Sublist Search**: Generally involves using a recursive approach to find a sublist within another list, though not a standard specific algorithm.
- **Recursive Unbounded Binary Search**: An unbounded binary search implemented recursively, suitable for infinite or unknown size arrays, where it first finds a bound and then applies binary search.
- **Recursive Interpolation Search with duplicates**: A recursive version of interpolation search that can be less efficient in the presence of duplicate elements in the array.
  
## Graphs

- Breadth First Search
- Depth First Search
- Dijkstra's Algorithm
- Bellman-Ford Algorithm
- Floyd-Warshall Algorithm
- Prim's Algorithm
- Kruskal's Algorithm
- Topological Sort
- Kahn's Algorithm
- Edmonds-Karp Algorithm
- Ford-Fulkerson Algorithm
- Hopcroft-Karp Algorithm
- Dinic's Algorithm
- Johnson's Algorithm
- Kosaraju's Algorithm
- Tarjan's Algorithm
- Hierholzer's Algorithm
- Fleury's Algorithm
- Gale-Shapley Algorithm
- Hungarian Algorithm
- Bron-Kerbosch Algorithm
- D* Algorithm
- A* Algorithm
- IDA* Algorithm
- Best-First Search
- Bidirectional Search
- Uniform Cost Search
- Iterative Deepening Depth-First Search
- Bidirectional Iterative Deepening Depth-First Search
- Iterative Deepening A* Search

## Compression

- Huffman Coding
- Shannon-Fano Coding
- Arithmetic Coding
- Lempel-Ziv-Welch Compression
- Lempel-Ziv-Storer-Szymanski Compression
- Lempel-Ziv-Markov Chain Compression
- Lempel-Ziv-Oberhumer Compression
- Lempel-Ziv-Welch Compression
- Lempel-Ziv-Storer-Szymanski Compression

## Cryptography

- Caesar Cipher
- Vigenère Cipher
- One-Time Pad Cipher
- RSA Cipher
- Diffie-Hellman Key Exchange
- ElGamal Cryptosystem
- Digital Signature Algorithm
- Merkle-Hellman Knapsack Cryptosystem
- Rabin Cryptosystem
- Blum Blum Shub Generator
- Rivest Shamir Adleman Cryptosystem
- Paillier Cryptosystem
- Goldwasser-Micali Cryptosystem
- McEliece Cryptosystem
- NTRUEncrypt Cryptosystem