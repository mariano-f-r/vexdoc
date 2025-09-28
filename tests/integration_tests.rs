//! Integration tests for VexDoc
//! 
//! These tests run the actual VexDoc binary and make sure it behaves
//! correctly from a user's perspective. They test the full workflow
//! from init to generate, making sure everything works together.

use assert_cmd::Command;
use assert_fs::fixture::TempDir;
use std::fs;

#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("vexdoc").unwrap();
    
    let output = cmd
        .args(&["init", "--dir", temp_dir.path().to_str().unwrap()])
        .assert()
        .success();
    
    // Check if config file was created
    let config_path = temp_dir.path().join("VexDoc.toml");
    assert!(config_path.exists());
    
    // Check config content
    let config_content = fs::read_to_string(config_path).unwrap();
    assert!(config_content.contains("inline_comments"));
    assert!(config_content.contains("multi_comments"));
    assert!(config_content.contains("ignored_dirs"));
    assert!(config_content.contains("file_extensions"));
}

#[test]
fn test_generate_command_with_config() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create config file
    fs::write(
        temp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs"]
"#,
    ).unwrap();
    
    // Create test file
    fs::write(
        temp_dir.path().join("test.rs"),
        r#"//! Test Function
/*startsummary
This is a test function.
endsummary*/
fn test_function() {
    println!("Hello, world!");
}
// ENDVEXDOC
"#,
    ).unwrap();
    
    let mut cmd = Command::cargo_bin("vexdoc").unwrap();
    let output = cmd
        .current_dir(temp_dir.path())
        .args(&["generate"])
        .assert()
        .success();
    
    // Check if documentation was created
    let doc_file = temp_dir.path().join("docs").join("test.rs.html");
    assert!(doc_file.exists());
}

#[test]
fn test_generate_command_verbose() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create config file
    fs::write(
        temp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs"]
"#,
    ).unwrap();
    
    // Create test file
    fs::write(
        temp_dir.path().join("test.rs"),
        r#"//! Test Function
fn test_function() {
    println!("Hello, world!");
}
"#,
    ).unwrap();
    
    let mut cmd = Command::cargo_bin("vexdoc").unwrap();
    let output = cmd
        .current_dir(temp_dir.path())
        .args(&["generate", "--verbose"])
        .assert()
        .success();
    
    // Check if verbose output was produced
    let output_str = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(output_str.contains("Beginning documentation"));
}

#[test]
fn test_generate_command_quiet() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create config file
    fs::write(
        temp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs"]
"#,
    ).unwrap();
    
    // Create test file
    fs::write(
        temp_dir.path().join("test.rs"),
        r#"//! Test Function
fn test_function() {
    println!("Hello, world!");
}
"#,
    ).unwrap();
    
    let mut cmd = Command::cargo_bin("vexdoc").unwrap();
    let output = cmd
        .current_dir(temp_dir.path())
        .args(&["generate", "--quiet"])
        .assert()
        .success();
    
    // Check if quiet mode suppressed output
    let output_str = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(!output_str.contains("Beginning documentation"));
}

#[test]
fn test_generate_command_with_specific_files() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create config file
    fs::write(
        temp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs"]
"#,
    ).unwrap();
    
    // Create test files
    fs::write(
        temp_dir.path().join("test1.rs"),
        r#"//! Test Function 1
fn test_function1() {
    println!("Hello from test1!");
}
"#,
    ).unwrap();
    
    fs::write(
        temp_dir.path().join("test2.rs"),
        r#"//! Test Function 2
fn test_function2() {
    println!("Hello from test2!");
}
"#,
    ).unwrap();
    
    let mut cmd = Command::cargo_bin("vexdoc").unwrap();
    let output = cmd
        .current_dir(temp_dir.path())
        .args(&["generate", "--files", "test1.rs"])
        .assert()
        .success();
    
    // Check if only test1.rs was documented
    let doc1 = temp_dir.path().join("docs").join("test1.rs.html");
    let doc2 = temp_dir.path().join("docs").join("test2.rs.html");
    
    assert!(doc1.exists());
    assert!(!doc2.exists());
}

#[test]
fn test_error_on_missing_config() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("vexdoc").unwrap();
    let output = cmd
        .current_dir(temp_dir.path())
        .args(&["generate"])
        .assert()
        .failure();
    
    // Check if appropriate error message was shown
    let output_str = String::from_utf8(output.get_output().stderr.clone()).unwrap();
    assert!(output_str.contains("could not read config file"));
}

#[test]
fn test_error_on_invalid_config() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create invalid config file
    fs::write(
        temp_dir.path().join("VexDoc.toml"),
        r#"invalid_toml_content = 
"#,
    ).unwrap();
    
    let mut cmd = Command::cargo_bin("vexdoc").unwrap();
    let output = cmd
        .current_dir(temp_dir.path())
        .args(&["generate"])
        .assert()
        .failure();
    
    // Check if appropriate error message was shown
    let output_str = String::from_utf8(output.get_output().stderr.clone()).unwrap();
    assert!(output_str.contains("an error has occurred at runtime"));
}
