use rand::Rng;
use std::mem::size_of;
use std::time::Instant;

const N: usize = 10_000_000;

fn main() {
    println!("Building {} rows", N);

    let total_start = Instant::now();

    // ---------- build ----------
    let mut rng = rand::thread_rng();
    let mut data: Vec<(f64, u32)> = Vec::with_capacity(N);

    for i in 0..N {
        let amount = rng.gen_range(0.0..1_000_000.0);
        data.push((amount, i as u32));
    }

    // ---------- sort ----------
    let sort_start = Instant::now();

    data.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("Sort time: {:?}", sort_start.elapsed());

    println!("Data length: {}", data.len());

    println!(
        "AAA {:?} {:?} {:?} --- {:?} {:?} {:?} --- {:?} {:?} {:?}",
        data.get(0),
        data.get(1),
        data.get(2),
        data.get(500_000),
        data.get(600_000),
        data.get(700_000),
        data.get(data.len() - 3),
        data.get(data.len() - 2),
        data.get(data.len() - 1),
    );

    // ---------- memory ----------
    let element_size = size_of::<(f64, u32)>();
    let total_mem = element_size * data.capacity();

    println!("Element size: {} bytes", element_size);
    println!("Approx RAM: {:.2} MB", total_mem as f64 / (1024.0 * 1024.0));

    // ---------- queries ----------
    let query_amount = 500_000.0;

    let qstart = Instant::now();

    // > amount
    let gt_pos = data.partition_point(|x| x.0 <= query_amount);
    let gt_slice = &data[gt_pos..];

    // < amount
    let lt_pos = data.partition_point(|x| x.0 < query_amount);
    let lt_slice = &data[..lt_pos];

    // between
    let low = 200_000.0;
    let high = 300_000.0;

    let start = data.partition_point(|x| x.0 < low);
    let end = data.partition_point(|x| x.0 <= high);

    let between_slice = &data[start..end];

    println!("Query time: {:?}", qstart.elapsed());

    println!("> count: {}", gt_slice.len());
    println!("< count: {}", lt_slice.len());
    println!("between count: {}", between_slice.len());

    println!("Total runtime: {:?}", total_start.elapsed());
}
