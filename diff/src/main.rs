use std::{fs::File, io::{BufRead}};

fn synopsis() -> ! {
    panic!("Usage: diff path1 path2");
}

fn main() {
    let path1 = std::env::args().nth(1);
    let path2 = std::env::args().nth(2);

    let file1 = match path1 {
        Some(p) => File::open(p).expect("The first file does not exist!"),
        None => synopsis()
    };
    let file2 = match path2 {
        Some(p) => File::open(p).expect("The second file does not exist!"),
        None => synopsis()
    };

    let mut buf_read1 = std::io::BufReader::new(file1).lines();
    let mut buf_read2 = std::io::BufReader::new(file2).lines();

    let mut nextline1 = buf_read1.next();
    let mut nextline2 = buf_read2.next();

    let mut diff = String::from("");
    
    loop {
        match (nextline1, nextline2) {
            (Some(Ok(line1)), Some(Ok(line2))) => {
                let mut chars1 = line1.chars();
                let mut chars2 = line2.chars();
                let mut char1 = chars1.next();
                let mut char2 = chars2.next();
                loop {
                    match (char1, char2) {
                        (Some(char1), Some(char2)) => {
                            if char1 != char2 {
                                diff.push(char2);
                            }
                        },
                        (Some(char1), None) => {
                            diff.push(char1);
                        },
                        (None, Some(char2)) => {
                            diff.push(char2)
                        }, 
                        (None, None) => {
                            // no more chars left in either file
                            break;
                        }
                    }
                    char1 = chars1.next();
                    char2 = chars2.next();
                }
            },
            (Some(line1), None) => {
                diff.push_str(&line1.unwrap());
            },
            (None, Some(line2)) => {
                diff.push_str(&line2.unwrap());
            },
            (Some(_), Some(_)) => {
                panic!("oh no");
            }
            (None, None) => {
                // no more lines left in either file
                break;
            },
        }

        diff.push('\n');
        nextline1 = buf_read1.next();
        nextline2 = buf_read2.next();
    }

    for (i, line) in diff.lines().enumerate() {
        println!("\t{}: {}", i, line)
    }
}
