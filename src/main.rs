fn main() {
    let mut stack = LinkedList::new();
    (0..100).for_each(|val| stack.insert_front(val));
    while let Some(val) = stack.remove_front() {
        println!("Value popped: {:?}", val);
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    fn insert_front(&mut self, val: T) {
        match self.head {
            Some(_) => {
                let new_head = Node {
                    data: val,
                    next: self.head.take(),
                };
                self.head = Some(Box::new(new_head));
            }
            None => {
                let new_head = Node {
                    data: val,
                    next: None,
                };
                self.head = Some(Box::new(new_head));
            }
        }
        self.len += 1;
    }

    fn remove_front(&mut self) -> Option<T> {
        match self.head {
            Some(_) => {
                let removed_head = self
                    .head
                    .take()
                    .expect("Not empty so head should not be None.");
                self.head = removed_head.next;
                self.len -= 1;
                Some(removed_head.data)
            }
            None => None,
        }
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
        assert_eq!(stack.remove_front(), None);
    }
}
