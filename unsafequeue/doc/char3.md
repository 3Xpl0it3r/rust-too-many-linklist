#### 0x00 指针转换
&emsp; 将一个普通指针转换为原始指针
```rust
let raw_tail: *mut _ = &mut *new_tail;
```

```rust
pub struct List<T>{
    head: Link<T>,
    tail: *mut Node<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    element: T,
    next: Link<T>
}

impl <T> Node<T> {
    fn new(element: T) -> Box<Node<T>>{
        Box::new(Node { element: element, next: None })
    }
}

impl <T> List<T> {
    fn new()->Self{
        // 下面两个语句等价
        List { head: None, tail: std::ptr::null_mut() }
        // List { head: None, tail: 0 as *mut _}
    }

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
}

#[cfg(test)]
mod test{
    use super::List;
    #[test]
    fn basic_test(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
    }
} 


```

#### 总结
&emsp;和第一版的区域在于
```rust
// v1
fn push(&'a mut self, element: T){
    // self mut 引用会一直持续到a生命周期结束， 导致push 只能使用一次
}
// v2
fn push(&mut self, element: T) {
    // 这里面使用一个原始指针，这个是原始的指针，对于rust来讲就是一个整数, 因此rust不需要标注生命周期
}
```
