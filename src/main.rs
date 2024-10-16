use clap::ArgMatches;
use clap::{Arg, ArgAction, Command};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
fn main() {
    let matches = get_arguments();
    if let Some(files) = matches.get_many::<String>("files") {
        println!("Input files:");
        for file in files {
            println!("  {}", file);
        }
    }
    // println!("{:?}",vect);
    // parse_string(args);
    // let file_content = read_file("lamin");
    // find_string(file_content.unwrap(), input_line);
}

fn get_arguments() -> ArgMatches{
    let matches = Command::new("Grep App")
        .version("1.0")
        .author("Hooman Keshvari")
        .about("A grep command line utility")
        .arg(
            Arg::new("invert")
                .short('v')
                .action(clap::ArgAction::SetTrue)
                .help("Invert match (exclude lines that match the pattern)")
                .required(false),
        )
        .arg(
            Arg::new("Case-insensitive")
                .short('i')
                .action(clap::ArgAction::SetTrue)
                .help("Case-insensitive search")
                .required(false),
        )
        .arg(
            Arg::new("Line-Number")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("Print line numbers")
                .required(false),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .action(clap::ArgAction::SetTrue)
                .help("Recursive directory search")
                .required(false),
        )
        .arg(
            Arg::new("file_names")
                .short('f')
                .action(clap::ArgAction::SetTrue)
                .help("Print filenames")
                .required(false),
        )
        .arg(
            Arg::new("file_names")
                .short('c')
                .action(clap::ArgAction::SetTrue)
                .help("Enable colored output")
                .required(false),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(clap::ArgAction::SetTrue)
                .help("Show help information")
                .required(false),
        )
        .arg(
            Arg::new("patten")
                .value_name("pattern")
                .help("A required text argument")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("files")
                .help("Input files to process")
                .value_name("files")
                .required(true)
                .num_args(1..)
                .index(2),
        )
        .get_matches();
    return matches;
}


fn parse_string(input: &String) -> HashMap<String, Vec<String>> {
    let mut parsed_map = HashMap::new();
    let splited_input: Vec<&str> = input.split(" ").collect();
    println!("{:?}", splited_input);
    return parsed_map;
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_string(file_content: String, wanted_string: String) {
    let cursor = Cursor::new(file_content);
    let mut line_number: i32 = 1;
    for line in cursor.lines() {
        match line {
            Ok(content) => {
                // println!("{}   ----- {}", content, wanted_string);
                if content.contains(&wanted_string.trim()) {
                    println!("{} -> {}", line_number, content);
                }
                line_number += 1;
            }
            Err(_e) => eprintln!("ayoooooooooooooo"),
        }
    }
}
