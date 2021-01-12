use std::iter::FromIterator;
use std::mem;

pub struct SimpleLinkedList<T> {
    length: usize,
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0,
        }
    }

    /// # Examples
    ///
    /// ```
    /// use simple_linked_list::SimpleLinkedList;
    /// let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    ///
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// # Examples
    ///
    /// ```
    /// use simple_linked_list::SimpleLinkedList;
    /// let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    /// list.push(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    /// # Examples
    ///
    /// ```
    /// use simple_linked_list::SimpleLinkedList;
    /// let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    /// list.push(9);
    /// list.push(42);
    /// ```
    pub fn push(&mut self, element: T) {
        let node = Node {
            elem: element,
            next: mem::replace(&mut self.head, None),
        };

        self.head = Some(Box::new(node));
        self.length += 1;
    }

    /// # Examples
    ///
    /// ```
    /// use simple_linked_list::SimpleLinkedList;
    /// let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    /// list.push(42);
    /// assert_eq!(list.pop(), Some(42));
    /// assert_eq!(list.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        // std::mem::take takes the value out of the Option leaving a None in
        // it's place. It's basically shorthand for the std::mem::replace
        // seen in `#push`
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.elem
        })
    }

    /// # Examples
    ///
    /// ```
    /// use simple_linked_list::SimpleLinkedList;
    /// let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    /// list.push(42);
    /// assert_eq!(list.peek(), Some(&42));
    /// assert_eq!(list.peek(), Some(&42));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        // `as_ref`: Converts from &Option<T> to Option<&T>. We need to leave
        // head where it is at and so we need a ref to the Node (not a ref to
        // the Option). See this for example:
        // https://doc.rust-lang.org/std/option/enum.Option.html#examples-3
        //
        // `map`: Convert Option<T> to Option<U> by applying a function to a
        // contained value. See:
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.map
        self.head.as_ref().map(|node| &node.elem)
    }

    /// # Examples
    ///
    /// ```
    /// use simple_linked_list::SimpleLinkedList;
    /// let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// let mut reversed = list.rev();
    /// assert_eq!(reversed.pop(), Some(1));
    /// assert_eq!(reversed.pop(), Some(2));
    /// assert_eq!(reversed.pop(), None);
    /// ```
    pub fn rev(self) -> SimpleLinkedList<T> {
        // How does this work?
        // 1. By implementing the Into trait for SimpleLinkedList, we define how
        //    a list should be converted into a vector
        // https://doc.rust-lang.org/rust-by-example/conversion/from_into.html#into
        let mut v: Vec<_> = self.into();

        // 2. Once we have the vector, create a draining iterator (which will
        //    drain the entire vector of all elements) and then call `collect`
        //    which, because we implemented FromIterator for SimpleLinkedList,
        //    will convert the list into the "relevant" type (a linked list)
        // https://doc.rust-lang.org/beta/core/iter/trait.Iterator.html#method.collect
        return v.drain(..).rev().collect();
    }
}

// By implementing FromIterator for a type, you define how it will be created
// from an iterator.
// https://doc.rust-lang.org/beta/core/iter/trait.FromIterator.html#examples
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for node in iter {
            list.push(node);
        }

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();

        while let Some(node) = self.pop() {
            v.push(node);
        }

        v.reverse();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty_on_creation() {
        let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        assert!(list.is_empty(), "List wasn't empty on creation");
    }

    #[test]
    fn test_is_not_empty_after_push() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        assert!(!list.is_empty())
    }

    #[test]
    fn test_is_empty_after_drain() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        list.pop();
        assert!(list.is_empty())
    }

    #[test]
    fn test_push_many_increments_length() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        assert_eq!(list.len(), 0);

        list.push(1);
        assert_eq!(list.len(), 1);

        list.push(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_pop_on_empty_list() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        let node = list.pop();
        assert_eq!(None, node);
    }

    #[test]
    fn test_pop_decrements_length() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        list.push(2);

        list.pop();
        assert_eq!(list.len(), 1);

        list.pop();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_peek_returns_reference_to_head_element_but_does_not_remove_it() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn test_push_pop_peek_len_transitions() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.peek(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_from_slice() {
        let mut array = vec!["1", "2", "3", "4"];
        let mut list: SimpleLinkedList<_> = array.drain(..).collect();
        assert_eq!(list.pop(), Some("4"));
        assert_eq!(list.pop(), Some("3"));
        assert_eq!(list.pop(), Some("2"));
        assert_eq!(list.pop(), Some("1"));
    }

    #[test]
    fn test_reverse() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut rev_list = list.rev();
        assert_eq!(rev_list.pop(), Some(1));
        assert_eq!(rev_list.pop(), Some(2));
        assert_eq!(rev_list.pop(), Some(3));
        assert_eq!(rev_list.pop(), None);
    }

    #[test]
    fn test_into_vector() {
        let mut v = Vec::new();
        let mut s = SimpleLinkedList::new();
        for i in 1..4 {
            v.push(i);
            s.push(i);
        }
        let s_as_vec: Vec<i32> = s.into();
        assert_eq!(v, s_as_vec);
    }

    #[test]
    fn test_is_generic() {
        let mut list_u32: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list_u32.push(1);
        assert_eq!(list_u32.pop(), Some(1));

        let mut list_str: SimpleLinkedList<String> = SimpleLinkedList::new();
        let books = String::from("books");
        list_str.push(books.clone());
        assert_eq!(list_str.pop(), Some(books));
    }
}
