use std::fs::File;
use std::io::Read;

pub struct Config<'a>{
    file_names: &'a Vec<String>,
}

impl Config<'_>{
    pub fn new(args: &Vec<String>) -> Config{
        Config{ file_names: args }
    }

    fn help(){
        eprintln!("Welcome to cat in Rust, This is a little copy of the cat utility present in the GNU core utilities. I made this to practice my rust skills. Check out my articles at https://shafinmurani.medium.com");
        eprintln!("To use: cat file_name1 file_name2 ... file_nameN");
    }

    fn not_enough_arguments(){
        eprintln!("cat: missing operand");
        eprintln!("for help: cat --help");
    }

    fn read_document(file: String) -> std::io::Result<String>{
        let mut file_buffer = File::open(file)?;
        let mut file_content = String::new();

        file_buffer.read_to_string(&mut file_content)?;
        Ok(file_content)
    }

    pub fn run(&self){
        if self.file_names.len() == 0 {
            Self::not_enough_arguments();
        } else if self.file_names.contains(&String::from("--help")) {
            Self::help();
        } else {
            for file in self.file_names {
               let result = Self::read_document(file.to_string());
                match result {
                    Ok(r) => println!("{}",r),
                    Err(e) => eprintln!("Application Error: `{}` {}",file,e)
                };
            }
        }
    }
}
