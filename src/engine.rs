use clap::{Parser};
use std::{fs::{self, OpenOptions}, process};
use chrono::{DateTime, Utc, Duration, NaiveDateTime};

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
            linebyte_split(&mut input.iter(), 
            config.file_basename.as_str(), 
            config.lower_bound, 
            config.upper_bound, 
            config.chunk_size,
            config.method.as_str());
        },
        "date" => {
            dateparser(&mut input.iter(),
            config.file_basename.as_str(),
            config.begin_time.as_str(),
            config.end_time.as_str(),
            config.granularity.as_str())
                },
        _ => println!("Method invalid or not implemented yet"),
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

#[derive(Debug)]
struct Boundary {
    lower: NaiveDateTime,
    upper: NaiveDateTime,
}

fn dateparser(lines: &mut std::slice::Iter<u8>, filebase: &str, begin: &str, end: &str, gran: &str) {
    //parse the line and store it in a temp_line_store
    //check if the line has a valid timestamps
    //if it does that should be the min_ts
    let granularity = match gran {
        "minute" => 1,
        "hour" => 60,
        "day" => 60 * 24,
        "month" => 60 * 24 * (365/12),
        _ => 60,
    };
    
    let mut min_ts = match begin {
        "1990-01-01T00:00:00+00:00" => DateTime::<Utc>::MIN_UTC.naive_utc(),
        _ => match DateTime::parse_from_str(begin, "%Y-%m-%dT%H:%M:%S%:z") {
                Ok(d) => d.naive_utc(),
                Err(_) => {
                    eprintln!("Problem parsing begin_time. Using default date instead. format: YYYY-mm-ddTHH:mm:SS+00:00");
                    DateTime::<Utc>::MIN_UTC.naive_utc()},
            },
    };
    let max_ts = match end {
        "2100-01-01T00:00:00+00:00" => DateTime::<Utc>::MAX_UTC.naive_utc(),
        _ => match DateTime::parse_from_str(end, "%Y-%m-%dT%H:%M:%S%:z") {
                Ok(d) => d.naive_utc(),
                Err(_) => 
                    {eprintln!("Problem parsing end_time. Using default date instead. format: YYYY-mm-ddTHH:mm:SS+00:00");
                    DateTime::<Utc>::MAX_UTC.naive_utc()},
                }
    };

    //looping until there are bytes in the input
    let mut content: Vec<u8> = vec![];
    
    //first loop is to make sure we have the minimum timestamp from the log and then generate the boundaries up until max_ts
    let mut l_temp = lines.clone();
    while let Some(b) = l_temp.next() {
        
        let mut l: Vec<u8> = vec![*b];
        //getting an entire line
        loop {
            match l_temp.next() {
            Some(&10) => {l.extend_from_slice(&[10]);
                        break},
            Some(s) => {
                l.extend_from_slice(&[*s]);
            },
            _ => {
                break},
            };
        }
        
        //parsing out the timestamp from the line
        let l_part = match &l.get(..25) {
            Some(t) => *t,
            None => b"asd"
        };

        let ts_str = match std::str::from_utf8(l_part) {
            Ok(s) => s,
            Err(_) => "a",
        };

        let line_ts = match DateTime::parse_from_str(ts_str, "%Y-%m-%dT%H:%M:%S%:z") {
                Ok(res) => Some(res.naive_utc()),
                Err(_) => None,
            };
        //Set the minimum timestamp to the first valid timestamp in the file
        if min_ts == DateTime::<Utc>::MIN_UTC.naive_utc() && !line_ts.is_none() {
            min_ts = line_ts.unwrap();
            break
        }
    }

    let mut boundary = Boundary {
        lower: min_ts,
        upper: min_ts + Duration::seconds(granularity * 60)
    };

    println!("{:?}", boundary);

    while let Some(b) = lines.next() {
        
        let filename = format!("{filebase}{}.out", boundary.upper.format("%Y%m%d%H%M%S").to_string());
        let mut l: Vec<u8> = vec![*b];
        //getting an entire line
        loop {
            match lines.next() {
            Some(&10) => {l.extend_from_slice(&[10]);
                        break},
            Some(s) => {
                l.extend_from_slice(&[*s]);
            },
            _ => {
                break},
            };
        }

        let l_part = match &l.get(..25) {
            Some(t) => *t,
            None => b"asd"
        };

        let ts_str = match std::str::from_utf8(l_part) {
            Ok(s) => s,
            Err(_) => "a",
        };

        let line_ts = match DateTime::parse_from_str(ts_str, "%Y-%m-%dT%H:%M:%S%:z") {
                Ok(res) => Some(res.naive_utc()),
                Err(_) => None,
            };
        
        //2023-04-03T16:10:17+00:00
        //2023-04-03T17:20:12+00:00
        match line_ts {
            None => content.append(&mut l),
            Some(ts) => {
                if ts >= boundary.lower && ts < boundary.upper {
                    content.append(&mut l);
                } else if ts >= boundary.upper && ts < max_ts {
                    filewriter(filename.as_str(), &content).expect("Error while writing to files");
                    content.clear();
                    content.append(&mut l);
                    boundary = Boundary {
                        lower: boundary.upper,
                        upper: boundary.upper + Duration::seconds(granularity * 60),
                    }
                } else if ts > max_ts {
                    filewriter(filename.as_str(), &content).expect("Error while writing to files");
                    content.clear();
                    break;
                }
            }
        }
    }
    if !content.is_empty() {
        let filename = format!("{filebase}{}.out", boundary.upper.format("%Y%m%d%H%M%S").to_string());
        filewriter(filename.as_str(), &content).expect("Error while writing to files");
    };

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
