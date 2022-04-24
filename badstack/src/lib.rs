struct List{
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    element: i32,
    next: Link,
}

impl List{
    fn new()->List{
        List { head: Link::Empty }
    }

    fn push_v1(&mut self, element: i32) {
        // 1 create new node
        let mut new_node = Node{element, next: Link::Empty};
        // 2 new node points to the head node
        new_node.next = std::mem::replace(&mut self.head, Link::Empty);
        // 3 head node points to the newnode
        self.head = Link::More(Box::new(new_node))
    } 

    fn push_v2(&mut self, element: i32) {
        let new_node = Link::More(Box::new(Node { element: element, next: std::mem::replace(&mut self.head, Link::Empty) }));
        self.head = new_node;
    }

    fn pop_v1(&mut self) -> Option<i32> {
        // let old_node = std::mem::replace(&mut self.head, Link::Empty);
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }

    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_node = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current_node {
            current_node = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn it_works() {
        let mut list = List::new();
        for _i in 0..5{
            list.push_v1(_i);
        }
        for _i in 5..10{
            list.push_v2(_i);
        }
        for _i in 0..10{
            assert_eq!(list.pop_v1(), Some(9-_i));
        }
    }
}
