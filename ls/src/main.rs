use ls::Config;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let path = Config::new(&args);
    path.run();
}
