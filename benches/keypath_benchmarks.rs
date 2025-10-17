//! Benchmarks for keypath operations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;
use std::collections::HashMap;

#[derive(Keypath, Debug, Clone)]
struct BenchmarkData {
    id: u32,
    name: String,
    value: f64,
    category: String,
    tags: Vec<String>,
}

fn generate_benchmark_data(size: usize) -> Vec<BenchmarkData> {
    (0..size)
        .map(|i| BenchmarkData {
            id: i as u32,
            name: format!("Item_{}", i),
            value: (i as f64) * 1.5,
            category: if i % 3 == 0 { "A".to_string() } else if i % 3 == 1 { "B".to_string() } else { "C".to_string() },
            tags: vec![format!("tag_{}", i % 10)],
        })
        .collect()
}

fn benchmark_filtering(c: &mut Criterion) {
    let mut group = c.benchmark_group("filtering");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_benchmark_data(*size);
        
        group.bench_with_input(BenchmarkId::new("keypath_filter", size), size, |b, _| {
            b.iter(|| {
                let result = filter_by_keypath(data.clone(), BenchmarkData::value(), |&value| value > 50.0).unwrap();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("traditional_filter", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<BenchmarkData> = data
                    .iter()
                    .filter(|item| item.value > 50.0)
                    .cloned()
                    .collect();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_mapping(c: &mut Criterion) {
    let mut group = c.benchmark_group("mapping");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_benchmark_data(*size);
        
        group.bench_with_input(BenchmarkId::new("keypath_map", size), size, |b, _| {
            b.iter(|| {
                let result = map_keypath_collection(&data, BenchmarkData::name(), |name| name.to_uppercase()).unwrap();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("traditional_map", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<String> = data
                    .iter()
                    .map(|item| item.name.to_uppercase())
                    .collect();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_grouping(c: &mut Criterion) {
    let mut group = c.benchmark_group("grouping");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_benchmark_data(*size);
        
        group.bench_with_input(BenchmarkId::new("keypath_group", size), size, |b, _| {
            b.iter(|| {
                let result = group_by_keypath(&data, BenchmarkData::category(), |category| category.clone()).unwrap();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("traditional_group", size), size, |b, _| {
            b.iter(|| {
                let mut groups: HashMap<String, Vec<BenchmarkData>> = HashMap::new();
                for item in &data {
                    groups.entry(item.category.clone()).or_insert_with(Vec::new).push(item.clone());
                }
                black_box(groups)
            })
        });
    }
    
    group.finish();
}

fn benchmark_folding(c: &mut Criterion) {
    let mut group = c.benchmark_group("folding");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_benchmark_data(*size);
        
        group.bench_with_input(BenchmarkId::new("keypath_fold", size), size, |b, _| {
            b.iter(|| {
                let result = fold_keypath(data.clone(), BenchmarkData::value(), 0.0, |acc, &value| acc + value).unwrap();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("traditional_fold", size), size, |b, _| {
            b.iter(|| {
                let result: f64 = data
                    .iter()
                    .fold(0.0, |acc, item| acc + item.value);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_composable_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("composable");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_benchmark_data(*size);
        
        group.bench_with_input(BenchmarkId::new("keypath_chain", size), size, |b, _| {
            b.iter(|| {
                let filtered = filter_by_keypath(data.clone(), BenchmarkData::value(), |&value| value > 50.0).unwrap();
                let result = map_keypath_collection(&filtered, BenchmarkData::name(), |name| name.clone()).unwrap();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("traditional_chain", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<String> = data
                    .iter()
                    .filter(|item| item.value > 50.0)
                    .map(|item| item.name.clone())
                    .collect();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_nested_keypaths(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_keypaths");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_benchmark_data(*size);
        
        group.bench_with_input(BenchmarkId::new("nested_keypath", size), size, |b, _| {
            b.iter(|| {
                let result = map_keypath_collection(&data, BenchmarkData::name(), |name| {
                    name.len()
                }).unwrap();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("traditional_nested", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<String> = data
                    .iter()
                    .map(|item| {
                        item.tags.first().unwrap_or(&"default".to_string()).clone()
                    })
                    .collect();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

#[cfg(feature = "parallel")]
fn benchmark_parallel_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel");
    
    for size in [1000, 10000, 100000].iter() {
        let data = generate_benchmark_data(*size);
        
        group.bench_with_input(BenchmarkId::new("parallel_keypath", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<String> = parallel_collections::par_map_keypath(
                    &data,
                    BenchmarkData::name(),
                    |name| name.to_uppercase()
                ).unwrap();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("sequential_keypath", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<String> = data
                    .iter()
                    .map_keypath(BenchmarkData::name(), |name| name.to_uppercase())
                    .collect();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_filtering,
    benchmark_mapping,
    benchmark_grouping,
    benchmark_folding,
    benchmark_composable_operations,
    benchmark_nested_keypaths
);

#[cfg(feature = "parallel")]
criterion_group!(
    parallel_benches,
    benchmark_parallel_operations
);

criterion_main!(benches);

#[cfg(feature = "parallel")]
criterion_main!(benches, parallel_benches);
