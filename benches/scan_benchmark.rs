use criterion::{black_box, Criterion};
use aegis_lib::engine::Engine;
use aegis_lib::rule::RuleConfig;

pub fn scan_benchmark(c: &mut Criterion) {
    let rules = RuleConfig::load_all_from_dir("./rules").unwrap();
    let mut engine = Engine::new().unwrap();

    c.bench_function("scan_vulnerable_app", |b| {
        b.iter(|| {
            let violations = engine.scan_file(
                "./vulnerable-app/src/index.ts",
                black_box(&rules),
            )
            .unwrap();
            black_box(violations)
        });
    });

    c.bench_function("scan_clean_app", |b| {
        b.iter(|| {
            let violations = engine.scan_file(
                "./test-clean/src/safe-app.ts",
                black_box(&rules),
            )
            .unwrap();
            black_box(violations)
        });
    });

    c.bench_function("scan_python_vulnerable", |b| {
        b.iter(|| {
            let violations = engine.scan_file(
                "./vulnerable-app/src/app.py",
                black_box(&rules),
            )
            .unwrap();
            black_box(violations)
        });
    });
}

criterion::criterion_group!(benches, scan_benchmark);
criterion::criterion_main!(benches);
