use std::{fs::File, io::{BufReader, BufRead}};

fn synopsis() -> ! {
    panic!("Usage: diff path1 path2");
}

fn main() {
    let path1 = std::env::args().nth(1);
    let path2 = std::env::args().nth(2);
    let file1: File;
    let file2: File;
    let mut buf_read1: std::io::Lines<BufReader<File>>;
    let mut buf_read2: std::io::Lines<BufReader<File>>;

    match path1 {
        Some(p) => {
            let _file1 = File::open(p);
            match _file1 {
                Ok(f) => file1 = f,
                Err(err) => panic!("The first file does not exist!\n{:?}", err)
            }
        },
        None => synopsis()
    }
    match path2 {
        Some(p) => {
            let _file2 = File::open(p);
            match _file2 {
                Ok(f) => file2 = f,
                Err(err) => panic!("The second file does not exist!\n{:?}", err)
            }
        }
        None => synopsis()
    }

    buf_read1 = std::io::BufReader::new(file1).lines();
    buf_read2 = std::io::BufReader::new(file2).lines();

    let mut diff = String::from("");

    let mut nextline1 = buf_read1.next();
    let mut nextline2 = buf_read2.next();
    
    loop {
        match (nextline1, nextline2) {
            (Some(line1), Some(line2)) => {
                match (line1, line2) {
                    (Ok(line1), Ok(line2)) => {
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
                                    break;
                                }
                            }
                            char1 = chars1.next();
                            char2 = chars2.next();
                        }
                    },
                    _ => {
                        panic!("oh no");
                    }
                }
            },
            (Some(line1), None) => {
                match line1 {
                    Ok(line1) => {
                        diff.push_str(&line1);
                    },
                    _ => {
                        panic!("oh no");
                    }
                }
            },
            (None, Some(line2)) => {
                match line2 {
                    Ok(line2) => {
                        diff.push_str(&line2);
                    },
                    _ => {
                        panic!("oh no");
                    }
                }
            },
            (None, None) => {
                // no more lines left
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
