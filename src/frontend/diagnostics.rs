use codespan::{FileId, Files};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug)]
pub struct ErrorReporter {
    files: Files<String>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            files: Files::new(),
        }
    }

    pub fn report(&self, diagnostic: Diagnostic<FileId>) {
        let writer = codespan_reporting::term::termcolor::StandardStream::stderr(
            codespan_reporting::term::termcolor::ColorChoice::Auto,
        );
        let config = codespan_reporting::term::Config::default();
        codespan_reporting::term::emit(&mut writer.lock(), &config, &self.files, &diagnostic).unwrap();
    }

    pub fn add_file(&mut self, name: String, source: String) -> FileId {
        self.files.add(name, source)
    }
}