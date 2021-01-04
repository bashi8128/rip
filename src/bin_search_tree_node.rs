//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Mon, 04 Jan 2021 23:46:52 +0900
extern crate typed_arena;

use std::cell::RefCell;
//use typed_arena::Arena;

pub struct BinSTreeNode<'a, T>
    where
        T: PartialEq {
    pub value: RefCell<&'a T>,
    pub parent: RefCell<Vec<Node<'a, T>>>,
    pub left_child: RefCell<Vec<Node<'a, T>>>,
    pub right_child: RefCell<Vec<Node<'a, T>>>,
}

type Node<'a, T> = &'a BinSTreeNode<'a, T>;


impl<'a, T> BinSTreeNode<'a, T>
    where
        T: PartialEq {
    pub fn create_node(value: &'a T) -> BinSTreeNode<'a, T> {
        BinSTreeNode{
            value: RefCell::new(value),
            parent: RefCell::new(vec![]),
            left_child: RefCell::new(vec![]),
            right_child: RefCell::new(vec![]),
        }
    }
}
