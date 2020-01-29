use std::fmt::Debug;
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::any::{Any, TypeId};
use std::ops::Deref;

const M: usize = 6;

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
    fn right_share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool;
    fn left_share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool;
    fn split(&mut self) -> (K, Rc<RefCell<dyn Node<K, V>>>);
    fn cmp_key(&self) -> K;
    fn take_items(&mut self) -> Rc<RefCell<dyn Node<K, V>>>;
    fn reorg_key(&mut self, delete_key_locate: usize) -> Option<K>;
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
        println!("{:?}", new_root);
        new_root.node.sort_by_key(|a| a.borrow().cmp_key());
        self.root_node = Box::new(new_root);
    }

    pub fn delete(&mut self, key: K) -> bool {
       match self.root_node.delete(key){
           Status::Not_Found=> false,
           _ => true
       }
    }
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
            while let Some(elem) = from.borrow_mut().keys.pop() {
                if !self.keys.contains(&elem) {
                    self.keys.push(elem);
                };
            }
            self.node.sort_by_key(|a| a.borrow().cmp_key());
            self.keys.sort();
        }
        true
    }

    fn right_share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<InternalNode<K, V>>>>>(Box::new(from));
            let len = from.borrow().node.len().clone();
            if M <= (self.node.len() + len) {
                for i in 0..(self.node.len() + len) / 2 - self.node.len() {
                    self.node.push(from.borrow_mut().node.pop().unwrap())
                };
                self.node.sort_by_key(|a| a.borrow().cmp_key());
                true
            } else { false }
        }
    }

    fn left_share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<InternalNode<K, V>>>>>(Box::new(from));
            let len = from.borrow().node.len().clone();
            if M <= (self.node.len() + len) {
                for i in 0..(self.node.len() + len) / 2 - self.node.len() {
                    self.node.push(from.borrow_mut().node.remove(0))
                };
                self.node.sort_by_key(|a| a.borrow().cmp_key());
                true
            } else { false }
        }
    }
    fn delete(&mut self, key: K) -> Status {
        self.node.sort_by_key(|a| a.borrow().cmp_key());
        let mut node_address = {
            let mut result = self.keys.len();
            for (i, comp_key) in self.keys.iter().enumerate() {
                if &key < comp_key {
                    result = i;
                    break;
                };
            }
            result
        };
        println!("{:?}", &key);
        println!("{:?}", &self.keys);
        println!("{:?}", &node_address);

        let delete_status = self.node[node_address].borrow_mut().delete(key);
        let mut reorg_key_frag = None;
        for (i, k) in self.keys.iter().enumerate() {
            if &key == k {
                reorg_key_frag = Some(i);
            }
        }
        if let Some(i) = reorg_key_frag {
            self.reorg_key(i);
        }
        println!("{:?}", &delete_status);

        let result = match delete_status {
            Status::Ok_REORG_LEFT =>
                {
                    let tmp = self.node.remove(node_address - 1);
                    println!("{:?}", self);
                    if M - 1 <= self.keys.len() {
                        self.node[node_address].borrow_mut().marge(tmp);
                    } else {
                        self.marge(tmp);
//                        if self.keys.len() >= M {
//                            self.node.sort_by_key(|a| a.borrow().cmp_key());
//                            let left = self.split();
//                            self.node.push(left.1);
//                        }
                    }
                    Status::OK
                }
            Status::OK_REORG_RIGHT =>
                {
                    let tmp = self.node.remove(node_address + 1);
                    println!("{:?}", self);

                    if M - 1 <= self.keys.len() {
                        self.node[node_address].borrow_mut().marge(tmp);
                    } else {
                        self.marge(tmp);
                    }
                    Status::OK
                }
            Status::OK_REORG =>
                {
                    if self.node[node_address].borrow().is_InternalNode() {
                        if (self.node.len() - 1) == node_address {
                            // 対象が最右の場合
                            println!("{:?}", &node_address);
                            unsafe {
                                let right = self.node.remove(node_address);
                                let left = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<InternalNode<K, V>>>>>(Box::new(self.node.remove(node_address - 1)));
                                left.borrow_mut().marge(right);
                                println!("{:?}", &left);
                                println!("{:?}", &self);

                                if M - 1 <= self.keys.len() {
                                    self.node.push(*left.clone());
                                } else {
                                    if left.borrow().keys.len() >= M {
                                        self.node.sort_by_key(|a| a.borrow().cmp_key());
                                        let sp = left.borrow_mut().split();
                                        self.node.push(sp.1);
                                        self.keys.remove(node_address - 1);
                                        self.keys.push(sp.0);
                                        self.node.push(*left.clone());
                                    } else {
                                        self.marge(*left.clone());
                                    }
                                }
                                self.key_conflict_resolver(*left);
                                self.node.sort_by_key(|a| a.borrow().cmp_key());
                            }
                            return Status::Ok_REORG_LEFT;
                        } else {
                            let right = self.node.remove(node_address + 1);
                            let mut tmp = InternalNode::new();
                            tmp.marge(right);
                            tmp.keys.push(tmp.node[0].borrow().cmp_key());
                            println!("{:?}", &tmp);

                            unsafe {
                                let mut left = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<InternalNode<K, V>>>>>(Box::new(self.node.remove(node_address)));
                                while let Some(elem) = left.borrow_mut().node.pop() {
                                    tmp.node.push(elem);
                                }
                                tmp.node.sort_by_key(|a| a.borrow().cmp_key());
                                tmp.keys.sort();
                                println!("{:?}", &tmp);
                            }
                            let tmp = Rc::new(RefCell::new(tmp));
                            if M - 1 <= self.keys.len() {
                                self.node.push(tmp.clone());
                            } else {
                                self.marge(tmp.clone());
                            }
                            self.key_conflict_resolver(tmp);
                            self.node.sort_by_key(|a| a.borrow().cmp_key());
                            return Status::OK_REORG_RIGHT;
                        }
                    } else {
                        println!("{:?}", self);
                        println!("{:?}", node_address);
                        let shared_node = if (self.node.len() - 1) == node_address { node_address - 1 } else { node_address + 1 };
                        let share_done = if (self.node.len() - 1) == node_address { self.node[node_address].borrow_mut().right_share(self.node[shared_node].clone()) } else { self.node[node_address].borrow_mut().left_share(self.node[shared_node].clone()) };
                        if !share_done {
                            if (self.node.len() - 1) == node_address {
//                               対象が最右の場合、左隣りにmargeする
                                self.node[node_address - 1].borrow_mut().marge(self.node[node_address].clone());
                                self.node.remove(node_address);
                            } else {
//                              右隣にmargeする
                                self.node[node_address].borrow_mut().marge(self.node[node_address + 1].clone());
                                self.node.remove(node_address + 1);
                                self.keys.remove(node_address);
                            }
                        } else {
                            self.node.sort_by_key(|a| a.borrow().cmp_key());

                            println!("{:?}", self);
                            self.keys.remove(if (self.node.len() - 1) == node_address { node_address - 1 } else { node_address });
                            self.keys.push(self.node[if (self.node.len() - 1) == node_address { node_address } else { node_address + 1 }].borrow().cmp_key());
                            self.keys.sort();
                            println!("{:?}", self);
                            println!("{:?}", self);
                        }
                    };
                    self.node.sort_by_key(|a| a.borrow().cmp_key());
                    if (M / 2) - 1 >= self.node.len() || self.node.len() == 1 {
                        return Status::OK_REORG;
                    }
                    return Status::OK;
                }
            Status::OK => delete_status,
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

    fn reorg_key(&mut self, delete_key_locate: usize) -> Option<K> {
        let node_locate = self.keys.len();
        let result = self.keys.remove(delete_key_locate);
        if let Some(key) = self.node[self.node.len() - 1].borrow_mut().reorg_key(0) {
            if !self.keys.contains(&key) {
                self.keys.push(key);
            };
        } else {
            let key = self.node[self.node.len() - 2].borrow_mut().reorg_key(0).unwrap();
            if !self.keys.contains(&key) {
                self.keys.push(key);
            };
        }
        self.keys.sort();
        return Some(result);
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

    fn key_conflict_resolver(&mut self, child_node: Rc<RefCell<dyn Node<K, V>>>) {
        if child_node.borrow().is_InternalNode() {
            unsafe {
                let internal = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<InternalNode<K, V>>>>>(Box::new(child_node));
                for (i, key) in self.keys.iter().enumerate() {
                    if internal.borrow().keys.contains(key) {
                        self.keys.remove(i);
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Status {
    OK,
    OK_REORG,
    Ok_REORG_LEFT,
    OK_REORG_RIGHT,
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
            self.data.sort_by_key(|a| a.0);
            self.next_leaf = from.borrow_mut().next_leaf.take();
        }
        true
    }

    fn right_share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<LeafNode<K, V>>>>>(Box::new(from));
            let len = from.borrow().data.len().clone();
            if 1 != (self.data.len() + len) && (M / 2) + 1 <= self.data.len() + len {
                for i in 0..(self.data.len() + len) / 2 - self.data.len() {
                    self.data.push(from.borrow_mut().data.pop().unwrap());
                }
                self.data.sort_by_key(|a| a.0);
                true
            } else { false }
        }
    }

    fn left_share(&mut self, from: Rc<RefCell<dyn Node<K, V>>>) -> bool {
        unsafe {
            let mut from = std::mem::transmute::<Box<Rc<RefCell<dyn Node<K, V>>>>, Box<Rc<RefCell<LeafNode<K, V>>>>>(Box::new(from));
            let len = from.borrow().data.len().clone();
            if 1 != (self.data.len() + len) && (M / 2) + 1 <= self.data.len() + len {
                for i in 0..(self.data.len() + len) / 2 - self.data.len() {
                    self.data.push(from.borrow_mut().data.remove(0));
                }
                self.data.sort_by_key(|a| a.0);
                true
            } else { false }
        }
    }

    fn delete(&mut self, key: K) -> Status {
        for i in 0..self.data.len() {
            if self.data[i].0 == key {
                self.data.remove(i);
                let threshold = (M / 2) - 1;
                let status = if threshold >= self.data.len() || self.data.len() == 0 {
                    Status::OK_REORG
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

    fn reorg_key(&mut self, delete_key_locate: usize) -> Option<K> {
        if let Some(k_v) = self.data.get(0) {
            Some(k_v.0)
        } else { None }
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

