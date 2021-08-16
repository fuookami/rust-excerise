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
    _curr: *mut LinkNode<f64>,
    _size: usize,
}

impl List {
    fn new() -> Self {
        let mut list = LinkList::new();
        for i in 0..MAX_LIST_SIZE {
            list.push(0.);
        }
        Self {
            list: list,
            _curr: std::ptr::null_mut(),
            _size: 0,
        }
    }

    fn front(&self) -> Option<f64> {
        if self.empty() {
            Option::None
        } else {
            unsafe { Option::Some((*self.list.head).val) }
        }
    }

    fn last(&self) -> Option<f64> {
        if self.empty() {
            Option::None
        } else {
            unsafe {
                let mut p = self.list.head;
                for _ in 0..self._size {
                    p = (*p).next;
                }
                Option::Some((*self.list.head).val)
            }
        }
    }

    fn current(&self) -> Option<f64> {
        if self._curr == std::ptr::null_mut() {
            Option::None
        } else {
            unsafe { Option::Some((*self._curr).val) }
        }
    }

    fn advance(&mut self) {
        if self._curr == std::ptr::null_mut() {
            if !self.empty() {
                self._curr = self.list.head;
            }
        } else {
            unsafe {
                let mut i = 0;
                let mut p = self.list.head;
                while p != self._curr {
                    p = (*p).next;
                    i += 1;
                }

                if i != self._size {
                    self._curr = (*self._curr).next
                }
            }
        }
    }

    fn reset(&mut self) {
        if !self.empty() {
            self._curr = self.list.head;
        }
    }

    fn insert(&mut self, target_val: f64, insert_val: f64) {
        unsafe {
            let mut i = 0;
            let mut p = self.list.head;
            while (*p).val != target_val && i != self._size {
                p = (*p).next;
                i += 1;
            }

            let mut val = insert_val;
            while i != self._size {
                std::mem::swap(&mut (*p).val, &mut val);
                p = (*p).next;
                i += 1;
            }
        }
    }

    fn empty(&self) -> bool {
        self._size == 0
    }

    fn size(&self) -> usize {
        self._size
    }
}

fn main() {
    println!("Hello, world!");
}
