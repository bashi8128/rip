//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Sat, 09 Jan 2021 22:31:20 +0900
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

pub fn subst_string<'a>(root_node: Node<'a, String>,
                                          target_str: &String,
                                          replace_str: &String) -> usize {
    let mut replaced_node: usize = 0;
    let replaced_str = 
        (*root_node.value.borrow_mut()).replace(target_str, replace_str);
    (*root_node.value.borrow_mut()) = replaced_str;
    replaced_node += 1;
    if !root_node.left_child.borrow().is_empty() {
        replaced_node += 
            subst_string(root_node.left_child.borrow()[0], target_str, replace_str);
    }
    if !root_node.right_child.borrow().is_empty() {
        replaced_node += 
            subst_string(root_node.right_child.borrow()[0], target_str, replace_str);
    }
    replaced_node
}

pub fn create_array<'a, T: Ord + Clone>(root_node: Node<'a, T>, order: usize) -> Vec<T>{
    let mut array = Vec::new();
    match order {
        0 => {
            let value = (*root_node.value.borrow()).clone();
            array.push(value);
            if !root_node.left_child.borrow().is_empty() {
                array.append(&mut create_array(root_node.left_child.borrow()[0], order));
            }
            if !root_node.right_child.borrow().is_empty() {
                array.append(&mut create_array(root_node.right_child.borrow()[0], order));
            }
            array
        },
        2 => {
            if !root_node.left_child.borrow().is_empty() {
                array.append(&mut create_array(root_node.left_child.borrow()[0], order));
            }
            if !root_node.right_child.borrow().is_empty() {
                array.append(&mut create_array(root_node.right_child.borrow()[0], order));
            }
            let value = (*root_node.value.borrow()).clone();
            array.push(value);
            array
        },
        _ => {
            if !root_node.left_child.borrow().is_empty() {
                array.append(&mut create_array(root_node.left_child.borrow()[0], order));
            }
            let value = (*root_node.value.borrow()).clone();
            array.push(value);
            if !root_node.right_child.borrow().is_empty() {
                array.append(&mut create_array(root_node.right_child.borrow()[0], order));
            }
            array
        },
    }
}
