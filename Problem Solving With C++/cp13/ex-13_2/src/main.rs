use std::alloc::*;
use std::mem;
use std::fmt;

struct LinkNode<T: Sized> {
    val: T,
    next: *mut LinkNode<T>,
}

impl<T: Sized> LinkNode<T> {
    const ALIGN: usize = mem::align_of::<LinkNode<T>>();
    const ELEM_SIZE: usize = mem::size_of::<LinkNode<T>>();

    fn new(v: T) -> *mut Self {
        unsafe {
            let allocator = System {};
            let ptr = allocator
                .alloc(Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap())
                as *mut Self;
            *ptr = Self {
                val: v,
                next: std::ptr::null_mut(),
            };
            ptr
        }
    }
}

struct LinkList<T: Sized> {
    head: *mut LinkNode<T>,
}

impl<T: Sized> LinkList<T> {
    fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
        }
    }

    fn empty(&self) -> bool {
        self.head == std::ptr::null_mut()
    }

    fn push(&mut self, val: T) {
        if self.empty() {
            self.head = LinkNode::new(val);
        } else {
            unsafe {
                let mut p = self.head;
                while (*p).next != std::ptr::null_mut() {
                    p = (*p).next;
                }
                (*p).next = LinkNode::new(val);
            }
        }
    }

    fn write(&self) where T: fmt::Display {
        if self.empty() {
            return;
        } else {
            unsafe {
                let mut p = self.head;
                while p != std::ptr::null_mut() {
                    print!("{}, ", (*p).val);
                    p = (*p).next;
                }
                print!("\n")
            }
        }
    }
}

fn merge<T: Sized + PartialOrd>(l1: LinkList<T>, l2: LinkList<T>) -> LinkList<T> {
    let mut list = LinkList::new();
    let mut curr = list.head;
    let mut p = l1.head;
    let mut q = l2.head;
    let mut add = |ptr| {
        unsafe {
            if curr == std::ptr::null_mut() {
                list.head = ptr;
                curr = ptr;
            } else {
                (*curr).next = ptr;
                curr = ptr;
            }
        }
    };
    while p != std::ptr::null_mut() || q != std::ptr::null_mut() {
        unsafe {
            if p == std::ptr::null_mut() {
                add(q);
                q = (*q).next;
            } else if q == std::ptr::null_mut() {
                add(p);
                p = (*p).next;
            } else {
                if (*p).val < (*q).val {
                    add(p);
                    p = (*p).next;
                } else {
                    add(q);
                    q = (*q).next;
                }
            }
        }
    }
    list
}

fn main() {
    let mut l1 = LinkList::new();
    let mut l2 = LinkList::new();

    l1.push(1);
    l1.push(3);
    l1.push(5);
    l2.push(2);
    l2.push(4);
    l1.write();
    l2.write();

    let l = merge(l1, l2);
    l.write();
}
