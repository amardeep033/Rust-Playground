use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::NaiveDateTime;

fn iso_to_timestamp(iso_ts: &str) -> Result<i64, String> {
    //2024-01-01T10:00:01

    //deprecated can be removed
    match NaiveDateTime::parse_from_str(iso_ts, "%Y-%m-%dT%H:%M:%S") {
        Ok(val) => Ok(val.timestamp()),
        Err(err) => Err(format!("Could not parse as timestamp: {}", err)),
    }
}

fn process_the_line(
    log_lines: &str,
    start_ts: i64,
    end_ts: i64,
    freq_store: &mut HashMap<String, u32>,
) -> Result<(), String> {
    // 2024-01-01T10:00:01,192.168.1.1
    let log_lines_part: Vec<&str> = log_lines.split(",").collect();
    let ts_string = log_lines_part[0];
    match iso_to_timestamp(ts_string) {
        Ok(ts) => {
            let ip = log_lines_part[1];

            if start_ts <= ts && ts <= end_ts {
                freq_store
                    .entry(ip.to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            Ok(())
        }
        Err(err) => Err(format!("Could not parse as timestamp: {}", err)),
    }
}

fn main() {
    let file = File::open("sample.txt").expect("file does not exist");
    let reader = BufReader::new(file);

    let start_iso = "2024-01-01T10:00:00";
    let start_ts = iso_to_timestamp(start_iso).expect("Could not parse the final_start_ts");
    let end_iso = "2024-01-01T10:00:30";
    let end_ts = iso_to_timestamp(end_iso).expect("Could not parse the final_end_ts");

    let mut top_n = 2;
    let mut counter: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("line is not proper");
        match process_the_line(&line, start_ts, end_ts, &mut counter) {
            Ok(_) => {}
            Err(err) => panic!("Could not process the line due to {}", err),
        };
    }

    // instead of sorting all - pq can be used
    let mut map_vec: Vec<_> = counter.iter().collect();

    map_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (k, v) in map_vec {
        if top_n > 0 {
            println!("{k} {v}");
            top_n = top_n - 1;
        }
    }
}
