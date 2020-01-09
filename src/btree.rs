use std::fmt::Debug;
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Btree<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> {
    pub root_node: Box<dyn Node<K, V>>,
}

const M: usize = 3;

pub trait Node<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static>: Debug {
    fn find(&self, key: K) -> Option<(K, V)>;
    fn insert(&mut self, k: K, v: V) -> Option<(K, Rc<RefCell<dyn Node<K, V>>>)>;
    fn split(&mut self) -> (K, Rc<RefCell<dyn Node<K, V>>>);
    fn cmp_key(&self) -> K;
    fn take_items(&mut self) -> Rc<RefCell<dyn Node<K, V>>>;
}

#[derive(Debug)]
pub struct TreeNode<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> {
    keys: Vec<K>,
    node: Vec<Rc<RefCell<dyn Node<K, V>>>>,
}

#[derive(Default, Clone, Debug)]
struct LeafNode<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> {
    data: Vec<(K, V)>,
    next_leaf: Option<Rc<RefCell<LeafNode<K, V>>>>,
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> Btree<K, V> {
    pub fn new() -> Self {
        Btree {
            root_node: Box::new(LeafNode::new()),
        }
    }

    pub fn print(&self) -> Vec<(K, V)> {
        self.print()
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
        let mut new_root = TreeNode::new();
        new_root.keys.push(new_key);
        new_root.node.push(new_node);
        new_root.node.push(self.root_node.take_items());
        new_root.keys.sort();
        new_root.node.sort_by_key(|a| a.clone().borrow().cmp_key());
        self.root_node = Box::new(new_root);
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> Node<K, V> for TreeNode<K, V> {
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

        Rc::new(RefCell::new(TreeNode { keys: taken_keys, node: taken_node }))
    }

    fn cmp_key(&self) -> K {
        self.keys[0]
    }

    fn insert(&mut self, k: K, v: V) -> Option<(K, Rc<RefCell<dyn Node<K, V>>>)> {
        for i in 1..M {
            let prev = match self.keys.get(i - 1) {
                Some(prev) => prev <= &k,
                None => false
            };
            let now = match self.keys.get(i) {
                Some(now) => &k < now,
                None => false
            };

            if i == 1 {
                if let None = self.keys.get(1) {
                    if !prev {
                        if let Some((new_key, new_node)) = self.node[i - 1].clone().borrow_mut().insert(k, v.clone()) {
                            self.node.push(new_node);
                            self.keys.push(new_key);
                            self.keys.sort();
//                            self.node.sort_by_key(|a| a.clone().borrow().cmp_key());
                        };
                    }
                }
            }

            self.node.sort_by_key(|a| a.clone().borrow().cmp_key());

            if !prev && now {
                if let Some((new_key, new_node)) = self.node[i - 1].clone().borrow_mut().insert(k, v.clone()) {
                    self.node.push(new_node);
                    self.keys.push(new_key);
                    self.keys.sort();
//                    self.node.sort_by_key(|a| a.clone().borrow().cmp_key());
                };
            } else if prev && !now {
                if let None = self.keys.get(i) {
                    if let Some((new_key, new_node)) = self.node[i].clone().borrow_mut().insert(k, v.clone()) {
                        self.node.push(new_node);
                        self.keys.push(new_key);
                        self.keys.sort();
//                        self.node.sort_by_key(|a| a.clone().borrow().cmp_key());
                    };
                } else { continue; }
            } else if prev && now {
                if let Some((new_key, new_node)) = self.node[i].clone().borrow_mut().insert(k, v.clone()) {
                    self.node.push(new_node);
                    self.keys.push(new_key);
                    self.keys.sort();
//                    self.node.sort_by_key(|a| a.clone().borrow().cmp_key());
                } else { continue; }
            } else { continue; };

            if self.keys.len() >= M {
                self.node.sort_by_key(|a| a.clone().borrow().cmp_key());
                return Some(self.split());
            } else {
                break;
            };
        }
        None
    }

    fn split(&mut self) -> (K, Rc<RefCell<dyn Node<K, V>>>) {
        let mut new_key = Vec::new();
        let mut new_node = Vec::new();
        let return_key = self.keys.remove(M / 2);
        self.keys.sort();
        self.node.sort_by_key(|a| a.clone().borrow().cmp_key());
        for _ in (self.keys.len() / 2)..self.keys.len() {
            new_key.push(self.keys.pop().unwrap());
        }
        for _ in (self.node.len() / 2)..self.node.len() {
            new_node.push(self.node.pop().unwrap())
        }
        new_key.sort();
        new_node.sort_by_key(|a| a.clone().borrow().cmp_key());
        let new_tree = Rc::new(RefCell::new(TreeNode { keys: new_key, node: new_node }));
        (return_key, new_tree.clone())
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> Eq for TreeNode<K, V> {}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> PartialEq for TreeNode<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.node[0].clone().borrow().cmp_key() == other.node[0].clone().borrow().cmp_key()
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> PartialOrd for TreeNode<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.node[0].clone().borrow().cmp_key().cmp(&other.node[0].clone().borrow().cmp_key()))
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> Ord for TreeNode<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node[0].clone().borrow().cmp_key().cmp(&other.node[0].clone().borrow().cmp_key())
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> TreeNode<K, V> {
    fn new() -> Self {
        TreeNode {
            keys: Vec::new(),
            node: Vec::new(),
        }
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> Node<K, V> for LeafNode<K, V> {
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
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> Eq for LeafNode<K, V> {}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> PartialEq for LeafNode<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.data[0].0 == other.data[0].0
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> PartialOrd for LeafNode<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.data[0].0.cmp(&other.data[0].0))
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> Ord for LeafNode<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data[0].0.cmp(&other.data[0].0)
    }
}

impl<K: PartialEq + PartialOrd + Ord + Copy + Clone + Debug + 'static, V: PartialEq + Eq + Clone + Debug + 'static> LeafNode<K, V> {
    fn new() -> Self {
        LeafNode {
            data: Vec::new(),
            next_leaf: None,
        }
    }
}

