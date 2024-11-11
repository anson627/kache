use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Pod {
    spec: PodSpec,
}

#[derive(Serialize, Deserialize)]
struct PodSpec {
    containers: Vec<Container>,
}

#[derive(Serialize, Deserialize)]
struct Container {
    name: String,
    image: String,
}

fn bench_encode(c: &mut Criterion) {
    let pod = Pod {
        spec: PodSpec {
            containers: vec![
                Container {
                    name: "example".to_string(),
                    image: "example/image".to_string(),
                },
            ],
        },
    };
    c.bench_function("encode", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&pod)).unwrap();
        })
    });
}

fn bench_decode(c: &mut Criterion) {
    let pod = Pod {
        spec: PodSpec {
            containers: vec![
                Container {
                    name: "example".to_string(),
                    image: "example/image".to_string(),
                },
            ],
        },
    };
    let data = serde_json::to_string(&pod).unwrap();
    c.bench_function("decode", |b| {
        b.iter(|| {
            let _: Pod = serde_json::from_str(black_box(&data)).unwrap();
        })
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);