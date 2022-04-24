use std::{rc::Rc, cell::{RefCell, Ref, RefMut}};

/// Point1
/// fn borrow(&self) -> Ref<'_, T>;
/// fn borrow_mut(&self) -> RefMut<'_, T>;
/// Ref/ RefMut like Rc, but for borrow/borrow_mut
///
/// 
/// Cell<T>  for copy , 只能通过set()  和 get（） 来获取和设置值
/// RefCell<T>  for none copy , 只能通过borrow_mut 和 borrow 来获取引用
///
///
///
/// RefCell/Cell 更像一个容器， 它不算一个智能指针
///
///
pub struct List<T>{
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct  Node<T> {
    element: T,
    prev: Link<T>,
    next: Link<T>
}

impl <T> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node { element: element, prev: None, next: None }))
    }
}

impl <T> List<T> {
    fn new()->Self {
        List { head: None, tail: None }
    }

    fn push_front(&mut self, element: T) {
        // point1  RefCell 只能通过borrow_mut 和 borrow 来获取引用，  /
        let new_node = Node::new(element);
        match self.head.take(){
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => { //  没有节点情况下
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head|{
            match old_head.borrow_mut().next.take(){
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None=> {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|head|{
            // borrow_mut()的函数前面如下
            // fn borrow_mut<'a>(&'a self) -> RefMut<'a, T> 它会创建一个临时变量RefMut
            // ,如果返回值是 Option<&T>的话，
            // 也就意味着我们只需要T，但是当离开作用域后Ref就会被释放掉, => 直接返回RefMut
            // head.borrow_mut().element
            Ref::map(head.borrow(), |node|&node.element)
        })
    }

    fn peek_front_mut(&mut self) -> Option<RefMut<T>>{
        self.head.as_ref().map(|head|{
            RefMut::map(head.borrow_mut(), |node|&mut node.element)
        })
    }

    fn push_back(&mut self, element: T){
        let new_tail = Node::new(element);
        match self.tail.take(){
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }
    

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail|{
            match old_tail.borrow_mut().prev.take(){
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().element
        })
    }

    fn peek_back(&self) -> Option<Ref<T>>{
        self.tail.as_ref().map(|tail|{
            Ref::map(tail.borrow(), |node|&node.element)
        })
    }

    fn peek_back_mut(&mut self) -> Option<RefMut<T>>{
        self.tail.as_ref().map(|tail|{
            RefMut::map(tail.borrow_mut(), |node|&mut node.element)
        })
    }
}


pub struct IntoIter<T>(List<T>);

impl <T> List<T> {
    fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
}


impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl <T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test{
    use super::List;
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

}

