use std::process::exit;
use clap::Parser;

use app::{user_friendly_read_from_file, user_friendly_write_to_file};

use crate::justfuck::mapper::encode_string;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The input file.
    input_file: String,
    /// The output file. 
    output_file: String,
}

mod justfuck;
mod app;

fn main() {
    let content = match user_friendly_read_from_file("examples/test.js") {
        Ok(content) => content,
        Err(why) => {
            println!("{}", why);
            exit(-1);
        },
    };
    match user_friendly_write_to_file("examples/test.fuck.js", &encode_string(&content)) {
        Ok(()) => {}
        Err(why) => {
            println!("{}", why);
            exit(-1);
        }
    }
}
