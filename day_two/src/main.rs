use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env.args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments.")
    }

    let path = &args[1];
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Could not open {}: {}", path.display(),
                                                    why.description()),
    };

    let mut input = String::new()
    match file.read_to_string(&mut input) {
        Ok(_) => println!("Read {}", path.display()),
        Err(why) => panic!("Could not read {}: {}", path.display(),
                                                    why.description()),
    };
    let mut input = input.split(",");
    /* OpCodes - 
        0 - Add
        1 - Mult
        99 - End */
    
}
