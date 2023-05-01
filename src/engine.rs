use crate::methods::{linebyte, dateex};

use clap::{Parser};
use std::fs;

#[derive(Parser, Default, Debug)]
#[command(about="Parser for HVR generated logfiles")]
#[command(author="Ferenc KISS")]
#[command(version="0.1.0")]
#[command(help_template="Author: {author}\n{about}\n\n{usage}\n{all-args}")]

pub struct Config {
    #[arg(short, long, required(true), value_parser(["lines", "date", "bytes"]), default_value_t = String::from("lines"))]
    method: String,
    
    #[arg(short, long, required_if_eq("method", "date"), value_parser(["minute", "hour", "day", "month"]), default_value_t = String::from("hour"))]
    ///Required if the method is date
    granularity: String,
    
    #[arg(short, long, hide_default_value(true), default_value_t = String::from("1990-01-01T00:00:00+00:00"))] //the real default min is DateTime::MIN_UTC. This is just a placeholder
    ///Specify from where lines should be extracted (format: YYYY-mm-ddTHH:mm:SS+00:00). The default is the beginning of the file.
    begin_time: String,
    
    #[arg(short, long, hide_default_value(true), default_value_t = String::from("2100-01-01T00:00:00+00:00"))] //the real default max is DateTime::MAX_UTC. This is just a placeholder
    ///Specify until where lines should be extracted (format: YYYY-mm-ddTHH:mm:SS+00:00). The default is the end of the file.
    end_time: String,
    
    #[arg(short, long, hide_default_value(true), default_value_t = 1)]
    ///Optional if the method is lines/bytes. This will be the starting line/byte.
    lower_bound: u64,

    #[arg(short, long, hide_default_value(true), default_value_t = u64::MAX)]
    ///Optional if the method is lines/bytes. This will be the final line/byte.
    upper_bound: u64,

    #[arg(short, long, required_if_eq_any([("method", "lines"), ("method", "bytes")]), default_value_t = 10)]
    ///Required if the method is lines/bytes. Given the selected method the value supplied will be the number of lines in a file, or the number of bytes.
    chunk_size: u64,
    
    #[arg(short, long, default_value_t = String::from("part_"))]
    file_basename: String,

    #[arg(required(true))]
    input_file: String
}

pub fn runner (config: Config) {
    
    let input = fs::read(config.input_file).expect("Unable to read file");

    match config.method.as_str() {
        "lines" | "bytes" => {
            linebyte::linebyte_split(&mut input.iter(), 
            config.file_basename.as_str(), 
            config.lower_bound, 
            config.upper_bound, 
            config.chunk_size,
            config.method.as_str());
        },
        "date" => {
            dateex::dateparser(&mut input.iter(),
            config.file_basename.as_str(),
            config.begin_time.as_str(),
            config.end_time.as_str(),
            config.granularity.as_str())
                },
        _ => println!("Method invalid or not implemented yet"),
    }
}