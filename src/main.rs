//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Thu, 31 Dec 2020 23:35:38 +0900
extern crate rip;

use rip::parse_args::Args;

fn main(){
    let args: Args = Args::parse_args();

    println!("{:?}", args);

}

