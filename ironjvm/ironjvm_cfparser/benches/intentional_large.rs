use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ironjvm_cfparser::ClassFileParser;

fn intentional_large(criterion: &mut Criterion) {
    criterion.bench_function("Intentional Large", |bencher| {
        bencher.iter(|| {
            let file = std::fs::read(
                "../test_classes/com/github/htgazurex1212/ironjvm/tests/Benchmark.class",
            )
                .unwrap();

            let mut parser = black_box(ClassFileParser::new(black_box(&file)));
            black_box(parser.parse().unwrap());
        })
    });
}

criterion_group!(group, intentional_large);
criterion_main!(group);
