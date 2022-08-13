use std::process::exit;
use clap::Parser;

use app::{user_friendly_read_from_file, user_friendly_write_to_file};
use crossterm::style::Stylize;
use justfuck::mapper::evaled_string;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// The input file.
    #[clap(value_parser)]
    input_file: String,

    /// The output file. 
    #[clap(long, value_parser, default_value="")]
    output_file: String,

}

mod justfuck;
mod app;

fn main() {
    let args = Args::parse();
    let app_name = &std::env::args().collect::<Vec<String>>()[0];
    let input_filename = args.input_file;
    let output_filename: String;
    if args.output_file == "" {
        output_filename = String::from(String::from(input_filename.trim_end_matches(".js")) + ".fuck.js")
    } else {
        output_filename = args.output_file;
    }
    let original_content = match user_friendly_read_from_file(&input_filename)  {
        Ok(content) => content,
        Err(error) => {
            eprintln!("{}", format!("{}:{}", app_name, error).bold().red());
            exit(-1);
        }
    };
    match user_friendly_write_to_file(&output_filename, &evaled_string(&original_content)) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("{}", format!("{}:{}", app_name, error).bold().red());
            exit(-1);
        }
    }
}
