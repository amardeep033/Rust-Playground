use std::{
    error::Error,
    fs::File,
    io::{BufRead as _, BufReader},
};

use chrono::{NaiveDateTime, ParseError};

fn get_ts_from_iso(line: &str) -> Result<i64, ParseError> {
    let line_splitted: Vec<&str> = line.split(",").collect();
    let ndt = NaiveDateTime::parse_from_str(line_splitted[0], "%Y-%m-%dT%H:%M:%S")?;
    let ts = ndt.and_utc().timestamp();
    Ok(ts)
}

fn process_batches(f1_line: &str, f2_line: &str) -> Result<u32, Box<dyn Error>> {
    let ts1 = get_ts_from_iso(&f1_line)?;
    let ts2 = get_ts_from_iso(&f2_line)?;
    if ts1 < ts2 {
        return Ok(1);
    } else {
        return Ok(2);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file1 = File::open("file1.txt")?;
    let mut reader1 = BufReader::new(file1).lines();

    let file2 = File::open("file2.txt")?;
    let mut reader2 = BufReader::new(file2).lines();

    let mut f1_line = reader1.next().transpose()?;
    let mut f2_line = reader2.next().transpose()?;

    loop {
        match (&f1_line, &f2_line) {
            (Some(l1), Some(l2)) => match process_batches(l1, l2)? {
                1 => {
                    println!("{}", l1);
                    f1_line = reader1.next().transpose()?;
                }
                2 => {
                    println!("{}", l2);
                    f2_line = reader2.next().transpose()?;
                }
                _ => {
                    panic!("")
                }
            },
            (Some(l1), None) => {
                println!("{}", l1);
                f1_line = reader1.next().transpose()?;
            }
            (None, Some(l2)) => {
                println!("{}", l2);
                f2_line = reader2.next().transpose()?;
            }
            (None, None) => break,
        }
    }

    Ok(())
}
