//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Sat, 02 Jan 2021 18:48:32 +0900
extern crate typed_arena;

use std::cell::RefCell;
use typed_arena::Arena;

pub struct BinSTreeNode<'a, T>
    where
        T: PartialEq {
    value: RefCell<T>,
    parent: RefCell<Node<'a, T>>,
    left_child: RefCell<Node<'a, T>>,
    right_child: RefCell<Node<'a, T>>,
}

type Node<'a, T> = &'a BinSTreeNode<'a, T>;


impl<'a, T> BinSTreeNode<'a, T>
    where
        T: PartialEq {
    pub fn create_node(value: &T) -> BinSTreeNode<'a, T> {
        BinSTreeNode{
            value,
            parent: Nil,
        }
    }
}
