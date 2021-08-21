use std::alloc::*;
use std::collections::HashMap;
use std::mem;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    South,
    North,
    West,
    East,
}

struct LinkNode {
    next: HashMap<Direction, *mut LinkNode>,
}

static mut allocator: System = System {};

impl LinkNode {
    const ALIGN: usize = mem::align_of::<LinkNode>();
    const ELEM_SIZE: usize = mem::size_of::<LinkNode>();

    fn new() -> *mut Self {
        unsafe {
            let ptr = allocator
                .alloc(Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap())
                as *mut Self;
            *ptr = Self {
                next: HashMap::new(),
            };
            ptr
        }
    }
}

struct Maze {
    start_node: *mut LinkNode,
    nodes: HashMap<&'static str, *mut LinkNode>,
}

impl Maze {
    fn new() -> Self {
        unsafe {
            let mut start_node = LinkNode::new();
            let mut maze = Maze {
                start_node: start_node,
                nodes: HashMap::new(),
            };

            maze.nodes.insert("A", start_node);

            let mut node = LinkNode::new();
            (*start_node).next.insert(Direction::East, node);
            (*node).next.insert(Direction::West, start_node);
            maze.nodes.insert("B", node);

            maze
        }
    }
}

fn main() {
    println!("Hello, world!");
}
