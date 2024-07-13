use std::fs;
use std::process;

use clap::{Arg, Command};
use pulldown_cmark::{html::push_html, Parser};
use log::{error, info};
use env_logger;

fn main() {
    // Initialize the logger
    env_logger::init();

    let matches = Command::new("Markdown to HTML Converter")
        .version("1.0")
        .author("Nenad <nbursa@gmail.com>")
        .about("Convert Markdown files to HTML")
        .arg(Arg::new("INPUT")
            .help("Sets the input Markdown file to use")
            .required(true)
            .index(1))
        .arg(Arg::new("OUTPUT")
            .help("Sets the output HTML file")
            .required(true)
            .index(2))
        .get_matches();

    let input_file = matches.get_one::<String>("INPUT").unwrap();
    let output_file = matches.get_one::<String>("OUTPUT").unwrap();

    // Read the markdown file
    let markdown_input = fs::read_to_string(input_file).unwrap_or_else(|err| {
        error!("Error reading input file {}: {}", input_file, err);
        process::exit(1);
    });

    info!("Successfully read input file: {}", input_file);

    let parser = Parser::new(&markdown_input);

    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    fs::write(output_file, html_output).unwrap_or_else(|err| {
        error!("Error writing output file {}: {}", output_file, err);
        process::exit(1);
    });

    info!("Successfully wrote to output file: {}", output_file);

    println!("Successfully converted {} to {}", input_file, output_file);
}
