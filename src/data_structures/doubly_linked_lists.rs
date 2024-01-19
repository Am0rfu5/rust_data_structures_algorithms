/** 
 * Doubly Linked Lists
 * 
 * Time Complexity: O(1) Constant Time Complexity
 * Space Complexity: O(n) Linear Space Complexity
 * 
 * A doubly linked list is a linear data structure, in which the elements are not stored at contiguous memory locations.
 * 
 * There is LinkedList built into the Rust standard library, but it is not doubly linked.
 */
pub struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> DoublyLinkedList<T> {
    // ... (other methods remain the same) ...

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node {
            data,
            next: self.head.take(),
            prev: None,
        });
        match self.head.take() {
            Some(old_head) => {
                old_head.prev = Some(new_node.clone());
                new_node.next = Some(old_head);
                self.head = Some(new_node);
            },
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let mut new_node = Box::new(Node {
            data,
            next: None,
            prev: self.tail.take(),
        });
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.next = Some(new_node.clone());
                new_node.prev = Some(old_tail);
                self.tail = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
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
pub struct IntoIter<T>(doublyLinkedList<T>);

/**
 * An iterator over the doubly linked list in reverse.
 */
pub struct IntoIterRev<T>(doublyLinkedList<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.data
        })
    }
}

impl<'a, T> Iterator for IterRev<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.prev.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterRevMut<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.prev.as_mut().map(|node| &mut **node);
            &mut node.data
        })
    }
}

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
        let mut list = doublyLinkedList::new();
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