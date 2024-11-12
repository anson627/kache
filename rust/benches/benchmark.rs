use criterion::{black_box, criterion_group, criterion_main, Criterion};
use k8s_openapi::api::core::v1::Pod;
use serde_json;
use std::time::Instant;

fn bench_encode(c: &mut Criterion) {
    let pod = Pod {
        metadata: Default::default(),
        spec: Some(k8s_openapi::api::core::v1::PodSpec {
            containers: vec![
                k8s_openapi::api::core::v1::Container {
                    name: "example".to_string(),
                    image: Some("example/image".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }),
        status: None,
    };

    c.bench_function("encode", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                serde_json::to_string(black_box(&pod)).unwrap();
            }
            start.elapsed()
        })
    });
}

fn bench_decode(c: &mut Criterion) {
    let data = "{\"spec\":{\"containers\":[{\"name\":\"example\",\"image\":\"example/image\"}]}}";

    c.bench_function("decode", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let _: Pod = serde_json::from_str(black_box(&data)).unwrap();
            }
            start.elapsed()
        })
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);