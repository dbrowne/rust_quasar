//! Rust quasar A program to periodically create a burst of network, cpu and disk IO activity
//! Dwight J. Browne
//!

use std::{thread, time};
use std::env;
use git_version::git_version;


fn fib(inp: u64) ->u64 {
    match inp{
        0 => 1,
        1 => 1,
        _ => fib(inp -1) + fib(inp -2),
    }
}


fn main() {
    const GIT_VERSION: &str = git_version!();
    let host_name :osString = match hostname::get() {
      Err(er) => panic!("Cant' get the system hostname : {}", er),
      Ok(host_name) =>host_name
    };

    println!("Quasar version {} on host {}\n",GIT_VERSION,host_name);
    let m = fib(4);
    println!("hit {}",m);

}