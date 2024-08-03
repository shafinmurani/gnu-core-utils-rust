use std::fs;
use std::path::PathBuf;
use std::process;

pub struct Config<'a> {
    pub path: &'a Vec<String>,
}

impl Config<'_> {
    pub fn new(args: &Vec<String>) -> Config {
        Config { path: args }
    }

    pub fn run(&self) {
        if self.path.len() == 0 {
            let dir = fs::read_dir("./").unwrap();

            for item in dir {
                let metadata = PathBuf::from(&item.as_ref().unwrap().path());
                if metadata.is_dir() {
                    print!("{}/ \t", item.unwrap().path().display());
                } else {
                    print!("{} \t", item.unwrap().path().display());
                }
            }
        } else {
            for paths in self.path {
                let dir = fs::read_dir(paths).unwrap_or_else(|e| {
                    eprintln!("Application error: {} ", e);
                    process::exit(1);
                });
                println!("Files in {}:", paths);
                for item in dir {
                    let metadata = PathBuf::from(&item.as_ref().unwrap().path());
                    if metadata.is_dir() {
                        print!("{}/ \t", item.unwrap().path().display());
                    } else {
                        print!("{} \t", item.unwrap().path().display());
                    }
                }
                println!("");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_with_no_arguments() {
        let args: Vec<String> = Vec::new();
        let config = Config::new(&args);
        config.run();
    }

    #[test]
    fn test_run_with_arguments() {
        let args: Vec<String> = vec![".".to_string()];
        let config = Config::new(&args);
        config.run();
    }

    #[test]
    fn test_run_with_multiple_arguments() {
        let args: Vec<String> = vec!["/".to_string(), ".".to_string()];
        let config = Config::new(&args);
        config.run();
    }
}
