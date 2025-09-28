//! Test suite for the docgen module
/*startsummary
These tests make sure our documentation generation actually works.
I've tried to cover the main happy paths and a few edge cases,
but if you find bugs, feel free to add more tests!
endsummary*/

use std::{env, error::Error};

use super::*;
use assert_fs::fixture::TempDir;
use rand::Rng;

/// Generates a random alphanumeric string of the specified length
fn generate_random_string(rng: &mut impl Rng, len: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..len)
        .map(|_| CHARS[rng.random_range(0..CHARS.len())] as char)
        .collect()
}

/// Creates a bunch of random files and directories for testing
/// 
/// This is used to test that our file discovery logic works correctly
/// even when there are lots of files scattered around. It creates a
/// mix of files and directories at different depths to make things
/// interesting.
fn rand_dir_entries(path: &Path) -> Vec<PathBuf> {
    let mut rng = rand::rng();
    let item_count: usize = rng.random_range(1..31);
    let mut paths = Vec::<PathBuf>::new();
    for i in 0..item_count {
        let item_depth: usize = rng.random_range(0..3);
        match item_depth {
            0 => {
                let item_name = format!("tempfile{i}");
                let full_path = path.join(item_name);
                dbg!(&full_path);
                File::create_new(&full_path).expect("Should be able to create file");
                paths.push(full_path);
            }
            1 => {
                let item_name = format!("tempfile{i}");
                let parent = generate_random_string(&mut rng, 7);
                let full_path = path.join(Path::new(&parent));
                DirBuilder::new()
                    .recursive(true)
                    .create(&full_path)
                    .expect("Should be able to create parent dir");

                let full_path = full_path.join(item_name);
                dbg!(&full_path);
                File::create_new(&full_path).expect("Should be able to create file");
                paths.push(full_path);
            }
            2 => {
                let item_name = format!("tempfile{i}");
                let parent = generate_random_string(&mut rng, 7);
                let parent2 = generate_random_string(&mut rng, 7);
                let full_path = path.join(Path::new(&parent).join(parent2));
                DirBuilder::new()
                    .recursive(true)
                    .create(&full_path)
                    .expect("Should be able to create parent dir");

                let full_path = full_path.join(item_name);
                dbg!(&full_path);
                File::create_new(&full_path).expect("Should be able to create file");
                paths.push(full_path);
            }
            _ => (),
        }
    }
    paths
}

#[test]
fn random_get_all_files() -> Result<(), Box<dyn Error>> {
    let temporary_dir = TempDir::new()?;
    let mut test_files = rand_dir_entries(temporary_dir.path());

    let mut files = DocGenConfig::get_files_helper(temporary_dir.path().into(), &vec![])?;

    test_files.sort();
    files.sort();
    dbg!(&test_files);
    dbg!(&files);

    assert_eq!(test_files, files);

    Ok(())
}

#[test]
fn reads_config() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new()?;

    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["c", "h"]
"#,
    )?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config().expect("Should be able to read config");

    assert!(
        conf.inline_comments == "//".to_string()
            && conf.multi_comments == vec!["/*".to_string(), "*/".to_string()]
            && conf.ignored_dirs.len() == 0
            && conf.file_extensions == vec!["c".to_string(), "h".to_string()]
    );

    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn deny_bad_config() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new().expect("Should be able to create temp dir. Check permissions");

    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = ""
multi_comments = []
ignored_dirs = []
file_extensions = []
"#,
    )?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config();
    dbg!(&conf);

    assert!(conf.is_err());

    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn error_on_incorrect_config() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new().expect("Should be able to create temp dir. Check permissions");

    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = ""
multi_comments = []
file_extensions = []
"#,
    )?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config();

    dbg!(&conf);

    if let SubcommandError::UserError {
        causes: _,
        source: _,
        kind: _,
        file: _,
    } = conf.unwrap_err()
    {
        // Restore original directory
        env::set_current_dir(original_dir)?;
        Ok(())
    } else {
        // Restore original directory
        env::set_current_dir(original_dir)?;
        panic!("Did not error correctly");
    }
}

#[test]
fn test_document_generation() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new()?;
    
    // Create test config
    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs"]
"#,
    )?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    // Create test file with documentation
    let test_file = "test.rs";
    fs::write(
        test_file,
        r#"//! Test Function
/*startsummary
This is a test function that does something useful.
endsummary*/

fn test_function() {
    println!("Hello, world!");
}
// ENDVEXDOC
"#,
    )?;

    let conf = DocGenConfig::read_config()?;
    let files = conf.get_files()?;
    
    // Test document generation
    let result = document(conf, files, false, false);
    if let Err(e) = &result {
        eprintln!("Document generation failed: {}", e);
    }
    assert!(result.is_ok());

    // Check if documentation was created
    let doc_file = Path::new("docs").join("test-rs.html");
    assert!(doc_file.exists());

    // Check if the documentation contains expected content
    let doc_content = fs::read_to_string(doc_file)?;
    assert!(doc_content.contains("Test Function"));
    assert!(doc_content.contains("This is a test function"));

    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn test_ignored_directories() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new()?;
    
    // Create test config with ignored directory
    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = ["ignored"]
file_extensions = ["rs"]
"#,
    )?;

    // Create files in ignored directory
    fs::create_dir_all(tmp_dir.path().join("ignored"))?;
    fs::write(
        tmp_dir.path().join("ignored").join("ignored.rs"),
        r#"//! This should be ignored
fn ignored_function() {
    println!("This should not be documented");
}
"#,
    )?;

    // Create file in root directory
    fs::write(
        tmp_dir.path().join("included.rs"),
        r#"//! This should be included
fn included_function() {
    println!("This should be documented");
}
"#,
    )?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config()?;
    let files = conf.get_files()?;
    
    // Should only find the included file, not the ignored one
    assert_eq!(files.len(), 1);
    assert!(files[0].to_string_lossy().contains("included.rs"));

    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn test_file_extension_filtering() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new()?;
    
    // Create test config with specific file extensions
    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs", "py"]
"#,
    )?;

    // Create files with different extensions
    fs::write(tmp_dir.path().join("test.rs"), "//! Rust file\nfn test() {}")?;
    fs::write(tmp_dir.path().join("test.py"), "#! Python file\ndef test(): pass")?;
    fs::write(tmp_dir.path().join("test.js"), "//! JavaScript file\nfunction test() {}")?;
    fs::write(tmp_dir.path().join("test.txt"), "This is a text file")?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config()?;
    let files = conf.get_files()?;
    
    // Should only find .rs and .py files
    assert_eq!(files.len(), 2);
    let file_names: Vec<String> = files.iter().map(|f| f.file_name().unwrap().to_string_lossy().to_string()).collect();
    assert!(file_names.contains(&"test.rs".to_string()));
    assert!(file_names.contains(&"test.py".to_string()));
    assert!(!file_names.contains(&"test.js".to_string()));
    assert!(!file_names.contains(&"test.txt".to_string()));

    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn test_quiet_mode() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new()?;
    
    // Create test config
    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs"]
"#,
    )?;

    // Create test file
    let test_file = tmp_dir.path().join("test.rs");
    fs::write(&test_file, "//! Test\n/*startsummary\nThis is a test function.\nendsummary*/\n\nfn test() {}\n// ENDVEXDOC")?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config()?;
    let files = conf.get_files()?;
    
    // Test quiet mode (should not panic or fail)
    let result = document(conf, files, false, true);
    assert!(result.is_ok());

    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn test_verbose_mode() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new()?;
    
    // Create test config
    fs::write(
        tmp_dir.path().join("VexDoc.toml"),
        r#"inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = []
file_extensions = ["rs"]
"#,
    )?;

    // Create test file
    let test_file = tmp_dir.path().join("test.rs");
    fs::write(&test_file, "//! Test\n/*startsummary\nThis is a test function.\nendsummary*/\n\nfn test() {}\n// ENDVEXDOC")?;

    let original_dir = env::current_dir()?;
    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config()?;
    let files = conf.get_files()?;
    
    // Test verbose mode (should not panic or fail)
    let result = document(conf, files, true, false);
    if let Err(e) = &result {
        eprintln!("Verbose mode test failed: {}", e);
    }
    assert!(result.is_ok());

    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}
