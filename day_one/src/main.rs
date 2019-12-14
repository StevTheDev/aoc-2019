use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments.")
    }
    
    let path = &args[1];
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", path.display(),
                                                    why.description()),
        Ok(file) => file,
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("Could not read {}: {}", path.display(),
                                                    why.description()),
        Ok(_) => println!("Read {}", path.display()),
    };

    let mut value: i32;
    let mut sum: i32;
    let mut total: i32 = 0;
    
    for string in input.lines() {
        sum = 0;
        match string.parse::<i32>(){
            Ok(_) => value = string.parse().unwrap(),
            Err(why) => {
                println!("Could not parse input: {}", why.description());
                break;
            },
        };
        /* Fuel required to launch a given module is based on its mass. 
        Specifically, to find the fuel required for a module, take its mass, 
        divide by three, round down, and subtract 2. */
        let mut req = fuel_calc(value);
        while req > 0 {
            sum += req;
            req = fuel_calc(req)
        }
        total += sum;
        println!(
            "Mass {}\trequires {} fuel.\tTotal fuel requirement is now {}",
            value, sum, total
        );
    }
}

fn fuel_calc(input: i32) -> i32 {
    let req: i32 = input / 3 - 2;
    if req > 0 {
        req
    } else {
        0
    }
}