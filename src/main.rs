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

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::create_dir_all;
use std::io::{self, Write};


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
#[command(about = "Spring Boot project generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
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

fn main() {
    match Cli::parse().command {
        Commands::Init => {
            if let Err(e) = init_project() {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Commands::Version => println!("dss version 0.3.0"),
    }
}

/* ---------- helpers de formatação ---------- */

fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == ' ' || c == '-' || c == '_')
        .filter(|seg| !seg.is_empty())
        .map(|seg| {
            let mut ch = seg.chars();
            match ch.next() {
                Some(f) => f.to_uppercase().collect::<String>() + &ch.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect()
}

fn to_camel_case(s: &str) -> String {
    let p = to_pascal_case(s);
    let mut ch = p.chars();
    match ch.next() {
        Some(f) => f.to_lowercase().collect::<String>() + ch.as_str(),
        None => String::new(),
    }
}

fn domain_to_path(domain: &str) -> String {
    domain
        .trim()
        .trim_start_matches("www.")
        .split('.')
        .filter(|seg| !seg.is_empty())
        .rev()
        .map(|seg| seg.to_lowercase())
        .collect::<Vec<_>>()
        .join("/")
}

/* ---------- prompts ---------- */

fn prompt_line(label: &str, default: Option<&str>) -> String {
    loop {
        print!(
            "{}{}: ",
            label,
            default.map(|d| format!(" [{d}]")).unwrap_or_default()
        );
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();

        if !input.is_empty() {
            return input.to_string();
        }
        if let Some(def) = default {
            return def.to_string();
        }
        println!("This field is required. Please fill it in.");
    }
}

fn prompt_number(label: &str) -> usize {
    loop {
        print!("{label}: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        if let Ok(n) = buf.trim().parse::<usize>() {
            return n;
        }
        println!("Please enter a valid number.");
    }
}

/* ---------- geração principal ---------- */

fn init_project() -> io::Result<()> {
    println!("{BANNER}");

    //coleta
    let project_name_raw = prompt_line("Project name (e.g.: MyAPI)", None);
    let domain_raw = prompt_line("Domain (e.g.: example.com)", None);

    let project_name = to_pascal_case(&project_name_raw);
    let domain_path = domain_to_path(&domain_raw);

    let num_physical = prompt_number("How many physical layers?");
    let mut physical_layers = Vec::with_capacity(num_physical);

    for i in 0..num_physical {
        println!("\n→ Physical layer #{}:", i + 1);
        let phys_name = prompt_line("  Name of physical layer", None);
        let num_logical = prompt_number("  How many logical layers?");
        let mut logical_layers = Vec::with_capacity(num_logical);

        for j in 0..num_logical {
            let log_name = prompt_line(&format!("    Name of logical layer #{}", j + 1), None);
            logical_layers.push(LogicalLayer { name: log_name });
        }
        physical_layers.push(PhysicalLayer {
            name: phys_name,
            logical_layers,
        });
    }

    //JSON
    let schema = ProjectSchema {
        project_name: project_name.clone(),
        physical_layers,
    };
    let json = serde_json::to_string_pretty(&schema).unwrap();
    let json_file = format!("{}_schema.json", project_name);
    fs::write(&json_file, &json)?;
    println!("\n-- Generated JSON schema:\n{json}");
    println!("-- Saved to file: '{json_file}'");

    //pastas
    create_dirs(&schema, &domain_path)?;

    Ok(())
}

/* ---------- criação de diretórios ---------- */

fn create_dirs(schema: &ProjectSchema, domain_path: &str) -> io::Result<()> {
    let root = &schema.project_name;
    create_dir_all(root)?;
    fs::write(format!("{root}/pom.xml"), "")?;//pom raiz

    for phys in &schema.physical_layers {
        let phys_camel = to_camel_case(&phys.name);
        let phys_dir = format!("{root}/{phys_camel}");
        create_dir_all(&phys_dir)?;

        create_dir_all(format!("{phys_dir}/src/test"))?;
        create_dir_all(format!("{phys_dir}/src/main/resource"))?;

        let base_code = format!("{phys_dir}/src/main/{domain_path}/{phys_camel}");
        create_dir_all(&base_code)?;

        //camadas lógicas
        for log in &phys.logical_layers {
            let log_camel = to_camel_case(&log.name);
            create_dir_all(format!("{base_code}/{log_camel}"))?;
        }

        fs::write(format!("{phys_dir}/pom.xml"), "")?;
    }

    println!("\n-- Directories created successfully!");
    Ok(())
}
