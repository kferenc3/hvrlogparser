use clap::{Parser};
use std::fs::{self, OpenOptions};

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
    
    #[arg(short, long, hide_default_value(true), default_value_t = String::from("2020-01-01T00:00:00+00:00"))]
    ///Specify from where lines should be extracted (format: YYYY-mm-ddTHH:mm:SS+00:00). The default is the beginning of the file.
    begin_time: String,
    
    #[arg(short, long, hide_default_value(true), default_value_t = String::from("2030-01-01T00:00:00+00:00"))]
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
            linebyte_split(&mut input.iter(), 
            config.file_basename.as_str(), 
            config.lower_bound, 
            config.upper_bound, 
            config.chunk_size,
            config.method.as_str());},
        _ => println!("Method not implemented yet"),
    }
}

fn linebyte_split(lines: &mut std::slice::Iter<u8>, filebase: &str, lbound: u64, ubound: u64, size: u64, m: &str) {
    let mut fileid = 0;

    //This section skips the lines/bytes up until the lower bound in case only certain rows from the middle are needed.
    match m {
        "lines" => {
            if lbound > 1 {
                for _ in 0..lbound-1 {
                    loop {
                        if lines.next() == Some(&10) {
                            break
                        }
                    }
                    
                }
            }
        },
        "bytes" => {
            if lbound > 1 {
                for _ in 1..=lbound-1 {
                    lines.next();
                }
            }
        },
        _ => println!("invalid method!")
    };
    
    let mut content_size = 0;
    let mut filename = format!("{filebase}{fileid}.out",);
    let mut content: Vec<u8> = vec![];
    
    //for lines it will go into a loop at each iteration looking for the \n char (10). Once that finishes then the loop breaks and the line
    //will be added to the content to be written to the file. For bytes it will simply just iterate over the bytes 
    //until the necessary chunk size is reached.
    for x in lbound..=ubound {    
        match m {
            "lines" => {
                //line
                loop {
                    match lines.next() {
                        Some(&10) => {
                            content.extend_from_slice(&[10]);
                            break
                        },
                        Some(s) => {
                            content.extend_from_slice(&[*s]);
                        },
                        _ => {
                            filewriter(filename.as_str(), &content).expect("Error while writing to files");
                            break},
                    }
                }
            },
            "bytes" => {
                    //byte
                    match lines.next() {
                        Some(s) => {
                            content.extend_from_slice(&[*s]);
                        },
                        _ => {
                            filewriter(filename.as_str(), &content).expect("Error while writing to files");
                            break},
                    }
            },
            _ => println!("invalid method supplied")   
        };
        content_size += 1;
        
        if content_size == size || x == ubound {
            //the additional looping is needed for the byte method so the current row is written to the file in full and not cut in half
            if m == "bytes" {
                loop {
                    match lines.next() {
                    Some(&10) => {content.extend_from_slice(&[10]);
                                break},
                    Some(s) => {
                        content.extend_from_slice(&[*s]);
                    },
                    _ => {
                        filewriter(filename.as_str(), &content).expect("Error while writing to files");
                        break},
                    }
                }
            }

            filewriter(filename.as_str(), &content).expect("Error while writing to files");
            fileid += 1;
            content_size = 0;
            filename = format!("{filebase}{fileid}.out",);
            content = vec![];

        };
    }
}

fn filewriter (f: &str, c: &[u8]) -> Result<(), String>{
    match OpenOptions::new().write(true).create_new(true).open(f) {
        Ok(_) => {
            fs::write(f, c).expect("Unable to write to file");
            Ok(())
        },
        Err(_) => Err(String::from("Error while creating file!")),
        }
}
