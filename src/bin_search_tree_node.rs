//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Thu, 07 Jan 2021 21:28:49 +0900
extern crate typed_arena;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Display;
use typed_arena::Arena;

pub struct BinSTreeNode<'a, T: Ord>{
    pub value: RefCell<T>,
    pub parent: RefCell<Vec<Node<'a, T>>>,
    pub left_child: RefCell<Vec<Node<'a, T>>>,
    pub right_child: RefCell<Vec<Node<'a, T>>>,
}

type Node<'a, T> = &'a BinSTreeNode<'a, T>;


impl<'a, T: Ord + Display> BinSTreeNode<'a, T>{
    pub fn create_node(value: T) -> BinSTreeNode<'a, T> {
        BinSTreeNode{
            value: RefCell::new(value),
            parent: RefCell::new(vec![]),
            left_child: RefCell::new(vec![]),
            right_child: RefCell::new(vec![]),
        }
    }
}

pub fn add_node<'a, T: Ord + Display>(tree: &Arena<BinSTreeNode<'a, T>>,
                                   root_node: Node<'a, T>,
                                   node: Node<'a, T>){

    match root_node.value.borrow().cmp(&*node.value.borrow()) {
        Ordering::Greater => {
            if root_node.left_child.borrow().is_empty() {
                (*root_node.left_child.borrow_mut()).push(node);
                (*node.parent.borrow_mut()).push(root_node);
            }
            else{
                add_node(tree, root_node.left_child.borrow()[0], node);
            }
        },
        _ => {
            if root_node.right_child.borrow().is_empty() {
                (*root_node.right_child.borrow_mut()).push(node);
                (*node.parent.borrow_mut()).push(root_node);
            }
            else{
                add_node(tree, root_node.right_child.borrow()[0], node);
            }
        },
    }
    
}

pub fn print_node<'a, T: Ord + Display>(root_node: Node<'a, T>, order: usize){
    match order {
        0 => {
            println!("{}", root_node.value.borrow());
            if !root_node.left_child.borrow().is_empty() {
                print_node(root_node.left_child.borrow()[0], order);
            }
            if !root_node.right_child.borrow().is_empty() {
                print_node(root_node.right_child.borrow()[0], order);
            }
        },
        2 => {
            if !root_node.left_child.borrow().is_empty() {
                print_node(root_node.left_child.borrow()[0], order);
            }
            if !root_node.right_child.borrow().is_empty() {
                print_node(root_node.right_child.borrow()[0], order);
            }
            println!("{}", root_node.value.borrow());
        },
        _ => {
            if !root_node.left_child.borrow().is_empty() {
                print_node(root_node.left_child.borrow()[0], order);
            }
            println!("{}", root_node.value.borrow());
            if !root_node.right_child.borrow().is_empty() {
                print_node(root_node.right_child.borrow()[0], order);
            }
        },
    }
}
