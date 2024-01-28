/**
 * Least Recently Used (LRU) Cache
 * 
 * Least Recently Used (LRU) Cache is a type of cache in which we remove the least recently used entry when the cache memory reaches its limit.
 * 
 * For example, if a cache with a capacity to store 5 keys has the following state(arranged from most recently used key to least recently used key) -
 * 5 3 2 1 4
 * Now, if the next key comes as 1(which is a cache hit), then the cache state should change to -
 * 1 5 3 2 4
 * Now, if the next key comes as 6(which is a cache miss), then the cache state should change to -
 * 6 1 5 3 2
 * 
 * Algorithm
 * 
 * We can use a doubly linked list of capacity (size) maintained in sorted order of access time. The most recently accessed page will be at the front of the list and the least accessed will be at the end of the list. A Hash with page number as key and address of the corresponding queue node as value.
 * 
 * When a page is referenced, the required page may be in the memory. If it is in the memory, we need to detach the node of the list and bring it to the front of the queue.
 * 
 * If the required page is not in the memory, we bring that in memory. In simple words, we add a new node to the front of the queue and update the corresponding node address in the hash. If the queue is full, i.e. all the frames are full, we remove a node from the rear of the queue, and add the new node to the front of the queue.
 * 
 * In Rust we can use a HashMap and a LinkedList to implement this algorithm. The HashMap will store the key-value pairs and the LinkedList will store the keys in the order of most recently used to least recently used.
 * 
 * These handle some of the complexity of the actual implementation under the hood. For example, if there is a need for the capacity to change it will be handled by the HashMap and the LinkedList will handle the order of the keys. The HashMap will also handle the lookup of the key-value pairs.
 * 
 * Time Complexity: O(1) for both get and put
 * Space Complexity: O(capacity) where capacity is the capacity of the cache
 * 
 * @param capacity The capacity of the cache
 * @return A new instance of the LRUCache
 */
use std::collections::HashMap;
use std::collections::LinkedList;

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    list: LinkedList<i32>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            list: LinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.map.get(&key) {
            self.list.remove(&key);
            self.list.push_front(key);
            *value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.list.remove(&key);
        } else if self.list.len() == self.capacity {
            let last = self.list.pop_back().unwrap();
            self.map.remove(&last);
        }
        self.list.push_front(key);
        self.map.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}