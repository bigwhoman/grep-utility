
use clap::ArgMatches;
use clap::{Arg, Command};
use colored::Colorize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

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
        if let Some(pattern) = &matches.get_one::<String>("pattern") {
            for file_pattern in file_patterns {
                match fs::metadata(file_pattern) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            /* Do nothing if -r is not set */
                            if matches.get_flag("recursive") {
                                let path = Path::new(file_pattern);
                                for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                                    if entry.file_type().is_file() {
                                        find_string(
                                            entry.path().display().to_string(),
                                            pattern.to_string(),
                                            &matches,
                                        );
                                    }
                                }
                            }
                        } else {
                            find_string(
                                file_pattern.to_string(),
                                pattern.to_string(),
                                &matches,
                            );
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

fn find_string(file: String, pattern: String, args: &ArgMatches) {
    let cursor = Cursor::new(read_file(&file).unwrap());
    let mut line_number: i32 = 1;
    for line in cursor.lines() {
        match line {
            Ok(mut content) => {
                // println!("{}   ----- {}", content, pattern);
                let content_array = content
                    .split(" ")
                    .map(|c| {
                        if args.get_flag("case-insensitive") {
                            c.to_lowercase()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<Vec<String>>();
                let lower = &pattern.trim().to_string();
                let search_pattern = if args.get_flag("case-insensitive") {
                    &lower.to_lowercase()
                } else {
                    &lower
                };
                if content_array.contains(search_pattern) {
                    if args.get_flag("invert") {
                        continue;
                    }
                    if args.get_flag("colored") {
                        content = content
                            .split(" ")
                            .map(|c| {
                                if c.to_lowercase() == pattern.to_lowercase() {
                                    c.red().to_string()
                                } else {
                                    c.white().to_string()
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(" ");
                    }
                } else {
                    if !args.get_flag("invert") {
                        continue;
                    }
                }
                if args.get_flag("line-Number") {
                    content = format!("{line_number}: ").to_string() + &content;
                }
                if args.get_flag("file_names") {
                    content = format!("{file}: {content}").to_string();
                }
                println!("{}", content);

                line_number += 1;
            }
            Err(_e) => eprintln!("ayoooooooooooooo"),
        }
    }
}
