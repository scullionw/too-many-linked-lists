fn main() {
    let mut stack = LinkedList::new();
    stack.insert_front(5.0);
    stack.insert_front(6.9);

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
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn insert_front(&mut self, val: T) {
        if self.is_empty() {
            let new_head = Node {
                data: val,
                next: None,
            };
            self.head = Some(Box::new(new_head));
        } else {
            let current_head = self.head.take();
            let new_head = Node {
                data: val,
                next: current_head,
            };
            self.head = Some(Box::new(new_head));
        }
        self.len += 1;
    }

    fn remove_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let removed_head = self
                .head
                .take()
                .expect("Not empty so head should not be None.");
            self.head = removed_head.next;
            self.len -= 1;
            Some(removed_head.data)
        }
    }
}
