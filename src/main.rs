/*
 * Copyright (C) 2025 Erique Rocha
 *
 * This file is part of developerStartSpringboot.
 *
 * developerStartSpringboot is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License.
 *
 * See the LICENSE file for more details.
 */

use std::io::{self, Write};
use std::fs;
use clap::{Parser, Subcommand};
use std::path::Path;


const BANNER: &str = concat!("\x1b[32m", r#"
        developerStartSpringboot
    ,---,       .--.--.      .--.--.
  .'  .' `\    /  /    '.   /  /    '.
,---.'     \  |  :  /`. /  |  :  /`. /
|   |  .`\  | ;  |  |--`   ;  |  |--`
:   : |  '  | |  :  ;_     |  :  ;_
|   ' '  ;  :  \  \    `.   \  \    `.
'   | ;  .  |   `----.   \   `----.   \
|   | :  |  '   __ \  \  |   __ \  \  |
'   : | /  ;   /  /`--'  /  /  /`--'  /
|   | '` ,/   '--'.     /  '--'.     /
;   :  .'       `--'---'     `--'---'
|   ,.'
'---'
 https://github.com/EriqueRocha/developerStartSpringboot
 START YOUR JAVA PROJECT WITH SPRINGBOOT
"#,
"\x1b[0m"
);

#[derive(Parser)]
#[command(name = "dss")]
#[command(about = "Spring Boot project generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Spring Boot project
    Init,
    /// Show version information
    Version,
}

fn init_project() -> io::Result<()> {
    println!("{}", BANNER);
    let project_name = prompt_required("Project name (e.g.: myAPI)");
    let project_name = to_pascal_case(&project_name);
    create_path(&project_name)?;
    println!("crated folder {}", project_name);
    Ok(())
}

fn create_path(name: &str) -> io::Result<()> {
    let path = Path::new(name);
    if path.exists() {
        println!("'{}' already exists", name);
    } else {
        fs::create_dir(path)?;
    }
    Ok(())
}

fn prompt_required(label: &str) -> String {
    loop {
        let input = prompt(label, None);
        if !input.trim().is_empty() {
            return input;
        }
        println!("This field is required. Please fill it in.");
    }
}

fn prompt(label: &str, default: Option<&str>) -> String {
    let mut input = String::new();
    print!("{}{}: ", label, default.map(|d| format!(" [{}]", d)).unwrap_or_default());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();
    if s.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        s
    }
}

fn to_pascal_case(input: &str) -> String {
    input
        .split(|c: char| c == ' ' || c == '-' || c == '_')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            if let Err(e) = init_project() {
                eprintln!("Error initializing project: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Version => {
            println!("dss version 0.1.0");
        }
    }
}