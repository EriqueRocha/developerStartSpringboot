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
use serde::{Serialize, Deserialize};


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
#[derive(Serialize, Deserialize, Debug)]
struct LogicalLayer {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PhysicalLayer {
    name: String,
    logical_layers: Vec<LogicalLayer>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectSchema {
    project_name: String,
    physical_layers: Vec<PhysicalLayer>,
}

fn init_project() -> io::Result<()> {
    println!("{}", BANNER);

    let project_name = prompt_required("Project name (e.g.: MyAPI)");

    let num_physical = prompt_number("How many physical layers?");
    let mut physical_layers = Vec::new();

    for i in 0..num_physical {
        println!("\n→ Physical layer #{}:", i + 1);
        let physical_name = prompt_required("  Name of physical layer");

        let num_logical = prompt_number("  How many logical layers?");
        let mut logical_layers = Vec::new();

        for j in 0..num_logical {
            let logical_name = prompt_required(&format!("    Name of logical layer #{}", j + 1));
            logical_layers.push(LogicalLayer { name: logical_name });
        }

        physical_layers.push(PhysicalLayer {
            name: physical_name,
            logical_layers,
        });
    }

    let project_schema = ProjectSchema {
        project_name: project_name.clone(),
        physical_layers,
    };

    let json = serde_json::to_string_pretty(&project_schema).unwrap();

    let file_name = format!("{}_schema.json", project_name);
    fs::write(&file_name, &json)?;

    println!("\n✅ Generated JSON schema:\n{}", json);
    println!("\n💾 Saved to file: '{}'", file_name);

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
    let s = input.trim();
    if s.is_empty() {
        default.unwrap_or("DeveloperStartSpringboot").to_string()
    } else {
        to_pascal_case(s)
    }
}

fn prompt_number(label: &str) -> usize {
    loop {
        print!("{}: ", label);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<usize>() {
            return num;
        }
        println!("Please enter a valid number.");
    }
}

fn to_pascal_case(input: &str) -> String {
    input
        .split(|c: char| c == ' ' || c == '-' || c == '_')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let first = first.to_uppercase().collect::<String>();
                    let rest = chars.collect::<String>().to_lowercase();
                    format!("{}{}", first, rest)
                }
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
                eprintln!("Error generating JSON: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Version => {
            println!("dss version 0.1.0");
        }
    }
}