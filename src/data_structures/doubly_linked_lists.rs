/** 
 * Doubly Linked Lists
 * 
 * Time Complexity: O(1) Constant Time Complexity for push_front, push_back, pop_front, pop_back, peek_front, peek_back
 * Space Complexity: O(n) Linear Space Complexity
 * 
 * A custom doubly linked list is a linear data structure, in which the elements are not stored at contiguous memory locations.
 * 
 * There is LinkedList built into the Rust standard library that is doubly linked. 
 * This is not that implementation which uses unsafe code and is optimized for performance.
 * This is an example implementation used for understanding the data structure.
 * If you build your own take time to view the source code for the standard library implementation.
 * (see https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html#51-60)
 * 
 * From the Rust Book: "It is almost always better to use Vec or VecDeque (ed: a ring buffer) because array-based containers are generally faster , more memory efficient and make better use of CPU cache."
 * (see https://doc.rust-lang.org/std/collections/linked_list/index.html#)
 */
pub struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Clone> Clone for DoublyLinkedList<T> {
    fn clone(&self) -> Self {
        DoublyLinkedList {
            head: self.head.clone(),
            tail: self.tail.clone(),
            size: self.size,
        }
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> Clone for Node<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Node {
            data: self.data.clone(),
            next: self.next.clone(),
            prev: self.prev.clone(),
        }
    }
}

impl<T> Node<T> {
    fn next(&self) -> Option<&Box<Node<T>>> {
        self.next.as_ref()
    }

    fn prev(&self) -> Option<&Box<Node<T>>> {
        self.prev.as_ref()
    }

    fn next_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        self.next.as_mut()
    }

    fn prev_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        self.prev.as_mut()
    }
    
    fn new(data: T) -> Self {
        Node { data, next: None, prev: None }
    }
    
}

impl<T> DoublyLinkedList<T> {
    
    pub fn new() -> Self {
        DoublyLinkedList { head: None, tail: None, size: 0 }
    }

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node {
            data,
            next: None, // Initially None, will be set below
            prev: None,
        });

        // Update the head to be the new node
        self.head = Some(new_node.next.take().unwrap());
        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));

        if self.head.is_none() {
            // The list is initially empty, so the new node becomes both head and tail.
            // We use Box::into_raw to get a raw pointer to the new node.
            let raw_new_node = Box::into_raw(new_node);

            // Since the list is empty, head and tail point to the same node.
            self.head = Some(unsafe { Box::from_raw(raw_new_node) });
            self.tail = Some(unsafe { Box::from_raw(raw_new_node) });
        } else {
            new_node.prev = self.tail.take();

            // We use Box::into_raw to avoid multiple ownership of the new node.
            let raw_new_node = Box::into_raw(new_node);

            // Update the old tail's next to point to the new node.
            unsafe {
                (*raw_new_node).prev.as_mut().unwrap().as_mut().next = Some(Box::from_raw(raw_new_node));
            }

            // Update the list's tail to the new node.
            self.tail = Some(unsafe { Box::from_raw(raw_new_node) });
        }

        self.size += 1;
    }
    
    /**
     * Remove the first element from the doubly linked list.
     */
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    /**
     * Remove the last element from the doubly linked list.
     */
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|node| {
            self.tail = node.prev;
            self.size -= 1;
            node.data
        })
    }

    /**
     * Get the size of the doubly linked list.
     */
    pub fn size(&self) -> usize {
        self.size
    }

    
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    
    pub fn peek_back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &node.data)
    }
    
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }
    
    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        self.tail.as_mut().map(|node| &mut node.data)
    }
    
    /**
     * Get an iterator over the doubly linked list.
     */
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
    
    /**
     * Get a mutable iterator over the doubly linked list.
     */
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
    
    /**
     * Get an iterator over the doubly linked list in reverse.
     */
    pub fn iter_rev(&self) -> IterRev<T> {
        IterRev { next: self.tail.as_ref().map(|node| &**node) }
    }
    
    /**
     * Get a mutable iterator over the doubly linked list in reverse.
     */
    pub fn iter_rev_mut(&mut self) -> IterRevMut<T> {
        IterRevMut { next: self.tail.as_mut().map(|node| &mut **node) }
    }
    
    /**
     * Get an iterator over the doubly linked list.
     */
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    
    /**
     * Get an iterator over the doubly linked list in reverse.
     */
    pub fn into_iter_rev(self) -> IntoIterRev<T> {
        IntoIterRev(self)
    }
}

/**
 * An iterator over the doubly linked list.
 */
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

/**
 * An iterator over the doubly linked list.
 */
pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

/**
 * An iterator over the doubly linked list in reverse.
 */
pub struct IterRev<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

/**
 * An iterator over the doubly linked list in reverse.
 */
pub struct IterRevMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

/**
 * An iterator over the doubly linked list.
 */ 
pub struct IntoIter<T>(DoublyLinkedList<T>);

/**
 * An iterator over the doubly linked list in reverse.
 */
pub struct IntoIterRev<T>(DoublyLinkedList<T>);


// Forward Iterators
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next().map(|node| &**node);
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next_mut().map(|next_node| &mut **next_node);
            &mut node.data
        })
    }
}

// Reverse Iterators
impl<'a, T> Iterator for IterRev<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.prev().map(|node| &**node);
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterRevMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.prev_mut().map(|prev_node| &mut **prev_node);
            &mut node.data
        })
    }
}


// Consuming Iterators
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> Iterator for IntoIterRev<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_doubly_linked_list() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);
        assert_eq!(list.peek_front_mut(), None);
        assert_eq!(list.peek_back_mut(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.iter().next(), None);
        assert_eq!(list.iter_mut().next(), None);
        assert_eq!(list.iter_rev().next(), None);
        assert_eq!(list.iter_rev_mut().next(), None);
        assert_eq!(list.into_iter().next(), None);
        assert_eq!(list.into_iter_rev().next(), None);

        list.push_front(1);
        assert_eq!(list.size(), 1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));
        assert_eq!(list.peek_front_mut(), Some(&mut 1));
        assert_eq!(list.peek_back_mut(), Some(&mut 1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.iter().next(), None);
        assert_eq!(list.iter_mut().next(), None);
        assert_eq!(list.iter_rev().next(), None);
        assert_eq!(list.iter_rev_mut().next(), None);
        assert_eq!(list.into_iter().next(), None);
        assert_eq!(list.into_iter_rev().next(), None);

        list.push_back(2);
        assert_eq!(list.size(), 1);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&2));
        assert_eq!(list.peek_front_mut(), Some(&mut 2));
        assert_eq!(list.peek_back_mut(), Some(&mut 2));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.iter().next(), None);
        assert_eq!(list.iter_mut().next(), None);
        assert_eq!(list.iter_rev().next(), None);
        assert_eq!(list.iter_rev_mut().next(), None);
        assert_eq!(list.into_iter().next(), None);
        assert_eq!(
            list.into_iter_rev().next(),
            None
        );  
        
        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        list.push_back(4);
        assert_eq!(list.size(), 4);
        assert_eq!(list.peek_front(), Some(&3));
        assert_eq!(list.peek_back(), Some(&4));
        assert_eq!(list.peek_front_mut(), Some(&mut 3));
        assert_eq!(list.peek_back_mut(), Some(&mut 4));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.iter().cloned().collect::<Vec<_>>(), [1, 2]);
        assert_eq!(list.iter_mut().next(), Some(&mut 1));
        assert_eq!(list.iter_rev().cloned().collect::<Vec<_>>(), [2, 1]);
        assert_eq!(list.iter_rev_mut().next(), Some(&mut 2));
        assert_eq!(list.into_iter().collect::<Vec<_>>(), [1, 2]);
        assert_eq!(list.into_iter_rev().collect::<Vec<_>>(), [2, 1]);
    }
}