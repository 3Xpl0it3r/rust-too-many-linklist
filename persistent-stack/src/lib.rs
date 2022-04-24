use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>
}


impl <T> List<T> {
    pub fn new() -> List<T>{
        List { head: None }
    }
    pub fn prepend(&mut self, element: T) -> List<T>{
        List{head: Some(Rc::new(Node{
            element,
            next: self.head.clone()
        }))}
    }

    pub fn tail(&mut self) -> List<T>{
        List { head: self.head.as_ref().and_then(|node|node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

// Rc readonly， reference， cannot mutable
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl <T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}


impl <T> Drop for List<T> {
    fn drop(&mut self) {
        let mut currnet_node = self.head.take();
        while let Some(node) = currnet_node {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                currnet_node = node.next.take();
            } else {
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn it_works() {
        let list = List::new().prepend(1).prepend(2).prepend(3).prepend(4);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
