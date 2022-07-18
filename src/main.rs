use std::{fs, io};
use std::io::Write;
use clap::{Parser};
use String;
use gencpp::{qprint, Mode, CppFile};


#[derive(Parser)]
struct CLI {
    /// Project name
    project_name: String,
    /// File creation type
    #[clap(arg_enum, value_parser, default_value_t = Mode::Project)]
    mode: Mode,
    /// Quiet mode
    #[clap(short, action, default_value_t = false)]
    quiet: bool,
}

impl CLI {
    fn show_args(&self) {
        println!("Args:\n\tName: {}\n\tMode: {:?}\n\tQuiet: {}", self.project_name, self.mode, self.quiet);
    }

    fn run(&self) -> Result<(), io::Error> {
        match self.mode {
            Mode::Project => {
                fs::create_dir(&self.project_name)?;
                qprint("Created folder.", self.quiet);

                let mut file_main = fs::File::create(format!("{}/main.cpp", &self.project_name))?;
                qprint("Created main file.", self.quiet);

                // File creation
                let mut file_content = CppFile::new(&self.project_name, self.mode);
                file_content.add_lib("iostream".to_string());

                file_main.write(file_content.create_main().as_bytes())?;
            }
            Mode::Cpp => {
                // Cpp creation
                let mut file_cpp = fs::File::create(format!("{}.cpp", self.project_name))?;
                let mut file_content = CppFile::new(&self.project_name, self.mode);

                file_content.add_lib("iostream".to_string());
                file_cpp.write(file_content.create_cpp().as_bytes())?;

                // Header creation
                let mut file_header = fs::File::create(format!("{}.h", self.project_name))?;
                let mut file_header_content = CppFile::new(&self.project_name, self.mode);

                file_header.write(file_header_content.create_header().as_bytes())?;
            }
            Mode::Header => {
                let mut file_header = fs::File::create(format!("{}.h", self.project_name))?;
                let mut file_content = CppFile::new(&self.project_name, self.mode);

                file_header.write(file_content.create_header().as_bytes())?;
            }
        }

        Ok(())
    }
}

fn main() {
    let cli = CLI::parse();

    cli.show_args();

    match cli.run() {
        Ok(_) => { qprint("Done.", cli.quiet) },
        Err(e) => { eprintln!("Error: {}", e) }
    }
}
