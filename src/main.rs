use std::env;
use std::process;

extern crate rust_grep;
use rust_grep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(| err | {
                          println!("Problem parsing arguments {}", err);
                          process::exit(1);
                      });
    
    if let Err(e) = rust_grep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    };
}



