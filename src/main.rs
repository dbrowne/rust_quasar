//! Rust quasar A program to periodically create a burst of network, cpu and disk IO activity
//! Dwight J. Browne
//!

use std::{thread, time};
use std::env;
use git_version::git_version;
use structopt::StructOpt;
use std::time::{Duration, Instant};

// Exponential complexity to consume resources


fn fib(inp: u64) -> u64 {
    match inp {
        0 | 1 | 2 => 1,
        _ => fib(inp - 1) + fib(inp - 2),
    }
}

fn fib_wrapper(inp: u64) {
    let start = Instant::now();

    let ret_val = fib(inp);
    let duration = start.elapsed();
    println!("Done with fib of {} = {} for {:?} seconds",inp, ret_val, duration);
}

fn main() {
    // check for command line args
    #[derive(StructOpt, Debug)]
    // #[derive(Debug)]
    #[structopt(rename_all = "kebab-case")]
    struct Opt {
        #[structopt(default_value = "0", short)]
        func: i32,
    }

    let opt = Opt::from_args();
    let args: Vec<String> = env::args().collect();
    let _func: i32;
    if args.len() > 1 {
        _func = opt.func;
    } else {
        _func = 0;
    }
    const GIT_VERSION: &str = git_version!();
    let host_name = match hostname::get() {
        Err(er) => panic!("Cant' get the system hostname : {}", er),
        Ok(host_name) => host_name
    };
    println!("Quasar version ->{}<- on host {:?}\n", GIT_VERSION, host_name);
    match _func {
        0 => fib_wrapper(46),
        _ => println!("Got something other than default {}", _func),
    }
}