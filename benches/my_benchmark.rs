use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn baka_solution(mut tasks: Vec<u32>, find: u32) {
    // let index = tasks
    //     .iter()
    //     .enumerate()
    //     .find(|(_, t)| **t == find)
    //     .map(|result| result.0)
    //     .unwrap_or(tasks.len() + 1);
    //
    // tasks.swap_remove(index);
    tasks.retain(|t| *t != find);
}

fn video_solution(mut tasks: Vec<u32>, find: u32) {
    tasks.retain(|t| *t != find);
}

fn esteban_chad_solution(mut tasks: Vec<u32>, find: u32) {
    (0..tasks.len()).any(|index| {
        if tasks[index] == find {
            tasks.swap_remove(index);
            true
        } else {
            false
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let total_len = 100_000_000;

    c.bench_function("baka solution", |b| {
        b.iter(|| baka_solution(black_box((0..total_len).collect()), total_len / 2))
    });

    c.bench_function("chad solution", |b| {
        b.iter(|| esteban_chad_solution(black_box((0..total_len).collect()), total_len / 2))
    });

    c.bench_function("video solution", |b| {
        b.iter(|| video_solution(black_box((0..total_len).collect()), total_len / 2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
