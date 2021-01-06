//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Thu, 07 Jan 2021 04:50:45 +0900
pub mod parse_args;
pub mod bin_search_tree_node;

use parse_args::Args;
use bin_search_tree_node::{BinSTreeNode, add_node};

use typed_arena::Arena;

fn main(){
    let args: Args = Args::parse_args();

    let root_node = BinSTreeNode::create_node("aaa".to_string());
    let node0 = BinSTreeNode::create_node("bbb".to_string());
    let node1 = BinSTreeNode::create_node("a".to_string());

    let tree = Arena::new();
    add_node(&tree, &root_node, &node0);
    add_node(&tree, &root_node, &node1);

    println!("{:?}", root_node.left_child.borrow()[0].value);
    println!("{:?}", root_node.right_child.borrow()[0].value);
}
