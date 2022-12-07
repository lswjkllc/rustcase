
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        // 构建新节点
        let new_node = Node{
            elem: elem,
            // 使用 Link::Empty 替换 self.head 的值, 并返回当前值
            next: std::mem::replace(&mut self.head, Link::Empty)
            // std::mem::take(&mut self.head); // 将 self.head 替换成默认值, 并返回当前值
        };
        
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(head) => {
                self.head = head.next;
                Some(head.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_node = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_node {
            cur_node = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[derive(Default)]
enum Link {
    #[default]
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(3);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(3));
        
        list.push(2);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        // drop(list);
    }
}