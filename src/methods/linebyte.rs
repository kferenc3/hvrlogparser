use crate::methods::helpers::*;

pub (crate) fn linebyte_split(lines: &mut std::slice::Iter<u8>, filebase: &str, lbound: u64, ubound: u64, size: u64, m: &str) {
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
                content.extend_from_slice(&lineloop(lines));
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
                content.extend_from_slice(&lineloop(lines));
            }
            if content.is_empty() {
                break
            };
            filewriter(filename.as_str(), &content).expect("Error while writing to files");
            fileid += 1;
            content_size = 0;
            filename = format!("{filebase}{fileid}.out",);
            content.clear();

        };
    }
}