//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Sat, 09 Jan 2021 22:22:22 +0900
pub mod parse_args;
pub mod bin_search_tree_node;

use parse_args::Args;
use bin_search_tree_node::{BinSTreeNode, add_node, print_node, create_array};

use typed_arena::Arena;

fn main(){
    let args: Args = Args::parse_args();

    let root_node = BinSTreeNode::create_node("bbb".to_string());
    let node0 = BinSTreeNode::create_node("z".to_string());
    let node1 = BinSTreeNode::create_node("a".to_string());
    let node2 = BinSTreeNode::create_node("abc".to_string());
    let node3 = BinSTreeNode::create_node("aab".to_string());
    let node4 = BinSTreeNode::create_node("aba".to_string());

    let tree = Arena::new();
    add_node(&tree, &root_node, &node0);
    add_node(&tree, &root_node, &node1);
    add_node(&tree, &root_node, &node2);
    add_node(&tree, &root_node, &node3);
    add_node(&tree, &root_node, &node4);

    print_node(&root_node, 1);
    let array = create_array(&root_node, 1);
    println!("{:?}", array);
}
