use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_pcap_parsing(_c: &mut Criterion) {
    // Placeholder benchmark for pcap parsing
    // Will be implemented when actual parsing logic is added
}

criterion_group!(benches, benchmark_pcap_parsing);
criterion_main!(benches);
