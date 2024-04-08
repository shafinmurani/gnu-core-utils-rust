use std::fs;

pub struct Config<'a> {
    dir_names: &'a Vec<String>,
}

impl Config<'_>{
    pub fn new(args: &Vec<String>) -> Config {
        Config { dir_names : args }
    }

    pub fn not_enough_arguments(){
        eprintln!("mkdir: missing operand");
        eprintln!("for help: mkdir --help");
    }

    pub fn help() {
        eprintln!("This is a cheap little ripoff of the mkdir utility in the GNU core utilities, the catch is I built it in Rust. Check out more of my work at https://shafinmurani.github.io");
        eprintln!("To use this util: mkdir folder_name1 folder_name2 .. folder_nameN");
    }

    fn create_dir(dir: String) -> std::io::Result<()> {
        fs::create_dir_all(dir)?;
        Ok(())
    }

    pub fn run(&self){
        if self.dir_names.len() == 0 {
            Self::not_enough_arguments();
        } else if self.dir_names.contains(&String::from("--help")){
            Self::help();
        } else {
            for dir in self.dir_names {
                let result = Self::create_dir(dir.to_string()); 
                match result {
                    Ok(()) => {},
                    Err(e) => eprintln!("Application Error : {}",e),
                };
            }
        }
    }
}
