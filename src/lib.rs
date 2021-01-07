//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Thu, 07 Jan 2021 21:38:01 +0900
pub mod parse_args;
pub mod bin_search_tree_node;

use parse_args::Args;
use bin_search_tree_node::{BinSTreeNode, add_node, print_node};

use typed_arena::Arena;

//TODO: Write test
