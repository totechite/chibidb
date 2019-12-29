use std::cell::RefCell;
use std::rc::Rc;

struct Btree {
    root_node: Node,
}

type Key = usize;
type Value = String;

struct Node {
    depth: usize,

    smaller: Option<Key>,
    middle: Option<Key>,
    bigger: Option<Key>,

    fst_node: Option<Box<Node>>,
    scd_node: Option<Box<Node>>,
    thd_node: Option<Box<Node>>,
    fth_node: Option<Box<Node>>,
}

struct Leaf {
    depth: usize,

    rnode_pointer: Option<Rc<RefCell<Node>>>,

    smaller: (Key, Value),
    middle: (Key, Value),
    bigger: (Key, Value),
}

