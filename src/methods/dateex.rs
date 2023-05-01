use crate::methods::helpers::*;

use chrono::{DateTime, Utc, Duration, NaiveDateTime};

#[derive(Debug)]
struct Boundary {
    lower: NaiveDateTime,
    upper: NaiveDateTime,
}

pub (crate) fn dateparser(lines: &mut std::slice::Iter<u8>, filebase: &str, begin: &str, end: &str, gran: &str) {
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
        
        l.extend_from_slice(&lineloop(&mut l_temp));
        
        //parsing out the timestamp from the line
        let line_ts = ts_extract(&l);
    
        //Set the minimum timestamp to the first valid timestamp in the file
        if min_ts == DateTime::<Utc>::MIN_UTC.naive_utc() {
            min_ts = line_ts.unwrap_or(DateTime::<Utc>::MIN_UTC.naive_utc());
        } else {
            break
        }
    
    }

    let mut boundary = Boundary {
        lower: min_ts,
        upper: min_ts + Duration::seconds(granularity * 60)
    };

    while let Some(b) = lines.next() {
        
        let filename = format!("{filebase}{}.out", boundary.upper.format("%Y%m%d%H%M%S"));
        let mut l: Vec<u8> = vec![*b];
        //getting an entire line
        l.extend_from_slice(&lineloop(lines));

        let line_ts = ts_extract(&l);
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
                } else if ts < boundary.lower {
                    content.clear();
                }
            }
        }
    }
    if !content.is_empty() {
        let filename = format!("{filebase}{}.out", boundary.upper.format("%Y%m%d%H%M%S"));
        filewriter(filename.as_str(), &content).expect("Error while writing to files");
    };

}