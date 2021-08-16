use std::alloc::*;
use std::fmt;
use std::mem;

const MAX_LIST_SIZE: usize = 50;

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
}

struct List {
    list: LinkList<f64>,
    size: usize,
}

impl List {
    fn new() -> Self {
        let mut list = LinkList::new();
        for i in 0..MAX_LIST_SIZE {
            list.push(0.);
        }
        Self {
            list: list,
            size: 0,
        }
    }

    fn add(&mut self, value: f64) {
        if self.size != MAX_LIST_SIZE {
            unsafe {
                let mut p = self.list.head;
                for i in 0..self.size {
                    p = (*p).next;
                }
                (*p).val = value;
            }
            self.size += 1;
        }
    }

    fn get_last(&self) -> Option<f64> {
        if self.size == 0 {
            Option::None
        } else {
            unsafe {
                let mut p = self.list.head;
                for i in 0..self.size {
                    p = (*p).next;
                }
                Option::Some((*p).val)
            }
        }
    }

    fn delete_last(&mut self) {
        if self.size != 0 {
            unsafe {
                let mut p = self.list.head;
                for i in 0..self.size {
                    p = (*p).next;
                }
                (*p).val = std::mem::zeroed();
            }
            self.size -= 1;
        }
    }

    fn full(&self) -> bool {
        self.size == MAX_LIST_SIZE
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mut p = self.list.head;
            for i in 0..self.size {
                match write!(f, "{}, ", (*p).val) {
                    fmt::Result::Ok(_) => {}
                    fmt::Result::Err(err) => return fmt::Result::Err(err),
                }
                p = (*p).next;
            }
        }
        fmt::Result::Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
