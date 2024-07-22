use std::{env, error::Error};

use super::*;
use assert_fs::fixture::TempDir;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

fn rand_dir_entries(path: &Path) -> Vec<PathBuf> {
    let item_count: usize = thread_rng().gen_range(1..31);
    let mut paths = Vec::<PathBuf>::new();
    for i in 0..item_count {
        let item_depth: usize = thread_rng().gen_range(0..3);
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
                let parent: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(|b| char::from(b))
                    .collect();
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
                let parent: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(|b| char::from(b))
                    .collect();
                let parent2: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(|b| char::from(b))
                    .collect();
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
    let test_files = rand_dir_entries(temporary_dir.path());

    let files = DocGenConfig::get_files_helper(temporary_dir.path().into(), &vec![])?;

    dbg!(&test_files);
    dbg!(&files);

    assert_eq!(test_files, files);

    Ok(())
}

#[test]
fn reads_config() {
    todo!();
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

    env::set_current_dir(tmp_dir.path())?;

    let conf = DocGenConfig::read_config();
    dbg!(&conf);

    assert!(conf.is_err());

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
        Ok(())
    } else {
        panic!("Did not error correctly");
    }
}
