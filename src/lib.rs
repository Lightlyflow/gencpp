#![allow(dead_code)]

use std::fmt::Display;
use clap::ArgEnum;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ArgEnum, Debug)]
pub enum Mode {
    /// Project
    Project,
    /// Cpp file (will also generate header)
    Cpp,
    /// Header file
    Header,
}

pub fn qprint(arg: impl Display, quiet: bool) {
    if !quiet {
        println!("{}", arg);
    }
}

// ============================================================================================
// ============================================================================================
// ============================================================================================

/// File creator
pub struct CppFile {
    name: String,
    libs: Vec<String>,
    mode: Mode,
}

impl CppFile {
    pub fn new(name: &String, mode: Mode) -> CppFile {
        CppFile {
            name: name.clone(),
            libs: vec![],
            mode,
        }
    }

    pub fn create_main(&self) -> String {
        let mut file_content: String = "".to_string();

        for lib in self.libs.iter() {
            add_lib(&mut file_content, lib);
        }
        add_main(&mut file_content);

        file_content
    }

    pub fn create_cpp(&self) -> String {
        let mut file_content = String::new();

        for lib in self.libs.iter() {
            add_lib(&mut file_content, lib);
        }
        file_content.push_str(&format!("\n#include \"{}.h\"\n", self.name));

        file_content
    }

    pub fn create_header(&self) -> String {
        let mut file_content = String::new();

        add_header(&mut file_content);
        for lib in self.libs.iter() {
            add_lib(&mut file_content, lib);
        }

        file_content
    }

    pub fn add_lib(&mut self, lib: String) {
        self.libs.push(lib);
    }
}

fn add_lib(file_content: &mut String, lib: &String) {
    file_content.push_str(&format!("#include <{lib}>\n"));
}

fn add_main(file_content: &mut String) {
    file_content.push_str("\nint main() {\n\t\n}");
}

fn add_header(file_content: &mut String) {
    file_content.push_str("#pragma once\n\n");
}