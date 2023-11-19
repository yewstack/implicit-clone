use implicit_clone::unsync::*;

fn main() {
    divan::main();
}

#[divan::bench(sample_size = 10000000)]
fn from_iter_empty_collections(bencher: divan::Bencher) {
    bencher.bench_local(move || {
        let _: IArray<u32> = divan::black_box(Vec::new()).into_iter().collect();
    });
}

#[divan::bench(sample_size = 10000000)]
fn from_iter_collection_with_single_element(bencher: divan::Bencher) {
    bencher.bench_local(move || {
        let _: IArray<u32> = divan::black_box(vec![42]).into_iter().collect();
    });
}
