use chrono::{DateTime, NaiveDateTime};
use std::{fs::{self, OpenOptions}};

pub (in crate::methods) fn lineloop (lines: &mut std::slice::Iter<u8>) -> Vec<u8> {
    let mut l: Vec<u8> = vec![];
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
    l.to_vec()
}

pub (in crate::methods) fn ts_extract (l: &[u8]) -> Option<NaiveDateTime> {
    
    let l_part = match &l.get(..25) {
        Some(t) => *t,
        None => b"asd"
    };

    let ts_str = std::str::from_utf8(l_part).unwrap_or("a");

    match DateTime::parse_from_str(ts_str, "%Y-%m-%dT%H:%M:%S%:z") {
            Ok(res) => Some(res.naive_utc()),
            Err(_) => None,
        }
}

pub (in crate::methods) fn filewriter (f: &str, c: &[u8]) -> Result<(), String>{
    match OpenOptions::new().write(true).create_new(true).open(f) {
        Ok(_) => {
            fs::write(f, c).expect("Unable to write to file");
            Ok(())
        },
        Err(_) => Err(String::from("Error while creating file!")),
        }
}