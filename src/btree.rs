use std::fmt::Debug;
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::any::{Any, TypeId};
use std::ops::Deref;

const M: usize = 3;

#[derive(Debug)]
pub struct Btree<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    pub root_node: Box<dyn Node<K, V>>,
}

pub trait Node<K, V>: Debug
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    //core
    fn delete(&mut self, key: K) -> Status;
    fn print(&self, buf: Vec<(K, V)>) -> Vec<(K, V)>;
    fn find(&self, key: K) -> Option<(K, V)>;
    fn insert(&mut self, k: K, v: V) -> Option<(K, Rc<RefCell<dyn Node<K, V>>>)>;

    //utils
    fn is_InternalNode(&self) -> bool;
    fn is_LeafNode(&self) -> bool;
    fn marge(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool;
    fn share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool;
    fn split(&mut self) -> (K, Rc<RefCell<dyn Node<K, V>>>);
    fn cmp_key(&self) -> K;
    fn take_items(&mut self) -> Rc<RefCell<dyn Node<K, V>>>;
    fn reorg_key(&mut self) -> K;
}

#[derive(Debug)]
pub struct InternalNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    keys: Vec<K>,
    node: Vec<Rc<RefCell<dyn Node<K, V>>>>,
}

#[derive(Default, Clone, Debug)]
pub struct LeafNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    data: Vec<(K, V)>,
    next_leaf: Option<Rc<RefCell<LeafNode<K, V>>>>,
}

impl<K, V> Btree<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    pub fn new() -> Self {
        Btree {
            root_node: Box::new(LeafNode::new()),
        }
    }

    pub fn print(&mut self) -> Vec<(K, V)> {
        self.root_node.print(Vec::new())
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some((new_key, new_node)) = self.root_node.insert(k, v) {
            self.split(new_key, new_node);
        }
    }

    pub fn find(&self, key: K) -> Option<(K, V)> {
        self.root_node.find(key)
    }

    fn split(&mut self, new_key: K, new_node: Rc<RefCell<dyn Node<K, V>>>) {
        let mut new_root = InternalNode::new();
        new_root.keys.push(new_key);
        new_root.node.push(new_node);
        new_root.node.push(self.root_node.take_items());
        new_root.keys.sort();
        new_root.node.sort_by_key(|a| a.borrow().cmp_key());
        self.root_node = Box::new(new_root);
    }

    pub fn delete(&mut self, key: K) -> Status {
        self.root_node.delete(key)
    }
}

enum NodePoint {
    smaller_than_this_key,
    bigger_than_this_key,
    smallest,
    biggest,
    N_A,
}

impl<K, V> Node<K, V> for InternalNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn is_InternalNode(&self) -> bool {
        TypeId::of::<InternalNode<K, V>>() == self.type_id()
    }
    fn is_LeafNode(&self) -> bool {
        TypeId::of::<LeafNode<K, V>>() == self.type_id()
    }

    fn marge(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<InternalNode<K, V>>>>>(Box::new(from));
            while let Some(elem) = from.borrow_mut().node.pop() {
                self.node.push(elem);
            }
            self.node.sort_by_key(|a| a.borrow().cmp_key());
            self.keys.sort();
        }
        true
    }

    fn share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<InternalNode<K, V>>>>>(Box::new(from));
            let len = from.borrow().node.len().clone();
            if M <= (self.node.len() + len) {
                for i in 0..M / 2 {
                    self.node.push(from.borrow_mut().node.remove(i))
                };
                self.node.sort_by_key(|a| a.borrow().cmp_key());
                true
            } else { false }
        }
    }

    fn delete(&mut self, key: K) -> Status {
        self.node.sort_by_key(|a| a.borrow().cmp_key());
        let mut need_reorg_key = false;
        let mut node_address = {
            let mut result = self.keys.len();
            for (i, comp_key) in self.keys.iter().enumerate() {
                if &key < comp_key { result = i; };
                if &key == comp_key { need_reorg_key = true; };
            }
            result
        };
        let keys_address = if node_address == self.keys.len() { node_address - 1 } else { node_address };
        let delete_status = self.node[node_address].borrow_mut().delete(key);
        println!("{:?}", &delete_status);
        if need_reorg_key {
            self.keys.remove(keys_address);
            println!("{:?}", node_address);
            println!("{:?}", self);
            self.keys.push(self.node[node_address].borrow_mut().reorg_key());
            self.keys.sort();
        };
        println!("{:?}", self);

        let result = match delete_status {
            Status::OK => delete_status,
            Status::OK_REMOVE => {
                let marge_node_address = if node_address == 0 { 0 } else { node_address - 1 };
                let (n1, n2) = (self.node[marge_node_address].clone(), self.node[marge_node_address + 1].clone());
                let shared = n1.borrow_mut().share(n2.clone());
                if !shared {
                    self.node.remove(node_address);
                } else {
                    if need_reorg_key {
                        self.reorg_key();
                    }
                }
                println!("{:?}", self);

                if M / 2 >= self.node.len() {
                    return Status::OK_NEED_REORG;
                };
                Status::OK
            }
            Status::OK_NEED_REORG => {
                let marge_node_address = if node_address == 0 { 0 } else { node_address - 1 };
                let (n1, n2) = (self.node[marge_node_address].clone(), self.node[marge_node_address + 1].clone());
                let shared = n1.borrow_mut().share(n2.clone());
                if shared {
                    n1.borrow_mut().marge(n2);
                    self.node.remove(marge_node_address + 1);
                    self
                };
                self.node.sort_by_key(|a| a.borrow().cmp_key());

                if n1.borrow().is_InternalNode() {
                    //子がInternalNodeの場合
//                    for i in node_address..self.node.len() - 1 {
//                        self.node[node_address] = self.node.remove(node_address + 1);
//                        self.keys[i] = self.keys.remove(node_address + 1);
//                    }
//                    self.node.remove(self.node.len() - 1);
                    if M / 2 >= self.node.len() {
                        return Status::OK_NEED_REORG;
                    };
                }
                if n1.borrow().is_LeafNode() {
                    //子がLeafNodeの場合
                    if M / 2 - 1 > self.node.len() {
                        return Status::OK_NEED_REORG;
                    };
                }
                Status::OK
            }
            Status::Not_Found => delete_status
        };
        return result;
    }

    fn print(&self, buf: Vec<(K, V)>) -> Vec<(K, V)> {
        //最小値をもつ節を辿る
        self.node[0].borrow().print(buf)
    }

    fn find(&self, key: K) -> Option<(K, V)> {
        let mut node_address = None;
        for i in 1..M {
            let prev = match self.keys.get(i - 1) {
                Some(prev) => prev <= &key,
                None => true
            };
            let now = match self.keys.get(i) {
                Some(now) => &key < now,
                None => true
            };

            if prev && now {
                node_address = Some(i);
                break;
            } else {
                if prev {
                    if let None = &self.keys.get(i + 1) {
                        node_address = Some(i + 1);
                        break;
                    }
                } else if now {
                    node_address = Some(i - 1);
                    break;
                }
            }
        }
        self.node[node_address.unwrap()].clone().borrow().find(key)
    }

    fn take_items(&mut self) -> Rc<RefCell<dyn Node<K, V>>> {
        let mut taken_keys = Vec::new();
        let mut taken_node = Vec::new();
        while let Some(key) = self.keys.pop() {
            taken_keys.push(key);
        }
        while let Some(node) = self.node.pop() {
            taken_node.push(node);
        }
        taken_keys.sort();
        taken_node.sort_by_key(|a| a.clone().borrow().cmp_key());

        Rc::new(RefCell::new(InternalNode { keys: taken_keys, node: taken_node }))
    }

    fn cmp_key(&self) -> K {
        self.keys[0]
    }

    fn insert(&mut self, k: K, v: V) -> Option<(K, Rc<RefCell<dyn Node<K, V>>>)> {
        self.node.sort_by_key(|a| a.borrow().cmp_key());
        let mut node_address = None;
//        insertする節を決定する
        for (i, comp_key) in self.keys.iter().enumerate() {
            if comp_key <= &k {
                node_address = Some(self.keys.len());
                continue;
            } else {
                node_address = Some(i);
                break;
            }
        }
        let split_info = self.node[node_address.unwrap_or(0)].borrow_mut().insert(k, v.clone());
        if let Some((new_key, new_node)) = split_info {
            self.node.push(new_node);
            self.keys.push(new_key);
            self.keys.sort();
        }
        if self.keys.len() >= M {
            self.node.sort_by_key(|a| a.borrow().cmp_key());
            return Some(self.split());
        }
        None
    }

    fn split(&mut self) -> (K, Rc<RefCell<dyn Node<K, V>>>) {
        let mut new_key = Vec::new();
        let mut new_node = Vec::new();
        let return_key = self.keys.remove(M / 2);
        self.keys.sort();
        self.node.sort_by_key(|a| a.clone().borrow().cmp_key());
        for _ in (M / 2)..self.keys.len() {
            new_key.push(self.keys.pop().unwrap());
        }
        for _ in (M / 2) + 1..self.node.len() {
            new_node.push(self.node.pop().unwrap())
        }
        new_key.sort();
        new_node.sort_by_key(|a| a.clone().borrow().cmp_key());
        let new_tree = Rc::new(RefCell::new(InternalNode { keys: new_key, node: new_node }));
        (return_key, new_tree.clone())
    }

    fn reorg_key(&mut self) -> K {
        let address = self.keys.len() / 2;
        let result = self.keys.remove(address);
        println!("{:?}", self);
        self.keys.push(self.node[address + 1].borrow_mut().reorg_key());
        self.keys.sort();
        return result;
    }
}

impl<K, V> Eq for InternalNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static {}

impl<K, V> PartialEq for InternalNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn eq(&self, other: &Self) -> bool {
        self.node[0].clone().borrow().cmp_key() == other.node[0].clone().borrow().cmp_key()
    }
}

impl<K, V> PartialOrd for InternalNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.node[0].clone().borrow().cmp_key().cmp(&other.node[0].clone().borrow().cmp_key()))
    }
}

impl<K, V> Ord for InternalNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.node[0].clone().borrow().cmp_key().cmp(&other.node[0].clone().borrow().cmp_key())
    }
}

impl<K, V> InternalNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn new() -> Self {
        InternalNode {
            keys: Vec::new(),
            node: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    OK,
    OK_REMOVE,
    OK_NEED_REORG,
    Not_Found,
}

impl<K, V> Node<K, V> for LeafNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn is_InternalNode(&self) -> bool {
        TypeId::of::<InternalNode<K, V>>() == self.type_id()
    }

    fn is_LeafNode(&self) -> bool {
        TypeId::of::<LeafNode<K, V>>() == self.type_id()
    }

    fn marge(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<LeafNode<K, V>>>>>(Box::new(from));
            while let Some(elem) = from.borrow_mut().data.pop() {
                self.data.push(elem);
            }
            self.next_leaf = from.borrow_mut().next_leaf.take();
        }
        true
    }

    fn share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<LeafNode<K, V>>>>>(Box::new(from));
            let len = from.borrow().data.len().clone();
            if M <= self.data.len() + len {
                for i in 0..(M / 2) {
                    self.data.push(from.borrow_mut().data.remove(i.clone()));
                }
                true
            } else { false }
        }
    }

    fn delete(&mut self, key: K) -> Status {
        for i in 0..self.data.len() {
            if self.data[i].0 == key {
                self.data.remove(i);
                let threshold = (M / 2) - 1;
                let status = if 0 == self.data.len() {
                    Status::OK_REMOVE
                } else if threshold == self.data.len() {
                    Status::OK_NEED_REORG
                } else {
                    Status::OK
                };
                return status;
            }
        }
        Status::Not_Found
    }

    fn print(&self, mut buf: Vec<(K, V)>) -> Vec<(K, V)> {
        let mut s = self.data.as_slice().clone();
        buf.append(&mut s.to_vec());
        if let Some(nl) = &self.next_leaf {
            nl.borrow().print(buf)
        } else { buf }
    }

    fn find(&self, key: K) -> Option<(K, V)> {
        for i in 0..self.data.len() {
            if key == self.data[i].0 {
                return Some(self.data[i].clone());
            }
        }
        None
    }

    fn take_items(&mut self) -> Rc<RefCell<dyn Node<K, V>>> {
        let mut taken_data = Vec::new();
        while let Some(data) = self.data.pop() {
            taken_data.push(data);
        }
        taken_data.sort_by_key(|a| a.0);
        Rc::new(RefCell::new(LeafNode { data: taken_data, next_leaf: self.next_leaf.take() }))
    }

    fn cmp_key(&self) -> K {
        self.data[0].0
    }

    fn insert(&mut self, k: K, v: V) -> Option<(K, Rc<RefCell<dyn Node<K, V>>>)> {
        self.data.push((k.clone(), v));
        self.data.sort_by(|prev, next| prev.0.cmp(&next.0));
        if self.data.len() >= M {
            return Some(self.split());
        }
        None
    }

    fn split(&mut self) -> (K, Rc<RefCell<dyn Node<K, V>>>) {
        let mut new_data: Vec<(K, V)> = Vec::new();
        let return_key = self.data[M / 2].0.clone();
        for _ in (M / 2)..M {
            let data = self.data.remove(self.data.len() - 1);
            new_data.push(data);
        }
        self.data.sort_by_key(|k_v| k_v.0);
        new_data.sort_by_key(|k_v| k_v.0);
        let new_leaf = Rc::new(RefCell::new(LeafNode { data: new_data, next_leaf: self.next_leaf.take() }));
        self.next_leaf = Some(new_leaf.clone());
        (return_key, new_leaf)
    }

    fn reorg_key(&mut self) -> K {
        self.data[0].0
    }
}

impl<K, V> Eq for LeafNode<K, V>
    where K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static {}

impl<K, V> PartialEq for LeafNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn eq(&self, other: &Self) -> bool {
        self.data[0].0 == other.data[0].0
    }
}

impl<K, V> PartialOrd for LeafNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.data[0].0.cmp(&other.data[0].0))
    }
}

impl<K, V> Ord for LeafNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.data[0].0.cmp(&other.data[0].0)
    }
}

impl<K, V> LeafNode<K, V>
    where
        K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static,
        V: PartialEq + Eq + Clone + Debug + 'static
{
    fn new() -> Self {
        LeafNode {
            data: Vec::new(),
            next_leaf: None,
        }
    }
}

