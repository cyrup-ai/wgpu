use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::LazyLock;

use crate::{DeviceState, is_test};

fn thread_count_list() -> &'static [usize] {
    if is_test() { &[2] } else { &[1, 2, 4, 8] }
}

fn run_bench(ctx: &mut Criterion) {
    let state = LazyLock::new(DeviceState::new);

    const RESOURCES_TO_CREATE: usize = 8;

    let mut group = ctx.benchmark_group("Resource Creation: Large Buffer");
    group.throughput(Throughput::Elements(RESOURCES_TO_CREATE as _));

    for &threads in thread_count_list() {
        let resources_per_thread = RESOURCES_TO_CREATE / threads;
        group.bench_function(
            format!("{threads} threads x {resources_per_thread} resource"),
            |b| {
                LazyLock::force(&state);

                b.iter_custom(|iters| {
                    profiling::scope!("benchmark invocation");

                    let mut duration = Duration::ZERO;

                    for _ in 0..iters {
                        profiling::scope!("benchmark iteration");

                        // We can't create too many resources at once, so we do it 8 resources at a time.
                        let start = Instant::now();

                        let buffers = (0..threads)
                            .into_par_iter()
                            .map(|_| {
                                (0..resources_per_thread)
                                    .map(|_| {
                                        state.device.create_buffer(&wgpu::BufferDescriptor {
                                            label: None,
                                            size: 256 * 1024 * 1024,
                                            usage: wgpu::BufferUsages::COPY_DST,
                                            mapped_at_creation: false,
                                        })
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>();

                        duration += start.elapsed();

                        drop(buffers);

                        state.queue.submit([]);
                        state.device.poll(wgpu::PollType::Wait).unwrap();
                    }

                    duration
                })
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = resource_creation;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = run_bench,
}
