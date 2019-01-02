fn main() {
    let mut stack = LinkedList::new();
    (0..=10).for_each(|val| stack.insert_front(val));
    while let Some(val) = stack.remove_front() {
        println!("Value popped: {:?}", val);
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
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
