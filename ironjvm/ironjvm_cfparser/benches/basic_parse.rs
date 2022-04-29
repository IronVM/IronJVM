use criterion::{criterion_group, criterion_main, Criterion};
use ironjvm_cfparser::ClassFileParser;

fn basic_parse(criterion: &mut Criterion) {
    criterion.bench_function("Hello World", |bencher| {
        bencher.iter(|| {
            let file = std::fs::read(
                "../test_classes/com/github/htgazurex1212/ironjvm/tests/HelloWorld.class",
            )
            .unwrap();

            let mut parser = ClassFileParser::new(&file);
            parser.parse().unwrap();
        })
    });
}

criterion_group!(group, basic_parse);
criterion_main!(group);
