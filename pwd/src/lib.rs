pub mod dir_get {
    use std::env;
    use std::path::PathBuf;

    fn get_working_directory() -> std::io::Result<PathBuf>{
        env::current_dir()
    }

    pub fn print_working_dir(){
        let result = get_working_directory();
        match result {
            Ok(path_buf) => println!("{}",path_buf.as_path().display()),
            Err(e) => eprintln!("Application Error: {}",e),
        };
    }
}
