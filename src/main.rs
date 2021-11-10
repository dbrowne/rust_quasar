//! Rust quasar A program to periodically create a burst of network, cpu and disk IO activity
//! Dwight J. Browne
//!

use std::env;
use git_version::git_version;
use structopt::StructOpt;
use std::time::Instant;
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};


use std::process::{Command, exit};


// Exponential complexity to consume resources


fn fib(inp: u64) -> u64 {
    match inp {
        0 | 1 | 2 => 1,
        _ => fib(inp - 1) + fib(inp - 2),
    }
}

fn fib_wrapper(inp: &i32) {
    let start = Instant::now();

    let xx = *inp as u64;
    let ret_val = fib(xx);
    let duration = start.elapsed();
    println!("Done with fib of {} = {} for {:?} seconds",inp, ret_val, duration);
}


fn main() {
    const UPPER: i32 = 55;
    const LOWER: i32 = 1;
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
    println!("args = {:?}",args);

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
    let pid = getpid();
    println!("this is pid {}",pid);
    match _func {
        0 => fib_wrapper(&_func),
        LOWER..=UPPER=>fib_wrapper(&_func),
        _ => {
            println!("Got something other than default {}", _func);
            exit(-1);
        },
    }

    let pid = unsafe {fork()};


    match pid.expect("Failed Fork: Cannot create child process!!"){
        Child => {
            println!("This is the child process: pid = {}, parent pid = {}", getpid(), getppid());
            println!("Calling command {} ",args[0].to_string());
            let tt = _func+1;
            let xxx = Command::new(&args[0].to_string()).args(["-f",&tt.to_string()]).status().expect("failed to execute process");
            println!("Called {:?}",xxx);
        },
        Parent { child} =>{

            let _res = wait();
            println!("This is the parent pid = {}, child= {}",getpid(), child);
        }
    }
}