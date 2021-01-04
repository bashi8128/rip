//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Mon, 04 Jan 2021 23:16:07 +0900
pub mod parse_args;
pub mod bin_search_tree_node;

use parse_args::Args;

fn main(){
    let args: Args = Args::parse_args();

    println!("{:?}", args);

}

