use std::alloc::*;
use std::cmp;
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

    fn release(ptr: *mut Self) {
        unsafe {
            let allocator = System {};
            allocator.dealloc(
                ptr as *mut u8,
                Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap(),
            )
        }
    }
}

struct LoopLinkList<T: Sized> {
    head: *mut LinkNode<T>,
    tail: *mut LinkNode<T>,
    _size: usize,
}

impl<T: Sized> LoopLinkList<T> {
    fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            _size: 0,
        }
    }

    fn push(&mut self, val: T) {
        unsafe {
            if self.head == std::ptr::null_mut() {
                let mut node = LinkNode::new(val);
                (*node).next = node;
                self.head = node;
                self.tail = node;
            } else {
                let mut node = LinkNode::new(val);
                (*self.tail).next = node;
                (*node).next = self.head;
                self.tail = node;
            }
        }
        self._size += 1;
    }

    fn remove(&mut self, index: usize)
    where
        T: std::fmt::Display,
    {
        unsafe {
            if self.empty() {
                return;
            } else if self._size == 1 {
                LinkNode::release(self.head);
                self.head = std::ptr::null_mut();
                self.tail = std::ptr::null_mut();
                self._size -= 1;
            } else {
                let mut p = self.tail;
                let mut q = self.head;
                for _ in 0..index {
                    p = q;
                    q = (*q).next;
                }
                let next = (*q).next;
                (*p).next = next;
                if q == self.head {
                    self.head = next;
                } else if q == self.tail {
                    self.tail = p;
                }
                LinkNode::release(q);
                self._size -= 1;
            }
        }
    }

    fn size(&self) -> usize {
        self._size
    }

    fn empty(&self) -> bool {
        self._size == 0
    }
}

struct Deque {
    list: LoopLinkList<usize>,
}

impl Deque {
    fn new(size: usize) -> Self {
        let mut list = LoopLinkList::new();
        for i in 0..size {
            list.push(i + 1);
        }
        Self { list: list }
    }

    fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn size(&self) -> usize {
        self.list.size()
    }
}

fn run(size: usize) -> usize {
    unsafe {
        let mut deque = Deque::new(size);
        let mut i = 0;
        while deque.size() != 1 {
            i += 2;
            i %= deque.size();
            deque.remove(i);
        }
        (*deque.list.head).val
    }
}

fn main() {
    print!("{}", run(8));
}
