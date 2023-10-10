use std::{cmp::PartialOrd, fmt::Display};

pub trait TreeTrait: Display + PartialOrd {}
impl<T> TreeTrait for T where T: Display + PartialOrd {}

pub struct Tree<T: TreeTrait> {
    data: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T: TreeTrait> Tree<T> {
    pub fn new(val: T) -> Self {
        Tree {
            data: val,
            left: None,
            right: None,
        }
    }

    pub fn get_data(&self) -> &T {
        return &self.data;
    }
}

pub fn insert_node<T: TreeTrait>(node: &mut Tree<T>, data: T) {
    if data < node.data {
        if node.left.is_none() {
            node.left = Some(Box::new(Tree::new(data)));
        } else {
            insert_node(node.left.as_mut().unwrap().as_mut(), data)
        }
    } else if data > node.data {
        if node.right.is_none() {
            node.right = Some(Box::new(Tree::new(data)));
        } else {
            insert_node(node.right.as_mut().unwrap().as_mut(), data);
        }
    }
}

pub fn inorder<T: TreeTrait>(node: &Tree<T>) {
    if node.left.is_some() {
        inorder(node.left.as_ref().unwrap());
    }
    println!("{}", node.get_data());
    if node.right.is_some() {
        inorder(node.right.as_ref().unwrap());
    }
}

pub fn preorder<T: TreeTrait>(node: &Tree<T>) {
    println!("{}", node.get_data());
    if node.left.is_some() {
        inorder(node.left.as_ref().unwrap());
    }
    if node.right.is_some() {
        inorder(node.right.as_ref().unwrap());
    }
}

pub fn postorder<T: TreeTrait>(node: &Tree<T>) {
    println!("{}", node.get_data());
    if node.left.is_some() {
        inorder(node.left.as_ref().unwrap());
    }
    if node.right.is_some() {
        inorder(node.right.as_ref().unwrap());
    }
}

pub fn tree_view<T: TreeTrait>(node: &Tree<T>, padding: usize) {
    if node.left.is_some() {
        tree_view(node.left.as_ref().unwrap(), padding + 1);
    }
    println!("{}{}", "    ".repeat(padding), node.data);
    if node.right.is_some() {
        tree_view(node.right.as_ref().unwrap(), padding + 1);
    }
}
