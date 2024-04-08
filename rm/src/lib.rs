use std::{fs, io};

pub struct Config<'a> {
    pub files: &'a Vec<String>,
}

impl Config<'_> {
    pub fn new(args: &Vec<String>) -> Config {
        Config { files: args }
    }

    fn not_enough_arguments() {
        eprintln!("rm: missing operand");
        eprintln!("For help use: rm --help");
    }

    fn help() {
        eprintln!("This is a cheap little clone of the rm utility in the GNU core utilities, the catch is that I made it in rust, check out more of my work at medium: https://shafinmurani.medium.com");
        eprintln!("This is recursive by default so dont worry about anything, just run it :D");
        eprintln!("To use this util: rm path1 path2 ... pathN");
    }

    fn rm_dir_all(src: &String) -> io::Result<()> {
        fs::remove_dir_all(src)?;
        Ok(())
    }

    pub fn run(&self) {
        if self.files.len() == 0 {
            Self::not_enough_arguments();
        } else if self.files.contains(&String::from("--help")) {
            Self::help();
        } else {
            for entry in self.files {
                let result = Self::rm_dir_all(entry);
                match result {
                    Ok(()) => {}
                    Err(e) => eprintln!("Application Error : `{}`  {}", entry , e),
                };
            }
        }
    }
}
