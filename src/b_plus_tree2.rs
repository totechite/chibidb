use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::option::Option;
use std::cell::{RefCell};
use std::ops::Deref;
use std::any::{Any, TypeId};


#[cfg(test)]
mod test {
    use crate::b_plus_tree2::BPlusTree;
    use rand::Rng;

    #[test]
    fn test() {
        let mut btree = BPlusTree::new();
        let mut rng = rand::thread_rng();

        for n in 0..100usize {
            let u8b: u32 = rng.gen();
            btree.insert(u8b as usize, n);
        }

        let mut data = btree.to_vec()[0];
        for i in 0..10 {
            data = btree.to_vec()[0];
            btree.remove(data.0);
        }
        assert!(btree.to_vec().is_sorted());
    }
}

const M: usize = 4;

enum DeleteAfterTask {
    Redistribute,
    Merge,

    Reorganize,
}

type K = usize;
type V = usize;

struct BPlusTree
{
    root: Option<Rc<RefCell<dyn Node>>>
}


struct InternalNode
{
    keys: Vec<K>,
    children: Vec<Rc<RefCell<dyn Node>>>,
}

#[derive(Clone)]
struct LeafNode
 {
    data: Vec<(K, V)>,
    prev_node: Weak<RefCell<LeafNode>>,
    next_node: Weak<RefCell<LeafNode>>,

}

trait Node
{
    fn has_children_volume(&self) -> usize;
    fn tree_search(&self, search_key: K) -> Option<(K, V)>;
    fn insert(&mut self, key: K, value: V) -> Option<(K, Rc<RefCell<dyn Node>>)>;
    fn delete(&mut self, key: K) -> Option<DeleteAfterTask>;
    fn aux_to_vec(&self, v: Vec<(K, V)>) -> Vec<(K, V)>;
    fn redistribute(&mut self, node: Rc<RefCell<dyn Node>>) -> K;
    fn marge(&mut self, node: Rc<RefCell<dyn Node>>);
    fn isInternalNode(&self) -> bool;
    fn aux_delete(&mut self, mut child: Rc<RefCell<dyn Node>>, delete_after_task: DeleteAfterTask) -> Option<K> {
        match delete_after_task {
            DeleteAfterTask::Redistribute => {
                let new_key = self.redistribute(child);
                Some(new_key)
            }
            DeleteAfterTask::Merge => {
                self.marge(child);
                None
            }
            DeleteAfterTask::Reorganize => {
                unsafe {
                    let internal_node = std::mem::transmute::<Box<Rc<RefCell<dyn Node>>>, Box<Rc<RefCell<InternalNode>>>>(Box::new(child));
                    return if (M / 2) + 1 < internal_node.borrow().children.len() {
                        self.aux_delete(*internal_node, DeleteAfterTask::Redistribute)
                    } else {
                        self.aux_delete(*internal_node, DeleteAfterTask::Merge)
                    };
                }
            }
        }
    }
    fn get_min_key(&self) -> K;
}

impl BPlusTree

{
    pub fn new() -> Self {
        BPlusTree { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(mut root) = self.root.take() {
            let expect_split = root.borrow_mut().insert(key, value);
            if let Some((key, right_node)) = expect_split {
                let new_child = InternalNode { keys: vec![key], children: vec![root, right_node] };
                self.root = Some(Rc::new(RefCell::new(new_child)));
            } else {
                self.root = Some(root);
            }
        } else {
            self.root = Some(Rc::new(RefCell::new(LeafNode { data: vec![(key, value)], prev_node: Weak::new(), next_node: Weak::new() })));
        }
    }

    pub fn find(&self, search_key: K) -> Option<(K, V)> {
        match &self.root {
            Some(root) => root.borrow().tree_search(search_key),
            None => None
        }
    }

    pub fn remove(&mut self, key: K) -> bool {
        match &self.root {
            Some(root) => {
                match root.clone().borrow_mut().delete(key) {
                    Some(delete_status) => {
                        if root.borrow().isInternalNode() {
                            if 1 == root.borrow().has_children_volume() {
                                unsafe {
                                    let expect_internal = std::mem::transmute::<Box<Rc<RefCell<dyn Node>>>, Box<Rc<RefCell<InternalNode>>>>(Box::new(root.clone()));
                                    self.root = Some(expect_internal.borrow_mut().children[0].clone());
                                }
                            }
                        }
                        true
                    }
                    None => false
                }
            }
            None => false
        }
    }

    pub fn to_vec(&self) -> Vec<(K, V)> {
        return if let Some(root) = &self.root {
            root.borrow().aux_to_vec(vec![])
        } else {
            vec![]
        };
    }
}


impl Node for InternalNode
{
    fn has_children_volume(&self) -> usize {
        self.children.len()
    }

    fn tree_search(&self, search_key: K) -> Option<(K, V)> {
        if search_key < self.keys[0] {
            return self.children[0].borrow().tree_search(search_key);
        } else {
            if let Some(&max_size_key) = self.keys.last() {
                if search_key >= max_size_key {
                    return self.children.last().unwrap().borrow().tree_search(search_key);
                } else {
                    for i in 0..self.keys.len() {
                        if self.keys[i] < search_key && search_key <= self.keys[i + 1] {
                            return self.children[i + 1].borrow().tree_search(search_key);
                        }
                    }
                }
            }
        }
        None
    }

    fn insert(&mut self, key: K, value: V) -> Option<(K, Rc<RefCell<dyn Node>>)> {
        if key < self.keys[0] {
            let new_child_node = self.children[0].borrow_mut().insert(key, value);
            return self.aux_insert(0, new_child_node);
        }
        for i in 0..self.keys.len() {
            if let Some(&next) = self.keys.get(i + 1) {
                if self.keys[i] <= key && key < next {
                    let new_child_node = self.children[i + 1].borrow_mut().insert(key, value);
                    return self.aux_insert(i + 1, new_child_node);
                };
            } else {
                if self.keys[i] <= key { // for a case that self.keys.len() is 1
                    let new_child_node = self.children[i + 1].borrow_mut().insert(key, value);
                    return self.aux_insert(i + 1, new_child_node);
                };
            }
        }
        panic!()
    }


    fn delete(&mut self, key: K) -> Option<DeleteAfterTask> {
        let (mut to, mut from, mut deleted_child_idx) = (None, None, None);
        if key < self.keys[0] {
            to = Some(0);
            from = Some(1);
            deleted_child_idx = Some(0);
        }
        for i in 0..self.keys.len() {
            if let (None, None) = (to, from) {
                if let Some(&next) = self.keys.get(i + 1) {
                    if self.keys[i] < key && key <= next {
                        to = Some(i);
                        from = Some(i + 1);
                        deleted_child_idx = Some(i + 1);
                    }
                } else {
                    if self.keys[i] <= key { // for a case that self.keys.len() is 1
                        to = Some(i - 1);
                        from = Some(i);
                        deleted_child_idx = Some(i + 1);
                    };
                }
            } else { break; }
        }
        if let (Some(to), Some(from), Some(deleted_child_idx)) = (to, from, deleted_child_idx) {
            let delete_status = self.children[deleted_child_idx].borrow_mut().delete(key);
            if let Some(delete_status) = delete_status {
                let redistoributed_key = self.children[to].borrow_mut().aux_delete(self.children[from].clone(), delete_status);
                if let Some(key) = redistoributed_key {
                    self.keys[from - 1] = key;
                } else {
                    self.keys.remove(from - 1);
                    self.children.remove(from);
                }

                if (M / 2) > self.children.len() {
                    return Some(DeleteAfterTask::Reorganize);
                }

                None
            } else { None }
        } else { panic!(); }
    }

    fn aux_to_vec(&self, v: Vec<(K, V)>) -> Vec<(K, V)> {
        self.children[0].borrow().aux_to_vec(v)
    }

    fn redistribute(&mut self, node: Rc<RefCell<dyn Node>>) -> K {
        unsafe {
            let defrayer_node = std::mem::transmute::<Box<Rc<RefCell<dyn Node>>>, Box<Rc<RefCell<InternalNode>>>>(Box::new(node));
            self.keys.append(&mut defrayer_node.borrow_mut().keys);
            self.children.append(&mut defrayer_node.borrow_mut().children);
            let defrayer_keys = self.keys.drain((M / 2) - 1..).collect::<Vec<_>>();
            let defrayer_children = self.children.drain(M / 2..).collect::<Vec<_>>();
            defrayer_node.borrow_mut().keys = defrayer_keys;
            defrayer_node.borrow_mut().children = defrayer_children;
            let key = defrayer_node.borrow().get_min_key();
            key
        }
    }

    fn marge(&mut self, mut node: Rc<RefCell<dyn Node>>) {
        unsafe {
            let sacrifice_node = std::mem::transmute::<Box<Rc<RefCell<dyn Node>>>, Box<Rc<RefCell<InternalNode>>>>(Box::new(node));
            let new_key = sacrifice_node.borrow().get_min_key();
            self.keys.push(new_key);
            self.keys.append(&mut sacrifice_node.clone().borrow_mut().keys);
            self.children.append(&mut sacrifice_node.clone().borrow_mut().children);
        }
    }

    fn isInternalNode(&self) -> bool {
        true
    }

    fn get_min_key(&self) -> K {
        self.children[0].borrow().get_min_key()
    }
}


impl InternalNode
{
    fn aux_insert(&mut self, i: usize, new_child_node: Option<(K, Rc<RefCell<dyn Node>>)>) -> Option<(K, Rc<RefCell<dyn Node>>)> {
        return if let Some((k, new_child)) = new_child_node {
            self.keys.insert(i, k);
            self.children.insert(i + 1, new_child);

            if self.children.len() > M {
                let (mut left_part, right_part) = self.split();
                let key = left_part.keys.pop().unwrap();
                self.keys = left_part.keys;
                self.children = left_part.children;
                return Some((key, Rc::new(RefCell::new(right_part))));
            }
            None
        } else { None };
    }

    fn split(&mut self) -> (Self, Self) {
        let (left_part, right_part) = {
            let left_part = self.children.drain(..M / 2).collect::<Vec<_>>();
            let right_part = self.children.drain(..).collect::<Vec<_>>();
            (left_part, right_part)
        };
        let (left_key, right_key) = {
            let (left_key, right_key) = self.keys.split_at(M / 2);
            (left_key.to_vec(), right_key.to_vec())
        };

        (InternalNode { keys: left_key, children: left_part }, InternalNode { keys: right_key, children: right_part })
    }
}

impl Node for LeafNode
    where
        K: Ord + Clone + 'static,
        V: 'static
{
    fn has_children_volume(&self) -> usize {
        self.data.len()
    }
    fn tree_search(&self, search_key: K) -> Option<(K, V)> {
        for (k, v) in &self.data {
            if &search_key == k {
                return Some((k.clone(), v.clone()));
            }
        };
        None
    }
    fn insert(&mut self, key: K, value: V) -> Option<(K, Rc<RefCell<dyn Node>>)> {
        self.data.push((key, value));
        self.data.sort_by_key(|(k, _)| k.clone());
        return if M >= self.data.len() {
            None
        } else {
            let (left_part, mut right_part) = self.into_data_split();

            self.data = left_part;
            let mut new_leaf = Rc_RefCell(LeafNode { data: right_part, prev_node: Weak::new(), next_node: Weak::new() });
            let new_key = new_leaf.clone().borrow().data[0].0;

            new_leaf.borrow_mut().prev_node = Rc::downgrade(&Rc_RefCell(self.deref().clone()).clone()).clone();
            new_leaf.borrow_mut().next_node = self.next_node.clone();
            self.next_node = Rc::downgrade(&new_leaf.clone());

            Some((new_key, new_leaf))
        };
    }

    fn delete(&mut self, key: K) -> Option<DeleteAfterTask> {
        if let Some(idx) = self.data.iter().position(|(k, _)| &key == k) {
            self.data.remove(idx);

            if M / 2 <= self.data.len() {
                None
            } else {
// もし隣接ノードのデータが(最大保持数の半分)＋１個以上ならば再分配。そうでなければマージする

                if let Some(next) = self.next_node.upgrade() {
                    return if (M / 2) + 1 < next.borrow().data.len() {
                        Some(DeleteAfterTask::Redistribute)
                    } else {
                        Some(DeleteAfterTask::Merge)
                    };
                }

                if let Some(prev) = self.prev_node.upgrade() {
                    return if (M / 2) + 1 < prev.borrow().data.len() {
                        Some(DeleteAfterTask::Redistribute)
                    } else {
                        Some(DeleteAfterTask::Merge)
                    };
                }

                panic!()
            }
        } else {
            None
        }
    }

    fn aux_to_vec(&self, mut v: Vec<(K, V)>) -> Vec<(K, V)> {
        let mut data = self.data.as_slice().to_vec();
        v.append(&mut data);
        if let Some(next) = &self.next_node.upgrade() {
            next.borrow().aux_to_vec(v)
        } else {
            v
        }
    }

    fn redistribute(&mut self, node: Rc<RefCell<dyn Node>>) -> K {
        unsafe {
            let defrayer_node = std::mem::transmute::<Box<Rc<RefCell<dyn Node>>>, Box<Rc<RefCell<LeafNode>>>>(Box::new(node));
            self.data.append(&mut defrayer_node.borrow_mut().data);
            let defrayer_date = self.data.drain(M / 2..).collect::<Vec<_>>();
            defrayer_node.borrow_mut().data = defrayer_date;
            let key = defrayer_node.borrow_mut().data[0].0;
            key
        }
    }

    fn marge(&mut self, node: Rc<RefCell<dyn Node>>) {
        unsafe {
            let sacrifice_node = std::mem::transmute::<Box<Rc<RefCell<dyn Node>>>, Box<Rc<RefCell<LeafNode>>>>(Box::new(node));
            self.data.append(&mut sacrifice_node.clone().borrow_mut().data);
            self.next_node = sacrifice_node.borrow_mut().next_node.clone();
        }
    }

    fn get_min_key(&self) -> K {
        self.data[0].0
    }

    fn isInternalNode(&self) -> bool {
        false
    }
}

impl LeafNode
    where
        K: Ord + Clone {
    fn into_data_split(&mut self) -> (Vec<(K, V)>, Vec<(K, V)>) {
        let left_part = self.data.drain(..(M / 2)).collect::<Vec<_>>();
        let right_part = self.data.drain(..).collect::<Vec<_>>();
        (left_part, right_part)
    }
}

fn Rc_RefCell(v: LeafNode) -> Rc<RefCell<LeafNode>>
    where
        K: Ord + Clone {
    Rc::new(RefCell::new(v))
}


