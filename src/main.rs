
mod runners;
use crate::runners::development::start_development;

use std::env;

fn main() -> () {

    // Get the additional arguments from "cargo run -- prod / dev"

    let args: Vec<String> = env::args().collect();    

    let mut env = "dev";

    if args.len() > 1 {
        env = &args[1];
    }

    // If the environment is development, then we want to start the frontend development astro server
    // Start the astro project in the frontend folder
    // Create a new thread to run the astro server

    if env == "dev" {
        start_development();
    }



    // If the environment is production, then we want to serve the frontend from the dist folder

}