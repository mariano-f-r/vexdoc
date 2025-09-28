use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::PathBuf;
use assert_fs::fixture::TempDir;
use vexdoc::docgen::DocGenConfig;

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
    
    let config = DocGenConfig {
        inline_comments: "//".to_string(),
        multi_comments: vec!["/*".to_string(), "*/".to_string()],
        ignored_dirs: vec![],
        file_extensions: vec!["rs".to_string()],
    };

    c.bench_function("process_files_sequential", |b| {
        b.iter(|| {
            let files = black_box(&test_files);
            let _result: Vec<_> = files
                .iter()
                .map(|file| {
                    let content = fs::read_to_string(file).unwrap();
                    (file, content)
                })
                .collect();
        })
    });

    c.bench_function("process_files_parallel", |b| {
        b.iter(|| {
            use rayon::prelude::*;
            let files = black_box(&test_files);
            let _result: Vec<_> = files
                .par_iter()
                .map(|file| {
                    let content = fs::read_to_string(file).unwrap();
                    (file, content)
                })
                .collect();
        })
    });
}

criterion_group!(benches, benchmark_file_processing);
criterion_main!(benches);
