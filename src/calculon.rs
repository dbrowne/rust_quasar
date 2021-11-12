pub mod clc{
    use std::time::Instant;
    // Exponential complexity to consume resources
    fn fib(inp: u64) -> u64 {
        match inp {
            0 | 1 | 2 => 1,
            _ => fib(inp - 1) + fib(inp - 2),
        }
    }

    pub fn fib_wrapper(inp: &i32) {
        let start = Instant::now();

        let xx = *inp as u64;
        let ret_val = fib(xx);
        let duration = start.elapsed();
        println!("Done with fib of {} = {} for {:?} seconds",inp, ret_val, duration);
    }
}