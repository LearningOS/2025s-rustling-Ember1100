/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

impl<T: Ord + Clone> LinkedList<T> {
    pub fn merge(list_a: &LinkedList<T>, list_b: &LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        let mut node_a = list_a.start;
        let mut node_b = list_b.start;

        while let (Some(a_ptr), Some(b_ptr)) = (node_a, node_b) {
            unsafe {
                if (*a_ptr.as_ptr()).val <= (*b_ptr.as_ptr()).val {
                    merged_list.add((*a_ptr.as_ptr()).val.clone());
                    node_a = (*a_ptr.as_ptr()).next;
                } else {
                    merged_list.add((*b_ptr.as_ptr()).val.clone());
                    node_b = (*b_ptr.as_ptr()).next;
                }
            }
        }

        while let Some(a_ptr) = node_a {
            unsafe {
                merged_list.add((*a_ptr.as_ptr()).val.clone());
                node_a = (*a_ptr.as_ptr()).next;
            }
        }

        while let Some(b_ptr) = node_b {
            unsafe {
                merged_list.add((*b_ptr.as_ptr()).val.clone());
                node_b = (*b_ptr.as_ptr()).next;
            }
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;

        while let Some(node) = current {
            unsafe {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", node.as_ref().val)?;
                current = node.as_ref().next;
                first = false;
            }
        }

        Ok(())
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(&list_a, &list_b);
        println!("merged List is {}", list_c);
        for (i, &val) in target_vec.iter().enumerate() {
            assert_eq!(val, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(&list_a, &list_b);
        println!("merged List is {}", list_c);
        for (i, &val) in target_vec.iter().enumerate() {
            assert_eq!(val, *list_c.get(i as i32).unwrap());
        }
    }
}
