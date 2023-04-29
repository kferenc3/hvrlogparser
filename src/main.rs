#![warn(clippy::all, clippy::pedantic)]
use std::{time};
use clap::Parser;
use hvrlogparser::engine::{Config, runner};

fn main() {
    let start_time = time::Instant::now();
    
    let runtime_cfg = Config::parse();
    runner(runtime_cfg);

    println!("hvrlogparser: Finished. (elapsed={:?}m {:?}s)", start_time.elapsed().as_secs()/60, start_time.elapsed().subsec_millis()/60);

}