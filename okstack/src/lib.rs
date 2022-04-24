pub struct List<T>{
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}


impl <T> List<T> {
    pub fn new()->List<T>{
        List { head: None }
    }

    pub fn push_v1(&mut self, element: T) {
        let new_node = Some(Box::new(Node{
            element,
            next: self.head.take()
        }));
        self.head = new_node;
    }

    pub fn push_v2(&mut self, element: T) {
        self.head.take().map(|node|{
            let new_node = Some(Box::new(Node{element , next: Some(node)}));
            self.head = new_node;
        });
    } 

    pub fn pop_v1(&mut self) -> Option<T> {
        let old_node = self.head.take();
        match old_node {
            Some(node) => {
                self.head = node.next;
                Some(node.element)
            },
            None => None
        }
    }
    pub fn pop_v2(&mut self) -> Option<T> {
        self.head.take().map(|node|{
            self.head = node.next;
            node.element
        })
    }

    pub fn peek_v1(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.element),
            None => None
        }
    }
    pub fn peek_v2(&self) -> Option<&T> {
        self.head.as_ref().map(|node|&node.element)
    }
    pub fn peek_v3(&self) -> Option<&T> {
        self.head.as_deref().map(|node|&node.element)
    }
}

// intoiter => got ownership
pub struct IntoIter<T>(List<T>);
impl <T> List<T> {
    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
}

impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.head.take() {
            Some(node) => {
                self.0.head = node.next;
                Some(node.element)
            }
            None => None
        }
    }
}

// iterator => get reference
pub struct Iter<'a,T> {
    next: Option<&'a Node<T>>,
}

impl <T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
        // Iter { next: self.head.as_ref().map(|node|&**node) }
    }
}

impl <'a,T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // move on reference  == copy
        self.next.map(|node|{
            // pub const fn as_deref(&self) -> Option<&T::Target>
            self.next = node.next.as_deref();
            // self.next = node.next.as_ref().map(|node|&**node);
            &node.element
        })
    }
}

// iter mut => get mut reference
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl <T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        // IterMut { next: self.head.as_ref().map(|node|&mut **node) }
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            // self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.element
        })
    }
}


impl <T> Drop for List<T> {
    fn drop(&mut self) {
        let mut currnode = self.head.take();
        while let Some(mut node) = currnode {
            currnode = node.next.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn it_works() {
        let mut list = List::new();
        for _i in 0..5 {
            list.push_v1(_i);
        }
        assert_eq!(list.peek_v1(), Some(&4));
        for _i in 5..10{
            list.push_v2(_i);
        }
        assert_eq!(list.peek_v3(), Some(&9));
        for _i in 0..5 {
            assert_eq!(list.pop_v1(), Some(9-_i));
        }
        for _i in 0..5 {
            assert_eq!(list.pop_v2(), Some(4-_i));
        }

    }

    #[test]
    fn intoiter_test() {
        let mut list = List::new();
        for _i in 0..5 {
            list.push_v1(_i);
        }
        let mut iter = list.into_iter();
        for _i in 0..5 {
            assert_eq!(iter.next(), Some(4 - _i));
        }
    }

    #[test]
    fn iter_test(){
        let mut list = List::new();

        for _i in 0..5 {
            list.push_v1(_i);
        }
        
        let mut iter = list.iter();
        for _i in 0..5 {
            assert_eq!(iter.next(), Some(&(4 - _i)));
        }
    }
    #[test]
    fn iter_mut_test(){
        let mut list = List::new();

        for _i in 0..5 {
            list.push_v1(_i);
        }
        
        let mut iter_mut  = list.iter_mut();

        for _i in 0..5 {
            assert_eq!(iter_mut.next(), Some(&mut (4 - _i)));
        }

        let mut new_number = 20;
        match list.iter_mut().next() {
            Some(mut element) => {
                element = &mut new_number;
                println!("{}", element);
                ()
            },
            None => ()
        };

        let mut iter = list.iter_mut();
        println!("{:?}", iter.next());
        
    }
}
