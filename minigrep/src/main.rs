// To read command-line args
use std::env;
use std::process;
use minigrep::Config;

fn main() {

    // Read command-line args
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{eprintln!("Problem passing args: {}", err); process::exit(1)});

    minigrep::run(config).unwrap();

    //println!("Args: {:?}", args);

}
