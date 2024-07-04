use std::cmp::Ordering;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn get_k_smallest_sort<T: Copy + Ord + PartialOrd>(arr: &mut [T], k: usize) -> Vec<T> {
    arr.sort_unstable();
    arr.iter().take(k).cloned().collect()
}

fn partition<T: Ord>(arr: &mut [T], pivot_index: usize) -> usize {
    arr.swap(pivot_index, arr.len() - 1);
    let mut store_index = 0;
    for i in 0..arr.len() - 1 {
        if arr[i] <= arr[arr.len() - 1] {
            arr.swap(store_index, i);
            store_index += 1;
        }
    }
    arr.swap(store_index, arr.len() - 1);
    store_index
}

fn quickselect<T: Ord>(arr: &mut [T], k: usize) {
    let mut left = 0;
    let mut right = arr.len() - 1;
    loop {
        if left == right {
            return;
        }
        let pivot_index = left + (right - left) / 2;
        let pivot_index = partition(&mut arr[left..=right], pivot_index - left) + left;
        match pivot_index.cmp(&k) {
            Ordering::Equal => return,
            Ordering::Less => left = pivot_index + 1,
            Ordering::Greater => right = pivot_index - 1,
        }
    }
}

fn get_k_smallest_quickselect<T: Copy + Ord>(arr: &mut [T], k: usize) -> Vec<T> {
    if k == 0 {
        return Vec::new();
    }
    quickselect(arr, k - 1);
    let mut result = arr.iter().take(k).cloned().collect::<Vec<_>>();
    result.sort_unstable(); // Optional: sort the result if needed
    result
}

fn generate_random_vec(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..1_000_000)).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let arr = generate_random_vec(20_000_000);
    let mut group = c.benchmark_group("get_k_smallest");
    group.bench_function("sort", |b| {
        b.iter(|| {
            let mut arr_clone = arr.clone();
            black_box(get_k_smallest_sort(&mut arr_clone, 10));
        });
    });
    group.bench_function("quickselect", |b| {
        b.iter(|| {
            let mut arr_clone = arr.clone();
            black_box(get_k_smallest_quickselect(&mut arr_clone, 10));
        });
    });
    group.finish();
}

criterion_group!(bench_criterion, criterion_benchmark);
criterion_main!(bench_criterion);