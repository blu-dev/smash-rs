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

impl<O: PartialOrd, V> TreeNode<O, V> {
    pub fn left(&self) -> Option<&TreeNode<O, V>> {
        if self.left.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.left)
            }
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        if self.left.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.left)
            }
        }
    }

    pub fn right(&self) -> Option<&TreeNode<O, V>> {
        if self.right.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.right)
            }
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        if self.right.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.right)
            }
        }
    }

    pub fn parent(&self) -> Option<&TreeNode<O, V>> {
        if self.parent.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.parent)
            }
        }
    }

    pub fn parent_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        if self.parent.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.parent)
            }
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
            Some(Ordering::Less) => self.left().map(|x| x.get(key)).flatten(),
            Some(Ordering::Equal) => Some(self.value()),
            Some(Ordering::Greater) => self.right().map(|x| x.get(key)).flatten(),
            None => None
        }        
    }

    pub fn get_mut(&mut self, key: &O) -> Option<&mut V> {
        match key.partial_cmp(self.key()) {
            Some(Ordering::Less) => self.left_mut().map(|x| x.get_mut(key)).flatten(),
            Some(Ordering::Equal) => Some(self.value_mut()),
            Some(Ordering::Greater) => self.right_mut().map(|x| x.get_mut(key)).flatten(),
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
        if self.root.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.root)
            }
        }
    }

    fn root_mut(&mut self) -> Option<&mut TreeNode<O, V>> {
        if self.root.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.root)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> KeyValueIter<O, V> {
        KeyValueIter::new(self)
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

pub struct KeyValueIter<'a, O: PartialOrd, V> {
    tree: &'a Tree<O, V>,
    current_node: *const TreeNode<O, V>,
    is_left: bool,
}

impl<'a, O: PartialOrd, V> KeyValueIter<'a, O, V> {
    pub fn new(tree: &'a Tree<O, V>) -> Self {
        Self {
            tree,
            current_node: std::ptr::null(),
            is_left: true
        }
    }
}

impl<'a, O: PartialOrd, V> Iterator for KeyValueIter<'a, O, V> {
    type Item = (&'a O, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_node.is_null() {
            if self.tree.root.is_null() {
                None
            } else {
                self.current_node = self.tree.root;
                self.is_left = true;
                unsafe {
                    Some(((*self.current_node).key(), (*self.current_node).value()))
                }
            }
        } else if self.is_left {
            let next = unsafe {
                (*self.current_node).left()
            };
            match next {
                Some(node) => {
                    self.current_node = node as *const TreeNode<O, V>;
                    Some((node.key(), node.value()))
                },
                None => {
                    self.is_left = false;
                    self.next()
                }
            }
        } else {
            let next = unsafe {
                (*self.current_node).right()
            };
            match next {
                Some(node) => {
                    self.current_node = node as *const TreeNode<O, V>;
                    self.is_left = true;
                    Some((node.key(), node.value()))
                },
                None => {
                    let mut current_node = unsafe {
                        (*self.current_node).parent()
                    };
                    let mut previous = self.current_node;
                    while let Some(current) = current_node {
                        if let Some(parent) = current.parent() {
                            if let Some(right) = parent.right() {
                                if right as *const TreeNode<O, V> != previous {
                                    self.current_node = right as *const TreeNode<O, V>;
                                    self.is_left = true;
                                    return Some((right.key(), right.value()));
                                } else {
                                    previous = parent as *const TreeNode<O, V>;
                                    current_node = parent.parent();
                                }
                            } else {
                                previous = parent as *const TreeNode<O, V>;
                                current_node = parent.parent();
                            }
                        } else {
                            return None;
                        }
                    }
                    None
                }
            }
        }       
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