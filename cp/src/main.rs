use cp::Config;
use std::env;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let config = Config::new(&args);
    config.run();
}
