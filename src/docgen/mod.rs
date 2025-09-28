//! # Documentation Generation
//! 
//! This is where the magic happens. The docgen module takes your source files,
//! finds the documentation blocks you've marked up, and turns them into
//! clean HTML pages.
//! 
//! The process is pretty straightforward:
//! 1. Read the configuration to know what file types and comment styles to look for
//! 2. Walk through your project directory, skipping ignored folders
//! 3. For each source file, parse it looking for our special comment markers
//! 4. Extract the title, description, and code from each documentation block
//! 5. Generate HTML with syntax highlighting and write it to the docs folder
//! 
//! The parser uses a simple state machine to track whether we're currently
//! inside a documentation block, reading a title, or processing code. This
//! keeps the logic clean and makes it easy to handle different comment styles
//! across programming languages.

use crate::errors::{SubcommandError, UserErrorKind};
use build_html::{Container, ContainerType, Html, HtmlContainer, HtmlElement, HtmlPage, HtmlTag};
use serde::Deserialize;
use std::ffi::OsString;
use std::fs::{self, DirBuilder, File};
use std::io::{self, ErrorKind, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

#[cfg(test)]
mod tests;

/// Configuration for how VexDoc should process your project
/// 
/// This struct holds all the settings that tell VexDoc what to look for
/// and how to process your files. It's loaded from the VexDoc.toml file
/// in your project root.
/// 
/// The configuration is pretty flexible - you can tell VexDoc to process
/// any file type by setting the right comment delimiters and file extensions.
/// This makes it work with everything from C++ to Python to JavaScript.
/// 
/// # Examples
/// 
/// ```rust
/// use vexdoc::docgen::DocGenConfig;
/// use std::path::PathBuf;
/// 
/// // Typical Rust project configuration
/// let config = DocGenConfig {
///     inline_comments: "//".to_string(),
///     multi_comments: vec!["/*".to_string(), "*/".to_string()],
///     ignored_dirs: vec![PathBuf::from("target"), PathBuf::from("node_modules")],
///     file_extensions: vec!["rs".to_string()],
/// };
/// 
/// // Python project would look like this:
/// let python_config = DocGenConfig {
///     inline_comments: "#".to_string(),
///     multi_comments: vec!["\"\"\"".to_string(), "\"\"\"".to_string()],
///     ignored_dirs: vec![PathBuf::from("__pycache__")],
///     file_extensions: vec!["py".to_string()],
/// };
/// ```
#[derive(Debug, Deserialize)]
pub struct DocGenConfig {
    inline_comments: String,
    multi_comments: Vec<String>,
    ignored_dirs: Vec<PathBuf>,
    file_extensions: Vec<String>,
}

impl DocGenConfig {
    /// Loads the configuration from VexDoc.toml
    /// 
    /// This function looks for a VexDoc.toml file in the current directory,
    /// reads it, and makes sure all the required settings are present and
    /// valid. If something's wrong, it gives you a helpful error message
    /// explaining what needs to be fixed.
    /// 
    /// The validation is pretty thorough - it checks that you have comment
    /// delimiters, file extensions, and that everything is formatted correctly.
    /// No more mysterious failures because you forgot to set something!
    /// 
    /// # Returns
    /// 
    /// * `Ok(DocGenConfig)` - Configuration loaded successfully
    /// * `Err(SubcommandError)` - Something went wrong (file missing, invalid format, etc.)
    /// 
    /// # Errors
    /// 
    /// Common issues this function catches:
    /// - Missing VexDoc.toml file
    /// - Invalid TOML syntax (missing quotes, brackets, etc.)
    /// - Empty or missing required fields
    /// - File extensions with leading dots (should be "rs" not ".rs")
    /// - Incomplete multiline comment pairs
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use vexdoc::docgen::DocGenConfig;
    /// 
    /// // This will read VexDoc.toml from the current directory
    /// let config = DocGenConfig::read_config()?;
    /// println!("Found {} file types to process", config.file_extensions.len());
    /// ```
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

        let mut errors = Vec::new();
        let mut suggestions = Vec::new();

        if config.multi_comments.is_empty() {
            errors.push("No multiline comment delimiters specified".to_string());
            suggestions.push("Add multiline comment delimiters, e.g., multi_comments = [\"/*\", \"*/\"]".to_string());
        }
        if config.inline_comments.is_empty() {
            errors.push("No inline comment delimiter specified".to_string());
            suggestions.push("Add an inline comment delimiter, e.g., inline_comments = \"//\"".to_string());
        }
        if config.file_extensions.is_empty() {
            errors.push("No file extensions specified".to_string());
            suggestions.push("Add file extensions without the period, e.g., file_extensions = [\"rs\", \"py\", \"c\"]".to_string());
        }
        
        // Validate multiline comment pairs
        if config.multi_comments.len() == 1 {
            errors.push("Multiline comments must have both opening and closing delimiters".to_string());
            suggestions.push("Add both opening and closing delimiters, e.g., multi_comments = [\"/*\", \"*/\"]".to_string());
        }
        
        // Validate file extensions format
        for ext in &config.file_extensions {
            if ext.starts_with('.') {
                let error_msg = format!("File extension '{}' should not start with a period", ext);
                errors.push(error_msg);
                suggestions.push("Remove the leading period from file extensions".to_string());
            }
        }

        if !errors.is_empty() {
            let mut error_message = String::new();
            error_message.push_str("Configuration validation failed:\n\n");
            
            for (i, error) in errors.iter().enumerate() {
                error_message.push_str(&format!("{}. {}\n", i + 1, error));
            }
            
            error_message.push_str("\nSuggested fixes:\n");
            for (i, suggestion) in suggestions.iter().enumerate() {
                error_message.push_str(&format!("{}. {}\n", i + 1, suggestion));
            }
            
            return Err(SubcommandError::UserError {
                causes: error_message,
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
            Ok(files) => {
                // Filter files by extension more efficiently
                let filtered_files: Vec<PathBuf> = files
                    .into_iter()
                    .filter(|f| {
                        f.extension()
                            .map(|ext| self.file_extensions.iter().any(|e| OsString::from(e) == ext))
                            .unwrap_or(false)
                    })
                    .collect();
                Ok(filtered_files)
            }
        }
    }

    fn get_files_helper(path: PathBuf, ign: &[PathBuf]) -> io::Result<Vec<PathBuf>> {
        let mut output = Vec::new();
        let current_directory = fs::read_dir(path)?;
        
        for item in current_directory {
            let entry = item?;
            let file_name = entry.file_name();
            
            if entry.file_type()?.is_dir() {
                if !ign.iter().any(|i| &file_name == i.as_os_str()) {
                    let new_files = DocGenConfig::get_files_helper(entry.path(), ign)?;
                    output.extend(new_files);
                }
            } else {
                let entry_path = entry.path();
                if !entry_path.starts_with("./.git") && !entry_path.ends_with(".gitignore") {
                    output.push(entry_path);
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

/// Generates HTML documentation from your source files
/// 
/// This is the main workhorse function that takes your source files and
/// turns them into pretty HTML documentation. It processes files in parallel
/// for speed, shows you progress as it works, and handles errors gracefully.
/// 
/// The function creates a `docs/` directory in your project root and fills
/// it with HTML files - one for each source file that contains documentation
/// blocks. Each HTML file includes syntax highlighting and a clean, readable
/// layout.
/// 
/// # Arguments
/// 
/// * `conf` - Your project configuration (comment styles, file types, etc.)
/// * `files` - The source files to process (usually found by `get_files()`)
/// * `verbose` - Print detailed info about each file being processed
/// * `quiet` - Suppress progress bars and notices (useful for scripts)
/// 
/// # Returns
/// 
/// * `Ok(())` - All files processed successfully
/// * `Err(SubcommandError)` - Something went wrong (file read error, etc.)
/// 
/// # What it does
/// 
/// 1. Creates the `docs/` directory if it doesn't exist
/// 2. Processes each file in parallel using Rayon
/// 3. Shows a progress bar (unless quiet mode is on)
/// 4. Extracts documentation blocks from each file
/// 5. Generates HTML with syntax highlighting
/// 6. Writes the HTML files to the docs folder
/// 7. Reports any files that had no documentation blocks
/// 
/// # Examples
/// 
/// ```no_run
/// use vexdoc::docgen::{DocGenConfig, document};
/// use std::path::PathBuf;
/// 
/// let config = DocGenConfig::read_config()?;
/// let files = vec![PathBuf::from("src/main.rs")];
/// 
/// // Generate docs with progress bar
/// document(config, files, false, false)?;
/// 
/// // Or quietly for scripting
/// document(config, files, false, true)?;
/// ```
pub fn document(conf: DocGenConfig, files: Vec<PathBuf>, verbose: bool, quiet: bool) -> Result<(), SubcommandError> {
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

    if new_files.is_empty() {
        if !quiet {
            println!("NOTICE: no files were documented. Ensure your config has the appropriate file extensions");
        }
        return Ok(());
    }

    // Create progress bar only if not quiet
    let pb = if quiet {
        ProgressBar::hidden()
    } else {
        let pb = ProgressBar::new(new_files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb
    };

    // Process files in parallel
    let results: Vec<Result<bool, SubcommandError>> = new_files
        .par_iter()
        .map(|path| {
            if verbose {
                println!("Documenting {} ...", path.display());
            }
            pb.set_message(format!("Documenting {}", path.display()));
            let result = create_doc(path, &conf);
            pb.inc(1);
            if verbose {
                println!("Done with {}", path.display());
            }
            result
        })
        .collect();

    if !quiet {
        pb.finish_with_message("Documentation generation complete!");
    }

    // Collect results and notices
    let mut notices = Vec::<String>::new();
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(false) => {
                notices.push(format!(
                    "NOTICE: {} contained no annotations, so nothing was actually written to its documentation. Ensure it has correct annotations",
                    new_files[i].display()
                ));
            }
            Err(e) => return Err(e),
            Ok(true) => {} // File had documentation, no notice needed
        }
    }

    if !quiet {
        for notice in notices {
            println!("{}", notice);
        }
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
    let single_multiline = conf.multi_comments.get(1).is_none();
    let filename = old_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    let mut body = Container::new(ContainerType::Div)
        .with_attributes([("class", "container")])
        .with_header(1, filename);

    let mut state = ParserState::Ignore;
    let mut included = Vec::<&str>::with_capacity(32); // Pre-allocate for better performance
    let mut comment_buffer = String::with_capacity(256); // Buffer for comment text
    let mut code_buffer = String::with_capacity(512); // Buffer for code text
    
    // Pre-compute common strings to avoid allocations in hot loop
    let inline_prefix = format!("{}!", conf.inline_comments);
    let filesummary_prefix = format!("{}filesummary", conf.multi_comments[0]);
    let endsummary_suffix = if single_multiline {
        format!("endsummary{}", conf.multi_comments[0])
    } else {
        format!("endsummary{}", conf.multi_comments[1])
    };
    let endvexdoc = format!("{}ENDVEXDOC", conf.inline_comments);

    for line in content.lines() {
        match state {
            ParserState::Ignore => {
                if line.starts_with(&inline_prefix) {
                    no_filesummary = true;
                    has_vexdoc = true;
                    state = ParserState::Title;
                    // Line is guaranteed to have at least n+1 characters due to above check
                    body.add_header(2, &line[inline_prefix.len()..].trim_start());
                } else if !no_filesummary && line.starts_with(&filesummary_prefix) {
                    has_vexdoc = true;
                    state = ParserState::FileSummary;
                }
            }
            ParserState::FileSummary => {
                if line.starts_with(&endsummary_suffix) {
                    comment_buffer.clear();
                    for (i, line) in included.iter().enumerate() {
                        if i > 0 {
                            comment_buffer.push(' ');
                        }
                        comment_buffer.push_str(line);
                    }
                    body.add_html(
                        HtmlElement::new(HtmlTag::ParagraphText)
                            .with_attribute("class", "comment")
                            .with_child(comment_buffer.clone().into()),
                    );
                    included.clear();
                    state = ParserState::Ignore;
                } else {
                    included.push(line);
                }
            }
            ParserState::Title => {
                let startsummary_prefix = format!("{}startsummary", conf.multi_comments[0]);
                if line.starts_with(&startsummary_prefix) {
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
                if line.starts_with(&endsummary_suffix) {
                    comment_buffer.clear();
                    for (i, line) in included.iter().enumerate() {
                        if i > 0 {
                            comment_buffer.push(' ');
                        }
                        comment_buffer.push_str(line);
                    }
                    body.add_html(
                        HtmlElement::new(HtmlTag::ParagraphText)
                            .with_attribute("class", "comment")
                            .with_child(comment_buffer.clone().into()),
                    );
                    included.clear();
                    state = ParserState::Code;
                } else {
                    included.push(line);
                }
            }
            ParserState::Code => {
                if line.replace(" ", "").starts_with(&endvexdoc) {
                    code_buffer.clear();
                    for (i, line) in included.iter().enumerate() {
                        if i > 0 {
                            code_buffer.push('\n');
                        }
                        code_buffer.push_str(line);
                    }
                    body.add_html(HtmlElement::new(HtmlTag::PreformattedText).with_html(
                        HtmlElement::new(HtmlTag::CodeText).with_child(code_buffer.clone().into()),
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
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    HtmlPage::new()
        .with_title(format!("{} - VexDoc", filename))
        .with_style(include_str!("styles.css"))
        .with_stylesheet(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css",
        )
        .with_script_link(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js",
        )
        .with_meta([("name", "viewport"), ("content", "width=device-width, initial-scale=1.0")])
        .with_meta([("name", "description"), ("content", &format!("Documentation for {}", filename))])
}

// fn clean_up() {
//     todo!("Write clean up function to remove orphaned docs")
// }
