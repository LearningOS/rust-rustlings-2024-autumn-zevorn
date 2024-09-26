/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    /// 反转列表的方向。
    /// 此方法会交换列表的 `start` 和 `end` 指针，使原来的 `start` 成为 `end`，反之亦然。
    /// 同时，它会反转每个节点的 `next` 和 `prev` 指针，以确保反转后的列表结构正确。
    pub fn reverse(&mut self) {
        // 保存原始的起始节点，用于反转后赋值给结束节点
        let d = self.start;
        // 将原始结束节点的 prev 指针设为 None，准备进行反转
        let mut a = unsafe { (*self.end.unwrap().as_ptr()).prev };
        unsafe { (*self.end.unwrap().as_ptr()).prev = None; }
        // 保存原始的结束节点，用于遍历并反转节点
        let mut b = self.end;
        // 从结束节点开始遍历，反转每个节点的指针
        while let Some(ptr) = b {
            if a == self.start {
                // 当遍历到达原始起始节点时，更新其 next 指针到原始结束节点
                unsafe {
                    (*ptr.as_ptr()).next = a;
                    // 原始起始节点的 prev 指针指向原始结束节点，并将其 next 指针设为 None
                    unsafe {
                        (*a.unwrap().as_ptr()).prev = b;
                        (*a.unwrap().as_ptr()).next = None;
                        break;
                    }
                }
            } else {
                // 对其他节点，更新它们的 next 指针到前一个节点，并调整 prev 指针
                unsafe {
                    (*ptr.as_ptr()).next = a;
                }
                // 临时存储前一个节点
                let tmp = a;
                // 移动 a 指向前一个节点之前的节点
                a = unsafe {
                    (*a.unwrap().as_ptr()).prev
                };
                // 更新临时存储节点的 prev 指针到当前节点，准备下一轮反转
                unsafe {
                    (*tmp.unwrap().as_ptr()).prev = b;
                }
                // 移动 b 到临时存储节点，准备下一轮遍历
                b = tmp;
            }
        }

        // 在整个列表反转完成后，更新起始和结束节点
        self.start = self.end;
        self.end = d;
    }

}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}