use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};
use std::cmp::Reverse;

use chrono::{NaiveDateTime, ParseError};

//parse error because only one type of error can come here
//to know error type : parse_from_str -> ParseResult -> pub type ParseResult<T> = Result<T, ParseError>;
fn iso_to_timestamp(iso_ts: &str) -> Result<i64, ParseError> {
    //2024-01-01T10:00:01
    //don't manually split - make use of fmt
    let ndt = NaiveDateTime::parse_from_str(iso_ts, "%Y-%m-%dT%H:%M:%S")?;

    //make use of and_utc to remove deprecated
    Ok(ndt.and_utc().timestamp())
}

//dyn error because two type of error can come here: malformed and invalid ts
fn process_log_line(
    log_lines: &str,
    start_ts: i64,
    end_ts: i64,
    freq_store: &mut HashMap<String, u32>,
) -> Result<(), Box<dyn Error>> {
    // 2024-01-01T10:00:01,192.168.1.1
    //make use of split_once
    let log_lines_part: Vec<&str> = log_lines.split(",").collect();
    let ts_string = log_lines_part[0];
    let ts = iso_to_timestamp(ts_string)?;

    let ip = log_lines_part[1];

    if start_ts <= ts && ts <= end_ts {
        //search "hashmap entry api rust"
        freq_store
            .entry(ip.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    };
    Ok(())
}

//Error is a trait in rust and so all custom Error struct should implement - hence Box<dyn Error> - will handle all err
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("sample.txt")?;
    //bufreader collects data into buffer reducing os read hits -- no 1Byte hit -- 8Kb by default
    let reader = BufReader::new(file);

    let start_iso = "2024-01-01T10:00:00";
    let start_ts = iso_to_timestamp(start_iso)?;
    let end_iso = "2024-01-01T10:00:30";
    let end_ts = iso_to_timestamp(end_iso)?;

    let top_n = 2;
    //will be string because lines are dropped at end of each iteration
    let mut counter: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        //don't panic here, just print on stderr
        let line = match line {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Err while parsing line: {err}");
                continue;
            }
        };

        //don't panic, just log
        if let Err(err) = process_log_line(&line, start_ts, end_ts, &mut counter) {
            eprintln!("Err while processing line: {err}");
        }
    }

    // instead of sorting all - use a fixed-size min-heap for top-k
    let mut min_heap: BinaryHeap<(Reverse<u32>, &String)> = BinaryHeap::new();
    for (ip, &count) in &counter {
        min_heap.push((Reverse(count), ip));
        if min_heap.len() > top_n {
            min_heap.pop();
        }
    }

    let mut top_ips: Vec<_> = min_heap.into_sorted_vec();
    for (Reverse(count), ip) in top_ips.into_iter().rev() {
        println!("{ip} {count}");
    }

    Ok(())
}
//time complexity : O(N) file iteration + O(NLogN) hashmap sorting + O(k) lookup


// Time Complexity:
// O(N) to scan the file and build the HashMap
// + O(M log K) to maintain a min-heap of the top K IPs,
// where N = total log lines, M = unique IPs, K = top_n.
//
// Overall: O(N + M log K)