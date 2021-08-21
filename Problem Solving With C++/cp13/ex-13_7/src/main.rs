use std::alloc::*;
use std::cmp;
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

struct LinkList<T: Sized> {
    _head: *mut LinkNode<T>,
    _size: usize,
}

impl<T: Sized> LinkList<T> {
    fn new() -> Self {
        Self {
            _head: std::ptr::null_mut(),
            _size: 0,
        }
    }

    fn size(&self) -> usize {
        self._size
    }

    fn empty(&self) -> bool {
        self._head == std::ptr::null_mut()
    }

    fn insert(&mut self, val: T)
    where
        T: cmp::Ord,
    {
        if self.empty() {
            self._head = LinkNode::new(val);
            self._size += 1;
        } else {
            unsafe {
                let mut p = std::ptr::null_mut();
                let mut q = self._head;
                while val <= (*q).val && q != std::ptr::null_mut() {
                    p = q;
                    q = (*q).next;
                }
                if val == (*q).val {
                    return;
                } else if p == std::ptr::null_mut() {
                    self._head = LinkNode::new(val);
                    (*self._head).next = q;
                    self._size += 1;
                } else {
                    let node = LinkNode::new(val);
                    (*p).next = node;
                    (*node).next = q;
                    self._size += 1;
                }
            }
        }
    }

    fn erase<F>(&mut self, pred: F)
    where
        F: Fn(&T) -> bool,
    {
        if self.empty() {
            return;
        } else {
            unsafe {
                let mut p = std::ptr::null_mut();
                let mut q = self._head;
                while !pred(&(*q).val) && q != std::ptr::null_mut() {
                    p = q;
                    q = (*q).next;
                }
                if p == std::ptr::null_mut() {
                    LinkNode::release(self._head);
                    self._head = q;
                    self._size -= 1;
                } else if q != std::ptr::null_mut() {
                    (*p).next = (*q).next;
                    LinkNode::release(q);
                    self._size -= 1;
                }
            }
        }
    }

    fn find<F>(&self, pred: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        if self.empty() {
            Option::None
        } else {
            unsafe {
                let mut p = self._head;
                while p != std::ptr::null_mut() {
                    if pred(&(*p).val) {
                        return Option::Some(&(*p).val);
                    }
                    p = (*p).next;
                }
                Option::None
            }
        }
    }

    fn for_each<F: Fn(&T)>(&self, pred: F) {
        unsafe {
            let mut p = self._head;
            while p != std::ptr::null_mut() {
                pred(&(*p).val);
                p = (*p).next;
            }
        }
    }
}

struct Computer {
    index: usize,
    user: String,
}

impl cmp::PartialEq for Computer {
    fn eq(&self, other: &Computer) -> bool {
        self.index == other.index
    }
}

impl cmp::Eq for Computer {}

impl cmp::PartialOrd for Computer {
    fn partial_cmp(&self, other: &Computer) -> Option<cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl cmp::Ord for Computer {
    fn cmp(&self, other: &Computer) -> cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

struct Lab {
    index: usize,
    computers: LinkList<Computer>,
    amount: usize,
}

impl Lab {
    fn new(index: usize, computer_amount: usize) -> Lab {
        Lab {
            index: index,
            computers: LinkList::new(),
            amount: computer_amount,
        }
    }

    fn login(&mut self, id: String, computer_index: usize) -> bool {
        if computer_index >= self.amount {
            return false;
        }

        let size = self.computers.size();
        self.computers.insert(Computer {
            index: computer_index,
            user: id,
        });

        size != self.computers.size()
    }

    fn logout(&mut self, id: String, computer_index: usize) -> bool {
        let size = self.computers.size();
        self.computers
            .erase(|computer| computer.index == computer_index);
        size != self.computers.size()
    }

    fn find(&self, id: String) -> Option<usize> {
        match self.computers.find(|computer| computer.user == id) {
            Option::Some(computer) => Option::Some(computer.index),
            Option::None => Option::None,
        }
    }

    fn print(&self) {
        print!("{}\t", self.index);
        self.computers
            .for_each(|computer| print!("{}: {},", computer.index, computer.user));
        println!("");
    }
}

fn main() {
    let mut lab = Lab::new(4, 5);

    lab.login("99577".to_string(), 4);
    lab.print();
    println!("");
}
