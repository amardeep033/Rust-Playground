use std::collections::HashMap;

fn main() {
    let mut counter_store:HashMap<&str, u32> = HashMap::new();

    let timestamp = [1, 2, 3, 4, 5, 6];
    let service   = ["auth", "billing", "auth", "search", "billing", "auth"];
    let level     = ["ERROR", "INFO", "ERROR", "ERROR", "ERROR", "INFO"];

    for i in 0..level.len() {
        if level[i] == "ERROR" {
            counter_store.entry(service[i]).and_modify(|cnt| {*cnt+=1}).or_insert(1);
        }
    }
    for (srvc,cnt) in counter_store{
        println!("{srvc} : {cnt}");
    }
}