use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::PathBuf;
use assert_fs::fixture::TempDir;

fn create_test_files(dir: &std::path::Path, count: usize) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for i in 0..count {
        let file_path = dir.join(format!("test_file_{}.rs", i));
        let content = format!(
            r#"//! Test Function {}
fn test_function_{}() {{
    println!("Hello from function {}");
}}

//! Another Function
fn another_function_{}() {{
    println!("Another function {}", i);
}}
"#,
            i, i, i, i, i
        );
        fs::write(&file_path, content).unwrap();
        files.push(file_path);
    }
    files
}

fn benchmark_file_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let test_files = create_test_files(temp_dir.path(), 100);

    c.bench_function("file_io_sequential", |b| {
        b.iter(|| {
            let files = std::hint::black_box(&test_files);
            let _result: Vec<_> = files
                .iter()
                .map(|file| {
                    let content = fs::read_to_string(file).unwrap();
                    (file, content)
                })
                .collect();
        })
    });

    c.bench_function("file_io_chunked", |b| {
        b.iter(|| {
            let files = std::hint::black_box(&test_files);
            let chunk_size = 10;
            let mut result = Vec::new();
            
            for chunk in files.chunks(chunk_size) {
                for file in chunk {
                    let content = fs::read_to_string(file).unwrap();
                    result.push((file, content));
                }
            }
            result
        })
    });
}

criterion_group!(benches, benchmark_file_processing);
criterion_main!(benches);
