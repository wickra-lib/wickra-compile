//! Criterion benchmarks for the wickra-compile codegen path: `generate`
//! (codegen + manifest) per target, and the two hashing primitives. The real
//! `compile` (an actual `cargo` build) is deliberately not benchmarked — it is
//! too slow and too environment-dependent for CI.

use compile_core::{generate, manifest_of, project_hash, spec_hash, CompileSpec, Target};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use serde_json::json;

fn spec(target: Target) -> CompileSpec {
    CompileSpec {
        strategy: json!({
            "symbol": "x",
            "timeframe": "1h",
            "indicators": {
                "fast": { "type": "Sma", "params": [10] },
                "slow": { "type": "Sma", "params": [30] }
            },
            "entry": { "cross_above": ["fast", "slow"] },
            "exit": { "cross_below": ["fast", "slow"] },
            "sizing": { "type": "fixed_qty", "qty": 1 }
        }),
        target,
        opt_level: compile_core::OptLevel::Release,
        embed_data: None,
        crate_name: Some("bench".to_owned()),
    }
}

fn bench_generate(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");
    for (name, target) in [
        ("wasm", Target::Wasm),
        ("binary", Target::Binary),
        (
            "no_std",
            Target::NoStd {
                mcu: "thumbv7em-none-eabihf".to_owned(),
            },
        ),
    ] {
        let spec = spec(target);
        group.bench_with_input(BenchmarkId::from_parameter(name), &spec, |b, spec| {
            b.iter(|| generate(spec).unwrap());
        });
    }
    group.finish();
}

fn bench_hashing(c: &mut Criterion) {
    let spec = spec(Target::Wasm);
    let project = generate(&spec).unwrap();

    c.bench_function("spec_hash", |b| b.iter(|| spec_hash(&spec).unwrap()));
    c.bench_function("project_hash", |b| {
        b.iter(|| project_hash(&project.files));
    });
    c.bench_function("manifest_of", |b| b.iter(|| manifest_of(&spec).unwrap()));
}

criterion_group!(benches, bench_generate, bench_hashing);
criterion_main!(benches);
