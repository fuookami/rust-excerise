use std::alloc::*;
use std::fmt;
use std::mem;

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

    fn reverse(&mut self) {
        if self.empty() {
            return;
        }

        let mut p = std::ptr::null_mut();
        let mut q = self.head;

        while q != std::ptr::null_mut() {
            unsafe {
                let r = (*q).next;
                (*q).next = p;
                p = q;
                q = r;
            }
        }
        self.head = p;
    }

    fn write(&self)
    where
        T: fmt::Display,
    {
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

fn main() {
    let mut list = LinkList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.write();
    list.reverse();
    list.write();
}
