#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]


use criterion::Criterion;
use criterion_macro::criterion;

#[criterion]
pub fn bench_asm_rdmsr(c: &mut Criterion) {
    let mut group = c.benchmark_group("asm");
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.bench_function("asm_rdmsr", |b| b.iter(|| { /*read_single_core_msr(0)*/ }));
}

// fn generate_criterion() -> Criterion<Energy> {
//     Criterion::default().with_measurement(Energy)
// }

// #[criterion(generate_criterion())]
// pub fn bench_cmd_rdmsr(c: &mut Criterion<Energy>) {
//     c.bench_function("cmd_rdmsr", |b| b.iter(|| { cmd_rdmsr() }));
// }
//
// #[criterion(generate_criterion())]
// pub fn bench_file_msr(c: &mut Criterion<Energy>) {
//     c.bench_function("file_rdmsr", |b| b.iter(|| { read_single_core_msr_file(0) }));
// }

