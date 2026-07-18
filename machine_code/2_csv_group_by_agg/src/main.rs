use std::{collections::HashMap, error::Error, fs::File};

// id,name,department,salary
// 1,Alice,Engineering,90000

#[derive(Debug, serde::Deserialize)]
struct Record {
    id: i32,
    name: String,
    department: String,
    salary: Option<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("inp.csv")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut agg_store: HashMap<String, u64> = HashMap::new();

    for result in rdr.deserialize() {
        let record: Record = match result {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error while reading line: {}", err);
                continue;
            }
        };
        agg_store
            .entry(record.department)
            .and_modify(|e| *e += record.salary.unwrap_or(0))
            .or_insert(record.salary.unwrap_or(0));
    }

    for (dept, amt) in agg_store {
        println!("{}: {}", dept, amt);
    }

    Ok(())
}
