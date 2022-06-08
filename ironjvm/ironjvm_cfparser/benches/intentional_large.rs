use criterion::{criterion_group, criterion_main, Criterion};
use ironjvm_cfparser::ClassFileParser;

fn intentional_large(criterion: &mut Criterion) {
    criterion.bench_function("Intentional Large", |bencher| {
        let file =
            std::fs::read("../test_classes/com/github/htgazurex1212/ironjvm/tests/Benchmark.class")
                .unwrap();

        bencher.iter(|| {
            let mut parser = ClassFileParser::new(&file);
            parser.parse().unwrap();
        })
    });
}

criterion_group!(group, intentional_large);
criterion_main!(group);
