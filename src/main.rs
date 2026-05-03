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

mod constants;
mod config;
mod cli;
mod ui;
mod utils;
mod interactive;
mod generator;
mod base;

use clap::Parser;
use cli::{Cli, Commands};
use interactive::{init_interactive, generate_from_json, export_template};
use base::run_base;

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init => init_interactive(),
        Commands::Base => run_base(),
        Commands::Generate { config } => generate_from_json(config),
        Commands::Template { output } => export_template(output.as_deref()),
        Commands::Version => {
            println!("dss version 0.2.0");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
