use crate::error::SniprunError;
use crate::interpreter::{Interpreter, SupportLevel};
use crate::DataHolder;

use std::fs::{write, DirBuilder, File};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct Rust {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,
    rust_work_dir: String,
    bin_path: String,
    main_file_path: String,
}

impl Interpreter for Rust {
    fn new_with_level(data: DataHolder, support_level: SupportLevel) -> Box<Rust> {
        let rwd = data.work_dir.clone() + "/rust-original";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&rwd)
            .expect("Could not create directory for rust-original");
        let mfp = rwd.clone() + "/main.rs";
        let bp = String::from(&mfp[..mfp.len() - 3]);
        Box::new(Rust {
            data,
            support_level,
            code: String::from(""),
            rust_work_dir: rwd,
            bin_path: bp,
            main_file_path: mfp,
        })
    }

    fn get_supported_languages() -> Vec<String> {
        vec![
            String::from("rust"),
            String::from("rust-lang"),
            String::from("rs"),
        ]
    }

    fn get_name() -> String {
        String::from("rust-original")
    }

    fn get_current_level(&self) -> SupportLevel {
        self.support_level
    }
    fn set_current_level(&mut self, level: SupportLevel) {
        self.support_level = level;
    }

    fn get_data(&self) -> DataHolder {
        self.data.clone()
    }

    fn get_max_support_level() -> SupportLevel {
        SupportLevel::Unsupported
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
        {
            self.code = self.data.current_bloc.clone();
        } else if !self.data.current_line.replace(" ", "").is_empty() {
            self.code = self.data.current_line.clone();
        } else {
            self.code = String::from("");
        }
        Ok(())
    }

    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        self.code = String::from("fn main() {") + &self.code + "}";
        Ok(())
    }

    fn build(&mut self) -> Result<(), SniprunError> {
        //write code to file
        let mut file =
            File::create(&self.main_file_path).expect("Failed to create file for rust-original");
        write(&self.main_file_path, &self.code).expect("Unable to write to file for rust-original");
        let output = Command::new("rustc")
            .arg("--out-dir")
            .arg(&self.rust_work_dir)
            .arg(&self.main_file_path)
            .output()
            .expect("Unable to start process");

        //TODO if relevant, return the error number (parse it from stderr)
        if !output.status.success() {
            return Err(SniprunError::CompilationError("".to_string()));
        } else {
            return Ok(());
        }
    }

    fn execute(&mut self) -> Result<String, SniprunError> {
        let output = Command::new(&self.bin_path)
            .output()
            .expect("Unable to start process");
        if output.status.success() {
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
    }
}