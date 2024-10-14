use std::fs::File;
use std::collections::HashMap;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::io::BufRead;
use clap::{Arg, ArgAction, Command};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let wanted_string = &args[1];
    let rest_of_strings = &args[2..args.len()];
    let mut files : Vec<String>;
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("Your Name")
        .about("Demonstrates option parsing")
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue)
            .help("Print more information"))
        .arg(Arg::new("count")
            .short('c')
            .long("count")
            .value_name("NUMBER")
            .help("Number of times to do something")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file")
            .required(false))
        .arg(Arg::new("files")
            .help("Input files to process")
            .required(true)
            .num_args(1..)  // This allows multiple values
            .index(1))      // This makes it a positional argument
        .get_matches();
    println!("{:?}", matches);
    // for  in  {
        
    // }
    // println!("{:?}",vect);
    // parse_string(args);
    // let file_content = read_file("lamin");
    // find_string(file_content.unwrap(), input_line);
}

fn parse_string(input : &String) -> HashMap<String,Vec<String>>{
    let mut parsed_map = HashMap::new();
    let splited_input:Vec<&str> = input.split(" ").collect();
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
    let mut line_number:i32 = 1;
    for line in cursor.lines(){
        match line {
              Ok(content) => {
                // println!("{}   ----- {}", content, wanted_string);
                if content.contains(&wanted_string.trim()) {
                    println!("{} -> {}", line_number, content);
                }
                line_number += 1;
              },
              Err(_e) => eprintln!("ayoooooooooooooo")
        }
    }
}
