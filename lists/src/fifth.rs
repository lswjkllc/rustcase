use std::ptr;


/*
单向队列: 从尾部push, 从头部pop
 */
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None, tail: ptr::null_mut() }
    }

    // 从尾部 push
    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node{
            elem: elem,
            next: None
        });

        let raw_tail: *mut _ = &mut *new_tail;
        
        if !self.tail.is_null() {
            // 尾节点 存在
            // 操作: 将 新节点 赋值给 当前尾节点 的下一个节点
            unsafe {
                // *self.tail 解引用 => Node<T>
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // 尾节点 不存在
            // 状态: 链表/队列 头节点/尾节点 都不存在
            // 操作: 将 新节点 赋值给 头节点
            self.head = Some(new_tail);
        }
        // 保存新增的节点为 尾节点
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}