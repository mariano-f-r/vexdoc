use crate::errors::{SubcommandError, UserErrorKind};
use build_html::{Container, ContainerType, Html, HtmlContainer, HtmlElement, HtmlPage, HtmlTag};
use serde::Deserialize;
use std::ffi::OsString;
use std::fs::{self, DirBuilder, File};
use std::io::{self, ErrorKind, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

// TODO: write some awful configs to ensure that error handling is decent
#[derive(Debug, Deserialize)]
pub struct DocGenConfig {
    inline_comments: String,
    multi_comments: Vec<String>,
    ignored_dirs: Vec<PathBuf>,
    file_extensions: Vec<String>,
}

impl DocGenConfig {
    // TODO: prevent user from creating configs without multiline comments. Perhaps create
    // UserError struct and implement Error on it?
    pub fn read_config() -> Result<DocGenConfig, SubcommandError> {
        let config =
            fs::read_to_string("./VexDoc.toml").map_err(|e| SubcommandError::FileReadError(e))?;
        // Ideally the serde stuff should not fail
        let config: DocGenConfig =
            toml::from_str(&config).map_err(|e| SubcommandError::UserError {
                causes: "fix missing values/incorrect syntax".into(),
                source: Some(Box::new(e)),
                kind: UserErrorKind::Config,
                file: "./VexDoc.toml".into(),
            })?;

        let mut error = false;
        let mut solution = String::new();

        if config.multi_comments.len() < 1 {
            error = true;
            solution.push_str("there must be more than zero multiline comment delmiter in the config. Add multiline comments.\n")
        }
        if config.inline_comments.is_empty() {
            error = true;
            solution.push_str("there must be an inline comment delimeter in the config. Add a single line comment delimeter.\n")
        }
        if config.file_extensions.is_empty() {
            error = true;
            solution.push_str("config must have at least one file extension. Add a file extension without the period: eg 'py', 'h' or 'c'\n")
        }

        if error {
            return Err(SubcommandError::UserError {
                causes: solution,
                source: None,
                kind: UserErrorKind::Config,
                file: "VexDoc.toml".into(),
            });
        }

        Ok(config)
    }

    pub fn get_files(&self) -> Result<Vec<PathBuf>, SubcommandError> {
        match DocGenConfig::get_files_helper(".".into(), &self.ignored_dirs) {
            Err(e) => return Err(SubcommandError::FileReadError(e)),
            Ok(mut files) => {
                files.retain(|f| match f.extension() {
                    None => false,
                    // Find any matches to the file extension in our config
                    Some(ext) => self
                        .file_extensions
                        .iter()
                        .any(|e| &OsString::from(e) == ext),
                });
                Ok(files)
            }
        }
    }

    fn get_files_helper(path: PathBuf, ign: &Vec<PathBuf>) -> io::Result<Vec<PathBuf>> {
        let mut output = Vec::<PathBuf>::new();
        let current_directory = fs::read_dir(path)?;
        for item in current_directory {
            let entry = item?;
            if entry.file_type()?.is_dir() {
                if !ign.iter().any(|i| &entry.file_name() == i.as_os_str()) {
                    let new_files = DocGenConfig::get_files_helper(entry.path(), &ign)?;
                    output.extend(new_files.into_iter())
                } else {
                    println!("Ignoring {}", entry.path().display());
                }
            } else {
                let path = entry.path();
                if !path.starts_with("./.git") && !path.ends_with(".gitignore") {
                    output.push(entry.path())
                }
            }
        }
        Ok(output)
    }
    pub fn create_config(mut dir: PathBuf) -> Result<(), io::Error> {
        let content = r#"inline_comments = ""
multi_comments = []
ignored_dirs = []
file_extensions = []"#;
        dir.push("VexDoc.toml");
        let mut file = File::create_new("./VexDoc.toml")?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

pub fn document(conf: DocGenConfig, files: Vec<PathBuf>) -> Result<(), SubcommandError> {
    if let Err(e) = DirBuilder::new().create("./docs") {
        match e.kind() {
            // if it already exists we don't need to worry about it not being created
            // TODO: Consider refactor and having a genuine error for this?
            ErrorKind::AlreadyExists => (),
            _ => return Err(SubcommandError::GenerationError(Box::new(e))),
        };
    }
    let new_files: Vec<&Path> = files
        .iter()
        .map(|p| p.strip_prefix("./").unwrap_or(p))
        .collect();

    let mut notices = Vec::<String>::new();

    for path in &new_files {
        println!("Documenting {} ...", path.display());
        if !create_doc(path, &conf)? {
            notices.push(format!(
                "NOTICE: {} contained no annotations, so nothing was actually written to its documentation. Ensure it has correct annotations",
                path.display()
            ))
        }
        println!("Done")
    }

    if files.len() == 0 {
        notices.push("NOTICE: no files were documented. Ensure your config has the appropriate file extensions".into());
    }

    for n in notices {
        println!("{}", n);
    }

    Ok(())
}

// Maybe??? give it a try later
// ok we will, State machine to help determine what exactly to put
// holy shit thank you me
#[derive(Debug, Clone, Copy)]
enum ParserState {
    Ignore,
    FileSummary,
    Title,
    ItemSummary,
    Code,
}

fn create_doc(old_path: &Path, conf: &DocGenConfig) -> Result<bool, SubcommandError> {
    let content = fs::read_to_string(old_path).map_err(|e| SubcommandError::FileReadError(e))?;
    let mut has_vexdoc = false;
    let mut no_filesummary = false;
    // let mut proper = true;
    let single_multiline = conf.multi_comments.get(1).is_none();
    let mut body = Container::new(ContainerType::Div)
        .with_attributes([("class", "container")])
        .with_header(1, format!("Documentation from {}", old_path.display()));

    let mut state = ParserState::Ignore;
    let mut included = Vec::<&str>::new();

    for line in content.lines() {
        match state {
            ParserState::Ignore => {
                if line.starts_with(&format!("{}!", conf.inline_comments)) {
                    no_filesummary = true;
                    has_vexdoc = true;
                    state = ParserState::Title;
                    // Line is guaranteed to have at least n+1 characters due to above check
                    body.add_header(2, &line[(conf.inline_comments.len() + 1)..].trim_start());
                } else if !no_filesummary
                    && line.starts_with(&format!("{}filesummary", conf.multi_comments[0]))
                // will fail if no multiline comments are present
                {
                    has_vexdoc = true;
                    state = ParserState::FileSummary;
                }
            }
            ParserState::FileSummary => {
                if line.starts_with(&format!(
                    "endsummary{}",
                    if single_multiline {
                        &conf.multi_comments[0]
                    } else {
                        &conf.multi_comments[1]
                    }
                )) {
                    body.add_html(
                        HtmlElement::new(HtmlTag::ParagraphText)
                            .with_attribute("class", "comment")
                            .with_child(included.join(" ").into()),
                    );
                    included.clear();
                    state = ParserState::Ignore;
                } else {
                    included.push(line);
                }
            }
            ParserState::Title => {
                if line.starts_with(&format!("{}startsummary", conf.multi_comments[0])) {
                    //will fail if no multiline comments are present
                    state = ParserState::ItemSummary;
                } else {
                    return Err(SubcommandError::UserError {
                        causes: "section titles must be followed by a summary".into(),
                        source: None,
                        kind: UserErrorKind::Annotations,
                        file: old_path.into(),
                    });
                }
            }
            ParserState::ItemSummary => {
                if line.starts_with(&format!(
                    "endsummary{}",
                    if single_multiline {
                        &conf.multi_comments[0]
                    } else {
                        &conf.multi_comments[1]
                    }
                )) {
                    // TODO: figure out a better way to test syntax of the annotations
                    body.add_html(
                        HtmlElement::new(HtmlTag::ParagraphText)
                            .with_attribute("class", "comment")
                            .with_child(included.join(" ").into()),
                    );
                    included.clear();
                    state = ParserState::Code;
                } else {
                    included.push(line);
                }
            }
            ParserState::Code => {
                if line
                    .replace(" ", "")
                    .starts_with(&format!("{}ENDVEXDOC", conf.inline_comments))
                {
                    body.add_html(HtmlElement::new(HtmlTag::PreformattedText).with_html(
                        HtmlElement::new(HtmlTag::CodeText).with_child(included.join("\n").into()),
                    ));
                    included.clear();
                    state = ParserState::Ignore;
                } else {
                    included.push(line);
                }
            }
        }
    }

    // This should never fail
    // TODO: Ensure this never fails

    fs::write(
        Path::new("./docs")
            .join(
                old_path
                    .display()
                    .to_string()
                    .replace(".", "-")
                    .replace("/", "_")
                    .replace("\\", "_"),
            )
            .with_extension("html"),
        doc_boilerplate_memo(&old_path)
            .with_container(body)
            .with_script_literal(r#"hljs.highlightAll();"#)
            .to_html_string(),
    )
    .map_err(|e| SubcommandError::GenerationWriteError(e))?;
    Ok(has_vexdoc)
}

fn doc_boilerplate_memo(path: &impl Deref<Target = Path>) -> HtmlPage {
    HtmlPage::new()
        .with_title(format!("Docs from {}", path.display()))
        .with_style(include_str!("styles.css"))
        .with_stylesheet(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css",
        )
        .with_script_link(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js",
        )
}

// fn clean_up() {
//     todo!("Write clean up function to remove orphaned docs")
// }
