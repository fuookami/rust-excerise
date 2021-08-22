use chrono;
use std::alloc::*;
use std::mem;

type DateTime = chrono::DateTime<chrono::Local>;

struct Customer {
    ticket_number: usize,
    entered_time: DateTime,
    helped_time: Option<DateTime>,
}

static mut allocator: System = System {};
static mut customer_amount: usize = 0;

struct LinkNode {
    customer: Customer,
    next: *mut LinkNode,
}

impl LinkNode {
    const ALIGN: usize = mem::align_of::<LinkNode>();
    const ELEM_SIZE: usize = mem::size_of::<LinkNode>();

    fn new() -> *mut Self {
        unsafe {
            let ptr = allocator
                .alloc(Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap())
                as *mut Self;
            *ptr = Self {
                customer: Customer {
                    ticket_number: customer_amount,
                    entered_time: chrono::Local::now(),
                    helped_time: Option::None,
                },
                next: std::ptr::null_mut(),
            };
            ptr
        }
    }

    fn release(ptr: *mut Self) {
        unsafe {
            allocator.dealloc(
                ptr as *mut u8,
                Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap(),
            )
        }
    }
}

struct CustomerList {
    head: *mut LinkNode,
    tail: *mut LinkNode,
    records: Vec<(usize, DateTime, DateTime)>,
    leisure_amount: usize,
}

impl CustomerList {
    const WINDOW_AMOUNT: usize = 3;

    fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            records: Vec::new(),
            leisure_amount: Self::WINDOW_AMOUNT,
        }
    }

    fn enter_custome(&mut self) -> Option<chrono::Duration> {
        let new_node = LinkNode::new();
        if self.leisure_amount != 0 {
            unsafe {
                (*new_node).customer.helped_time = Option::Some(chrono::Local::now());
                self.refresh_records(&(*new_node).customer);
            }
            LinkNode::release(new_node);
            Option::Some(chrono::Duration::seconds(0))
        } else {
            if self.empty() {
                self.head = new_node;
                self.tail = new_node;
            } else {
                unsafe {
                    (*self.tail).next = new_node;
                }
                self.tail = new_node;
            }
            self.average_wait_time()
        }
    }

    fn help_custome(&mut self) {
        if self.empty() {
            self.leisure_amount += 1;
            return;
        } else {
            unsafe {
                (*self.head).customer.helped_time = Option::Some(chrono::Local::now());
                self.refresh_records(&(*self.head).customer);
                LinkNode::release(self.head);
                if self.head == self.tail {
                    self.head = std::ptr::null_mut();
                    self.tail = std::ptr::null_mut();
                } else {
                    self.head = (*self.head).next;
                }
            }
        }
    }

    fn empty(&self) -> bool {
        self.head == std::ptr::null_mut()
    }

    fn refresh_records(&mut self, customer: &Customer) {
        if self.records.len() == Self::WINDOW_AMOUNT {
            self.records.remove(0);
        }
        assert!(customer.helped_time.is_some());
        self.records.push((
            customer.ticket_number,
            customer.entered_time,
            customer.helped_time.unwrap(),
        ));
    }

    fn average_wait_time(&self) -> Option<chrono::Duration> {
        if self.records.is_empty() {
            Option::None
        } else {
            let mut wait_time = chrono::Duration::seconds(0);
            for (_, entered_time, helped_time) in &self.records {
                wait_time = wait_time + (*helped_time - *entered_time);
            }
            Option::Some(wait_time)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
