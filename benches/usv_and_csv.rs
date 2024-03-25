use std::error::Error;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::*;
use rand::distributions::*;
use usv::*;

/// Generate a random string of given length
#[inline]
pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate random USV data
pub fn random_usv_data(unit_length: usize, unit_count: usize, record_count: usize) -> String {
    let mut s = String::new();
    for _ in 0..record_count {
        for _ in 0..unit_count {
            s += &random_string(unit_length);
            s += "␟";
        }
        s += "␞";
    }
    s
}

/// Generate random CSV data
pub fn random_csv_data(unit_length: usize, unit_count: usize, record_count: usize) -> String {
    let mut s = String::new();
    for _ in 0..record_count {
        for u in 0..unit_count {
            s += &random_string(unit_length);
            if u < unit_count - 1 { s += ","; }
        }
        s += "\n";
    }
    s
}

pub fn parse_usv(s: &String) -> usize {
    let mut n = 0;
    for record in s.records() {
        for _/*unit*/ in record {
            n += 1;
        }
    }
    n
}

fn parse_csv(s: &String) -> Result<usize, Box<dyn Error>> {
    let mut n = 0;
    let mut reader = csv::Reader::from_reader(s.as_bytes());
    for result in reader.records() {
        if let Ok(record) = result {
            for _/*field*/ in record.iter() {
                n += 1;
            }
        }
    }
    Ok(n)
}

fn bench_usv_csv(c: &mut Criterion){
    let unit_length = 10;
    let unit_count = 1000;
    let record_count = 1000;

    let usv_data = random_usv_data(unit_length, unit_count, record_count);
    let csv_data = random_csv_data(unit_length, unit_count, record_count);

    let mut group = c.benchmark_group(
        &format!("benchmark group unit_length: {}, unit_count: {}, record_count: {}", unit_length, unit_count, record_count)
    );
    group.sample_size(10);
    group.bench_function("parse_usv", |b| b.iter(|| parse_usv(&usv_data)));
    group.bench_function("parse_csv", |b| b.iter(|| parse_csv(&csv_data)));
    group.finish();
}

criterion_group!(benches, bench_usv_csv);
criterion_main!(benches);
