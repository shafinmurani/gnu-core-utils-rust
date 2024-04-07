use std::path::Path;
use std::{fs, io};

pub struct Config<'a> {
    pub files: &'a Vec<String>,
}

impl Config<'_> {
    pub fn new(args: &Vec<String>) -> Config {
        Config { files: args }
    }

    fn not_enough_arguments() {
        eprintln!("mkdir: missing operand");
        eprintln!("For help use: cp --help");
    }

    fn help() {
        eprintln!("This is a cheap little clone of the cp utility in the GNU core utilities, the catch is that I made it in rust, check out more of my work at medium: https://shafinmurani.medium.com");
        eprintln!("This is recursive by default so dont worry about anything, just run it :D");
        eprintln!("To use this util: cp source destination/folder_name");
    }

    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                Self::copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    pub fn run(&self) {
        if self.files.len() == 0 {
            Self::not_enough_arguments();
        } else if self.files.contains(&String::from("--help")) {
            Self::help();
        } else {
            let result = Self::copy_dir_all(self.files[0].clone(), self.files[1].clone());
            match result {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Application error: {}", e);
                }
            };
        }
    }
}
