fn main() {
    let mut stack = LinkedList::new();
    (0..=10).for_each(|val| stack.insert_front(val));

    for x in stack.iter() {
        println!("{}", x);
    }

    let sum: i32 = stack.iter().take(5).sum();
    println!("Sum is: {}", sum);
    // while let Some(val) = stack.remove_front() {
    //     println!("Value popped: {:?}", val);
    // }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn next_as_ref(&self) -> Option<&Node<T>> {
        match &self.next {
            Some(node) => Some(&(*node)),
            None => None,
        }
    }

    fn next_as_ref_mut(&mut self) -> Option<&mut Node<T>> {
        match &mut self.next {
            Some(node) => Some(&mut (*node)),
            None => None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert_front(&mut self, val: T) {
        let new_head = Node {
            data: val,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_head));
        self.len += 1;
    }

    pub fn remove_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(removed_head) => {
                self.head = removed_head.next;
                self.len -= 1;
                Some(removed_head.data)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(head) => Some(&head.data),
            None => None,
        }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        match self.head.as_mut() {
            Some(head) => Some(&mut head.data),
            None => None,
        }
    }

    fn head_as_ref(&self) -> Option<&Node<T>> {
        match &self.head {
            Some(node) => Some(&(*node)),
            None => None,
        }
    }

    fn head_as_ref_mut(&mut self) -> Option<&mut Node<T>> {
        match &mut self.head {
            Some(node) => Some(&mut (*node)),
            None => None,
        }
    }

    pub fn iter(&self) -> ListIterable<T> {
        ListIterable {
            curr: self.head_as_ref(),
        }
    }

    pub fn iter_mut(&mut self) -> ListIterableMut<T> {
        ListIterableMut {
            curr: self.head_as_ref_mut(),
        }
    }
}

pub struct ListIterable<'a, T> {
    curr: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIterable<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            Some(node) => {
                self.curr = node.next_as_ref();
                Some(&node.data)
            }
            None => None,
        }
    }
}

pub struct ListIterableMut<'a, T> {
    curr: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for ListIterableMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|node| {
            self.curr = node.next.as_mut().map(|node| &mut **node);
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_push_pop() {
        let mut stack = LinkedList::new();
        assert_eq!(stack.remove_front(), None);
        assert_eq!(stack.remove_front(), None);
        stack.insert_front(1);
        stack.insert_front(2);
        assert_eq!(stack.remove_front(), Some(2));
        assert_eq!(stack.remove_front(), Some(1));
        assert_eq!(stack.remove_front(), None);
        stack.insert_front(3);
        stack.insert_front(4);
        assert_eq!(stack.remove_front(), Some(4));
        assert_eq!(stack.remove_front(), Some(3));
        assert_eq!(stack.remove_front(), None);
    }
}
