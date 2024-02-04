
mod runners;
use runners::production::start_production;

use crate::runners::development::start_development;

use std::env;



fn main() -> () {

    // Get the additional arguments from "cargo run 
    // List of arguments 
    // --host=127.0.0.1
    // --port=8080
    // --env=prod / dev  
    // --astro-port=4321
    // --prod-astro-build=true / false  

    let args: Vec<String> = env::args().collect();

    let mut host = "127.0.0.1";
    let mut port = "8080";    
    let mut env = "dev";
    let mut astro_port = "4321";
    let mut prod_astro_build: bool = true;

    for arg in &args {
        if arg.starts_with("--env=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                env = split[1];
            }
        }
        if arg.starts_with("--host=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                host = split[1];
            }
        }
        if arg.starts_with("--port=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                port = split[1];
            }
        }

        if arg.starts_with("--astro-port=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                astro_port = split[1];
            }
        }

        if arg.starts_with("--prod-astro-build=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                prod_astro_build = split[1].parse::<bool>().unwrap();
            }
        }
    }
    

    // If the environment is development, then we want to start the frontend development astro server
    // Start the astro project in the frontend folder
    // Create a new thread to run the astro server

    if env == "dev" {
        start_development(host, port, astro_port);
    }

    if env == "prod" {
       start_production(host, port, prod_astro_build)
    }



    // If the environment is production, then we want to serve the frontend from the dist folder

}