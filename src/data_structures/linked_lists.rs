/**
 * Linked Lists
 * 
 * Time Complexity: O(1) Constant Time Complexity
 * Space Complexity: O(n) Linear Space Complexity
 * 
 * A linked list is a linear data structure, in which the elements are not stored at contiguous memory locations.
 */
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /**
     * Create a new linked list.
     */
    pub fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    /**
     * Add a new element to the front of the linked list.
     */
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    /**
     * Remove the first element from the linked list.
     */
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    /**
     * Get the size of the linked list.
     */
    pub fn size(&self) -> usize {
        self.size
    }

    /**
     * Get the first element from the linked list.
     */
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /**
     * Get the first element from the linked list.
     */
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    /**
     * Get an iterator over the linked list.
     */
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

/**
 * An iterator over the linked list.
 */
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // Take the next node, leaving a None in its place.
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.size(), 3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.size(), 0);
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        assert_eq!(list.pop(), None);
    }
}