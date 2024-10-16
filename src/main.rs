use clap::ArgMatches;
use clap::{Arg, Command};
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;

const HELP_STRING: &str = r#"
Usage: grep [OPTIONS] <pattern> <files...>
Options:
-i                Case-insensitive search
-n                Print line numbers
-v                Invert match (exclude lines that match the pattern)
-r                Recursive directory search
-f                Print filenames
-c                Enable colored output
-h, --help        Show help information
"#;

fn main() {
    let matches = get_arguments();

    if let Some(file_patterns) = matches.get_many::<String>("files") {
        if let Some(pattern) = matches.get_one::<String>("pattern") {
            for file_pattern in file_patterns {
                match fs::metadata(file_pattern) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            /* Do nothing if -r is not set */
                        } else {
                            find_string(read_file(file_pattern).unwrap(), pattern.to_string());
                        }
                    }
                    Err(e) => println!("Error accessing {}: {}", file_pattern, e),
                }
            }
        }
    } else {
        println!("Error, No files were given")
    }
}

fn get_arguments() -> ArgMatches {
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
            Arg::new("case-insensitive")
                .short('i')
                .action(clap::ArgAction::SetTrue)
                .help("Case-insensitive search")
                .required(false),
        )
        .arg(
            Arg::new("line-Number")
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
            Arg::new("colored")
                .short('c')
                .action(clap::ArgAction::SetTrue)
                .help("Enable colored output")
                .required(false),
        )
        .arg(
            Arg::new("pattern")
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
        .help_template(HELP_STRING)
        .get_matches();
    return matches;
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
