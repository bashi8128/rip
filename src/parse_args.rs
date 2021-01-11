//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Mon, 11 Jan 2021 21:35:22 +0900
use std::env;

use docopt::Docopt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Args {
    arg_file: String,
    flag_p: Option<i32>,
    flag_s: Option<String>,
    flag_u: bool,
    flag_r: Option<String>,
    flag_h: bool,
}

impl Args {
    pub fn parse_args() -> Args {
        const USAGE: &'static str= "
        Rust implemetation of PJT
        
        Usage:
          rip [options] [<file>]
          rip (-h|--help)
        
        Options:
          -p <order>        output BinTree
          -s SUBST          substitute specified node and output BinTree
          -u                output BinTree with a deduplication
          -r REMOVE         remove specified node
          -h --help         output this help
        ";
        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(env::args().into_iter()).deserialize())
            .unwrap_or_else(|e| e.exit());
        args
    }
}
