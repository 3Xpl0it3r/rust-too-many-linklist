### 第一版本
```rust
pub struct List<T>{
    head: Link<T>,
    tail: Link<T>,
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
        List { head: None, tail: None }
    }

    fn push(&mut self, element:T) {
        let new_node = Node::new(element);
        let  old_tail = std::mem::replace(&mut self.tail, Some(new_node));
        match old_tail{
            Some(mut old_tail) => {
                old_tail.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node)
            }
        }
    }


    fn pop(&mut self) -> Option<T>{
        self.head.take().map(|head|{
            let head = *head;
            self.head = head.next;

            if self.head.is_none(){
                self.tail = None
            }

            head.element
        })
    }
}

```
报错信息
```text
error[E0382]: use of moved value: `new_node`
  --> src/lib.rs:29:38
   |
25 |         let new_node = Node::new(element);
   |             -------- move occurs because `new_node` has type `Box<Node<T>>`, which does not implement the `Copy` trait
26 |         let  old_tail = std::mem::replace(&mut self.tail, Some(new_node));
   |                                                                -------- value moved here
...
29 |                 old_tail.next = Some(new_node);
   |                                      ^^^^^^^^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `unsafequeue` due to previous error

```
错误信息: Box 是有所有权的 ,Box 对里面对象指向是有所有权的， v2 版本使用一个non-owning pointer， Option<&'a mut Node<T>>


#### v2
```rust
pub struct List<'a, T>{
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>
}

impl <T> Node<T> {
    fn new(element :T )-> Box<Node<T>>{
        Box::new(Node{element, next:None})
    }
}

impl <'a, T> List<'a, T> {
    fn new() -> Self{
        List { head: None, tail: None }
    }

    fn push(&'a mut self, element: T) {
        let new_node = Node::new(element);
        // swap the old tail to point to the new tail
        let old_tail = mem::replace(&mut self.tail, Some(new_tail));
        match old_tail {
            Some(mut old_tail) => {
                // If the old tail existed, update it to point to the new tail
                old_tail.next = Some(new_tail);
            }
            None => {
                // Otherwise, update the head to point to it
                self.head = Some(new_tail);
            }
        }
    }
}
```

执行单元测试报错如下
```text
warning: `unsafequeue` (lib) generated 7 warnings
   Compiling unsafequeue v0.1.0 (/Users/l0calh0st/Git/l0calh0st.cn/Rust/rust-too-many-linklist/unsafequeue)
error[E0499]: cannot borrow `list` as mutable more than once at a time
  --> src/lib.rs:47:9
   |
46 |         list.push(1);
   |         ------------ first mutable borrow occurs here
47 |         list.push(1);
   |         ^^^^^^^^^^^^
   |         |
   |         second mutable borrow occurs here
   |         first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `unsafequeue` due to previous error
```
> 这个是因为在push 和 pop上面都标注了生命周期，这个意味着我们在‘a这个生命周期范围内可变的借用了self,  这个将一直持续到生命周期结束,(在rust里面 &mut 只能被调用一次)




