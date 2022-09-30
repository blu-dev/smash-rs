use std::{cmp::Ordering, ops::{Index, IndexMut}};

#[repr(C)]
pub struct TreeNode<O: PartialOrd, V> {
    right: *mut TreeNode<O, V>,
    left: *mut TreeNode<O, V>,
    parent: *mut TreeNode<O, V>,
    is_black: bool,
    padding: [u8; 7],
    key: O,
    value: V
}

pub struct RecursiveTreeNodeIter<'a, O: PartialOrd, V> {
    node: &'a TreeNode<O, V>,
    done_left: bool,
    done_right: bool,
    done_self: bool,
    current_node: Option<Box<RecursiveTreeNodeIter<'a, O, V>>>
}

impl<'a, O: PartialOrd, V> RecursiveTreeNodeIter<'a, O, V> {
    pub fn new(node: &'a TreeNode<O, V>) -> Self {
        Self {
            node,
            done_left: false,
            done_right: false,
            done_self: false,
            current_node: None
        }
    }
}

impl<'a, O: PartialOrd, V> Iterator for RecursiveTreeNodeIter<'a, O, V> {
    type Item = (&'a O, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done_left {
            if self.current_node.is_none() {
                self.current_node = self.node.left().map(|node| Box::new(RecursiveTreeNodeIter::new(node)));
            }
            match &mut self.current_node {
                Some(current_node) => match current_node.next() {
                    Some(v) => return Some(v),
                    None => {
                        self.done_left = true;
                        self.current_node = None;
                    }
                },
                None => {
                    self.done_left = true;
                }
            }
        }
        if !self.done_self {
            self.done_self = true;
            return Some((self.node.key(), self.node.value()));
        }
        if !self.done_right {
            if self.current_node.is_none() {
                self.current_node = self.node.right().map(|node| Box::new(RecursiveTreeNodeIter::new(node)));
            }
            match &mut self.current_node {
                Some(current_node) => match current_node.next() {
                    Some(v) => return Some(v),
                    None => {
                        self.done_right = true;
                        self.current_node = None;
                    }
                },
                None => {
                    self.done_right = true;
                }
            }
        }
        None
    }
}

impl<O: PartialOrd, V> TreeNode<O, V> {
    pub fn left(&self) -> Option<&TreeNode<O, V>> {
        unsafe {
            self.left.as_ref()
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        unsafe {
            self.left.as_mut()
        }
    }

    pub fn right(&self) -> Option<&TreeNode<O, V>> {
        unsafe {
            self.right.as_ref()
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        unsafe {
            self.right.as_mut()
        }
    }

    pub fn parent(&self) -> Option<&TreeNode<O, V>> {
        unsafe {
            self.parent.as_ref()
        }
    }

    pub fn parent_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        unsafe {
            self.parent.as_mut()
        }
    }

    pub fn is_black(&self) -> bool {
        self.is_black
    }

    pub fn key(&self) -> &O {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn get(&self, key: &O) -> Option<&V> {
        match key.partial_cmp(self.key()) {
            Some(Ordering::Less) => self.right().map(|x| x.get(key)).flatten(),
            Some(Ordering::Equal) => Some(self.value()),
            Some(Ordering::Greater) => self.left().map(|x| x.get(key)).flatten(),
            None => None
        }        
    }

    pub fn get_mut(&mut self, key: &O) -> Option<&mut V> {
        match key.partial_cmp(self.key()) {
            Some(Ordering::Less) => self.right_mut().map(|x| x.get_mut(key)).flatten(),
            Some(Ordering::Equal) => Some(self.value_mut()),
            Some(Ordering::Greater) => self.left_mut().map(|x| x.get_mut(key)).flatten(),
            None => None
        }
    }
}

#[repr(C)]
pub struct Tree<O: PartialOrd, V> {
    end: *mut TreeNode<O, V>,
    root: *mut TreeNode<O, V>,
    length: usize
}

impl<O: PartialOrd, V> Tree<O, V> {
    fn root(&self) -> Option<&TreeNode<O, V>> {
        unsafe {
            self.root.as_ref()
        }
    }

    fn root_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        unsafe {
            self.root.as_mut()
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> TreeKeyValueIter<O, V> {
        TreeKeyValueIter::new(self)
    }

    pub fn get(&self, key: &O) -> Option<&V> {
        self.root()
            .map(|root| root.get(key))
            .flatten()
    }

    pub fn get_mut(&mut self, key: &O) -> Option<&mut V> {
        self.root_mut()
            .map(|root| root.get_mut(key))
            .flatten()
    }
}

pub struct TreeKeyValueIter<'a, O: PartialOrd, V> {
    tree: &'a Tree<O, V>,
    root: Option<RecursiveTreeNodeIter<'a, O, V>>
}

impl<'a, O: PartialOrd, V> TreeKeyValueIter<'a, O, V> {
    pub fn new(tree: &'a Tree<O, V>) -> Self {
        Self {
            tree,
            root: None
        }
    }
}

impl<'a, O: PartialOrd, V> Iterator for TreeKeyValueIter<'a, O, V> {
    type Item = (&'a O, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.root.is_none() {
            self.root = self.tree.root().map(|x| RecursiveTreeNodeIter::new(x));
        }
        self.root
            .as_mut()
            .map(|root| root.next())
            .flatten()
    }
}

impl<O: PartialOrd, V> Index<O> for Tree<O, V> {
    type Output = V;

    fn index(&self, index: O) -> &Self::Output {
        self.get(&index).expect("Failed to find key in tree!")
    }
}

impl<O: PartialOrd, V> IndexMut<O> for Tree<O, V> {
    fn index_mut(&mut self, index: O) -> &mut Self::Output {
        self.get_mut(&index).expect("Failed to find key in tree!")
    }
}