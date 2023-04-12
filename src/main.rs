#![warn(clippy::all, clippy::pedantic)]
use std::{time};
use clap::Parser;
//use chrono::{DateTime};
use hvrlogparser::engine::{Config, runner};

fn main() {
    let start_time = time::Instant::now();
    
    let runtime_cfg = Config::parse();
    runner(runtime_cfg);
    
    //let min_ts_str = &lines.peek().unwrap().split(' ').next().unwrap().chars();
    //let min_ts_str = min_ts_str.as_str();
    //let _min_ts = DateTime::parse_from_str(min_ts_str, "%Y-%m-%dT%H:%M:%S%:z:").unwrap().naive_utc();

    println!("hvrlogparser: Finished. (elapsed={:?}m {:?}s)", start_time.elapsed().as_secs()/60, start_time.elapsed().subsec_millis()/60);

}