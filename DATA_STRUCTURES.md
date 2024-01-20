# Data Structures

For Rust specific visualization of the metadata for heap containers (e.g. Box, Vec, Rc, Arc, Cell) checkout  https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p

## Overview

Eventually we expect to add links. For now, visit the [src](src/data_structures/) directory to see the progress on various data structures, including:

### Linear Structures
- **Array**: Stores elements in a contiguous block of memory.
- **Dynamic Array (ArrayList)**: Similar to an array but allows resizable storage.
- **Linked List**: Consists of nodes with data and references to the next node in the sequence.
- **Doubly Linked List**: Nodes have references to both the next and the previous node.
- **Circular Buffer (Circular Queue, Ring Buffer)**: A linear structure using a single, fixed-size buffer as if it were connected end-to-end.

### Stack and Queue Structures
- **Stack**: A LIFO (Last In, First Out) data structure.
- **Queue**: A FIFO (First In, First Out) data structure.
- **Priority Queue**: Elements are removed based on priority, not just insertion order.

### Tree and Heap Structures
- **Binary Tree**: A tree where each node has at most two children.
- **Binary Search Tree (BST)**: A binary tree maintaining order among elements for efficient search.
- **Balanced Trees**: Including AVL, Red-Black, and Splay trees.
- **Heap**: A tree-based structure satisfying the heap property.
- **B+ Tree**: A type of tree that maintains sorted data and allows searches, sequential access, insertions, and deletions in logarithmic time.

### Associative Structures
- **Hash Table**: Maps keys to values, implementing an associative array.
- **Set**: A collection of distinct objects.
- **Map/Dictionary**: A collection of key-value pairs.

### Graph Structures
- **Graph**: Consists of nodes (vertices) and edges connecting pairs of nodes.
- **Graph Variants**: Includes directed, undirected, and weighted graphs.

### Advanced and Specialized Structures
- **Trie**: A tree-like structure for efficient retrieval of keys in a dataset of strings.
- **Skip List**: A probabilistic structure allowing fast search, insertion, and deletion.
- **Union-Find (Disjoint Set)**: Keeps track of elements partitioned into disjoint subsets.

For Rust implementations the standard library types are referenced where applicable, and for more specialized structures a crate may be referenced. In many cases a custom implementation will be required but these offer a good starting point. In some cases the same types are referenced by multiple structures. For example, a `LinkedList` can be used as a Singly or a Doubly Linked List and a VecDeque can be used as a Queue or a Circular Buffer. In many cases these are built on top of underlying types, particularly Vec.

| Name                    | Rust Implementation           | Common Use Cases                         | Big O Worst Case Time Complexity | Big O Worst Case Memory Complexity |
|-------------------------|-------------------------------|------------------------------------------|----------------------------------|------------------------------------|
| Array                   | `[T; N]` or `Vec<T>`          | Storing a fixed/variable number of elements | Access: O(1), Insert/Delete: O(n) | O(n) |
| Linked List             | `std::collections::LinkedList` | Sequences with frequent insertions and deletions | Access/Insert/Delete: O(n)       | O(n) |
| Doubly Linked List      | `std::collections::LinkedList` | Sequences with frequent insertions and deletions at both ends | Access/Insert/Delete: O(n)       | O(n) |
| Circular Buffer         | `std::collections::VecDeque`  | Fixed-size buffer with wrap-around semantics | Access: O(1), Insert/Delete: O(1) | O(n) |
| Stack                   | `Vec<T>`                      | LIFO operations, undo mechanisms, parsing | Push/Pop: O(1)                    | O(n) |
| Queue                   | `std::collections::VecDeque`  | FIFO operations, task scheduling         | Enqueue/Dequeue: O(1)             | O(n) |
| Priority Queue          | `std::collections::BinaryHeap` | Task scheduling with priorities           | Insert/Delete: O(log n)           | O(n) |
| Binary Tree             | Custom implementation         | Hierarchical data, sorted data            | Access/Insert/Delete: O(n)        | O(n) |
| Binary Search Tree      | Custom implementation         | Sorted data, dictionaries, priority queues | Balanced: O(log n), Unbalanced: O(n) | O(n) |
| B+ Tree                 | Custom implementation         | Databases, filesystems                     | Search/Insert/Delete: O(log n)    | O(n) |
| Heap                    | `std::collections::BinaryHeap` | Priority queues, heap sort                | Insert/Delete: O(log n)           | O(n) |
| Hash Table              | `std::collections::HashMap`   | Fast lookups, unique item storage         | Average: O(1), Worst: O(n)        | O(n) |
| Graph                   | Custom implementation         | Network routing, social networks          | Depends on representation         | O(n + e) |
| Trie                    | Custom implementation         | Autocomplete, spell checkers              | O(L) (L = key length)             | O(n) |
| Skip List               | Custom implementation         | Probabilistic data structures, fast search | Search/Insert/Delete: O(log n)    | O(n log n) |
| Union-Find              | Custom implementation         | Disjoint-set operations                    | Union/Find: O(log n)              | O(n) |

Note: The Rust implementations column mentions types from the Rust standard library where possible. For data structures not directly available, custom implementations are typically used, which can vary in complexity and performance characteristics. The time and memory complexities are approximate and can vary based on specific implementations and use cases.

Each of these structures is chosen based on specific requirements such as access speed, memory efficiency, and the nature of the operations performed.

## Big O Notation

Each data structure has its own worst-case scenarios for runtime and memory usage, scaling with the size of the input data. Below is a table of data structures with their Big O runtimes and memory usage, ordered from fastest to slowest for runtime.

| Data Structure        | Big O (worst case runtime) | Memory Usage (worst case) |
|-----------------------|----------------------------|---------------------------|
| Array                 | Access: O(1), Insert/Delete: O(n) | O(n)                    |
| Linked List           | Access/Insert/Delete: O(n) | O(n)                    |
| Stack                 | O(1) for push/pop         | O(n)                    |
| Queue                 | O(1) for enqueue/dequeue  | O(n)                    |
| Hash Table            | Average: O(1), Worst: O(n) | O(n)                   |
| Binary Search Tree    | Balanced: O(log n), Unbalanced: O(n) | O(n)           |
| B+ Tree               | O(log n)                  | O(n)                    |
| Heap                  | Insert/Delete: O(log n), Search: O(n) | O(n)           |
| Graph                 | Depends on representation (e.g., O(n^2) for adjacency matrix) | O(n + e) where e is the number of edges |
| Trie                  | O(L) (L = key length)     | O(n) (varies based on key structure and size) |

Note: The complexity for Graphs varies greatly depending on the representation and the specific operation. The memory usage for Tries can be significant due to their structure, especially with a large number of long keys. For Hash Tables, the worst-case scenario occurs with hash collisions. The complexities listed for Binary Search Trees are for the basic operations like search, insert, and delete.