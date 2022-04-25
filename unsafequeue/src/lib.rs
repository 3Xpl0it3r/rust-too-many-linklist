mod example;
pub struct List<T>{
    head: Link<T>,
    tail: *mut Node<T>
}

// type Link<T> = Option<Box<Node<T>>>;
type Link<T> = *mut Node<T>;

struct Node<T>{
    element: T,
    next: Link<T>
}

impl <T> Node<T> {
    fn new(element: T) -> Box<Node<T>>{
        Box::new(Node { element: element, next: std::ptr::null_mut() })
    }
}

impl <T> List<T> {
    fn new()->Self{
        // 下面两个语句等价
        List { head: std::ptr::null_mut(), tail: std::ptr::null_mut() }
        // List { head: None, tail: 0 as *mut _}
    }

    fn push(&mut self, element:T) {
        unsafe{
            let new_tail = Box::into_raw(Node::new(element));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            }else {
                self.head = new_tail;
            }
            self.tail = new_tail;
        }
    }

    fn pop(&mut self) -> Option<T>{
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = std::ptr::null_mut();
                }
                Some(head.element)
            }
        }
    }
    /*
    fn push(&mut self, element:T) {
        let mut new_node = Node::new(element);
        // 将box 强制转换为原始指针
        let raw_node_ptr: *mut _ = &mut *new_node;

        if !self.tail.is_null() { // 判断尾部是不是存在, 如果尾节点存在
            // self.tail.next = Some(new_node);  // [v1] 报错no field `next` on type `*mut Node<T>，
            // 在rust里面当我们需要引用一个原始指针的时候，需要手动的取消他们的引用
            // (*self.tail).next = Some(new_node) ; // [v2] 依旧不ok ，dereference of raw pointer is unsafe and requires unsafe function or block，  z
            //  在rust里面，不安全的代码只能放在unsafe里面
            unsafe{
                (*self.tail).next = Some(new_node);
            }
            // 如果我们不去解引用这些原始指针没有任何问题，因为我们只是在读取或者写一个整数，但是一旦我们解引用的时候，rust就会去提醒我们这是不安全的，我们需要在unsafe
            // 里面去引用。
        } else {
            self.head = Some(new_node);
        }
        self.tail = raw_node_ptr;
    }

    fn pop(&mut self) -> Option<T>{
        self.head.take().map(|head|{
            let head = *head;
            self.head = head.next;

            if self.head.is_none(){
                self.tail = std::ptr::null_mut();
            }
            head.element
        })
    }
    */
}

// into iter = get owner ship
pub struct IntoIter<T>(List<T>);
impl <T> List<T> {
    fn into_iter(self)-> IntoIter<T>{
        IntoIter(self)
    }
}

impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter = shared ownerreference
pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>,
}

impl <T> List<T> {
    fn iter(&self) -> Iter<'_, T>{
        unsafe{
            Iter { next: self.head.as_ref() }
        }
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe{
            self.next.map(|head|{
                self.next = head.next.as_ref();
                &head.element
            })
        }
    }
}


pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
}

impl <T> List<T> {
    fn iter_mut(&mut self) -> IterMut<'_, T>{
        unsafe{
            IterMut { next: self.head.as_mut() }
        }
    }
}

impl <'a,T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe{
            self.next.take().map(|node|{
                self.next = node.next.as_mut();
                &mut node.element
            })
        }
    }
}


#[cfg(test)]
mod test{
    use super::List;
    #[test]
    fn iter_mut(){
        let mut list = List::new();
        for _i in 0..10{
            list.push(_i);
        }
        // let mut iter_mut = list.iter_mut();
    }
    #[test]
    fn basic_test(){
        let mut list = List::new();
        for _i in 0..10{
            list.push(_i);
        }

        let mut into_iter = list.into_iter();
        for _i in 0..10{
            assert_eq!(into_iter.next(), Some(_i));
            // assert_eq!(list.pop(), Some(_i));
        }
    }


    
} 


