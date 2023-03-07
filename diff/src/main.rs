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
    
    while let (Some(Ok(line1)), Some(Ok(line2))) = (buf_read1.next(), buf_read2.next()) {
        if line1 != line2 {
            while let (chars1, chars2) = (line1.chars(), line2.chars()) {
                if chars1 != chars2 {
                    // TODO: define what a diff is.
                }
            }
        }
    }

    println!("Lines left in buf1: {}", buf_read1.count());
    println!("Lines left in buf2: {}", buf_read2.count());

}
