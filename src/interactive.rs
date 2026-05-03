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

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::constants::*;
use crate::config::{
    ProjectConfig, ProjectInfo, DeveloperInfo, LayersConfig, PhysicalLayer,
    EntityConfig, EntityLocation, FeaturesConfig,
};
use crate::ui::{
    prompt, prompt_required, prompt_yes_no, prompt_number,
    print_section, print_subsection, print_info, select_interactive,
};
use crate::generator::CodeGenerator;

pub fn build_config_interactive() -> ProjectConfig {
    println!("{}", BANNER);

    print_section("Project Configuration");

    let app_name = prompt_required("Application name (e.g.: myAPI)").replace(' ', "-");
    let domain = prompt_required("Domain (e.g.: com.example.demo)");
    let description = prompt("Description", Some("Spring Boot API"));

    print_section("Developer Information");
    let dev_name = prompt("Developer name", Some("Developer"));
    let dev_email = prompt("Developer email", Some("dev@example.com"));
    let dev_url = prompt("Developer website", Some("example.com"));

    print_section("Physical Layers (Maven Modules)");
    print_info("Define the physical layers of your project.");
    println!("  {CYAN}Common architectures:{RESET}");
    println!("    {GREEN}•{RESET} Monolithic: 1 layer (all in one)");
    println!("    {GREEN}•{RESET} Clean Architecture: 3 layers (core, application, infrastructure)");
    println!("    {GREEN}•{RESET} Hexagonal: 3+ layers (domain, ports, adapters)");
    println!();

    let num_layers = prompt_number("Number of physical layers", 3);

    let mut physical_layers: Vec<PhysicalLayer> = Vec::new();

    for i in 0..num_layers {
        print_subsection(&format!("Layer {}", i + 1));

        let default_name = match i {
            0 => "core",
            1 => "application",
            2 => "infrastructure",
            _ => "",
        };

        let layer_name = prompt(&format!("Layer {} name", i + 1), Some(default_name));

        print_info(&format!("Define logical packages for '{}' (comma-separated)", layer_name));
        println!("  {CYAN}Example:{RESET} domain/entities,domain/valueobjects,domain/services");

        let default_logical = match layer_name.as_str() {
            "core" => "domain/entities,domain/valueobjects",
            "application" => "usecases,ports/repositories,ports/services",
            "infrastructure" => "adapters/web/controllers,adapters/web/dto,adapters/repositories,adapters/services,config",
            _ => "",
        };

        let logical_input = prompt("Logical packages", Some(default_logical));
        let logical: Vec<String> = logical_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut dependencies: Vec<String> = Vec::new();
        if i > 0 && !physical_layers.is_empty() {
            let available: Vec<&str> = physical_layers.iter().map(|l| l.name.as_str()).collect();
            print_info(&format!("Available layers for dependency: {}", available.join(", ")));
            let deps_input = prompt("Dependencies (comma-separated, or empty)", None);
            dependencies = deps_input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        let is_main = if i == num_layers - 1 {
            prompt_yes_no("Is this the main module (contains Application class)?", true)
        } else {
            prompt_yes_no("Is this the main module (contains Application class)?", false)
        };

        physical_layers.push(PhysicalLayer {
            name: layer_name,
            logical,
            dependencies,
            is_main,
        });
    }

    print_section("Entities");

    let (mut entities, entity_location) = if prompt_yes_no("Create entities?", true) {
        let mut options: Vec<String> = Vec::new();
        let mut option_map: Vec<(String, String)> = Vec::new();
        for layer in &physical_layers {
            for logical in &layer.logical {
                options.push(format!("{} {} {}", layer.name, CYAN.to_string() + "›" + RESET, logical));
                option_map.push((layer.name.clone(), logical.clone()));
            }
        }

        let mut entities: Vec<EntityConfig> = Vec::new();
        let mut first_location: Option<EntityLocation> = None;
        let mut entity_counter = 0usize;

        loop {
            let idx = select_interactive("Where should entities be placed?", &options);
            let (chosen_layer, chosen_logical) = option_map[idx].clone();
            println!("  {GREEN}✓ Entities will be placed in: {chosen_layer} › {chosen_logical}{RESET}");

            let location = EntityLocation { layer: chosen_layer, logical: chosen_logical };

            if first_location.is_none() {
                first_location = Some(EntityLocation { layer: location.layer.clone(), logical: location.logical.clone() });
            }

            let num_entities = prompt_number("Number of entities to create", 1);
            for _ in 0..num_entities {
                entity_counter += 1;
                print_subsection(&format!("Entity {}", entity_counter));
                let entity_name = prompt(&format!("Entity {} name", entity_counter), Some("User"));
                let role = prompt("Role for this entity", Some("USER"));
                entities.push(EntityConfig {
                    name: entity_name,
                    role,
                    authenticatable: false,
                    location: Some(EntityLocation { layer: location.layer.clone(), logical: location.logical.clone() }),
                });
            }

            if !prompt_yes_no("Add entities in another location?", false) {
                break;
            }
        }

        (entities, first_location)
    } else {
        (Vec::new(), None)
    };

    print_section("Features");
    let spring_security = prompt_yes_no("Include Spring Security (JWT)?", false);

    if spring_security && !entities.is_empty() {
        print_subsection("Authentication Entities");
        for entity in &mut entities {
            entity.authenticatable = prompt_yes_no(
                &format!("Should {} have authentication endpoints (login/register)?", entity.name),
                false,
            );
        }
    }

    let example_endpoints = prompt_yes_no("Include example endpoints for testing?", true);
    let swagger = prompt_yes_no("Include Swagger/OpenAPI documentation?", true);
    let flyway = prompt_yes_no("Include Flyway migrations?", false);

    ProjectConfig {
        project: ProjectInfo {
            name: app_name,
            domain,
            description,
            developer: DeveloperInfo {
                name: dev_name,
                email: dev_email,
                url: dev_url,
            },
        },
        layers: LayersConfig {
            physical: physical_layers,
            entity_location,
        },
        entities,
        features: FeaturesConfig {
            spring_security,
            example_endpoints,
            swagger,
            flyway,
        },
    }
}

pub fn init_interactive() -> io::Result<()> {
    let config = build_config_interactive();

    println!("\n=== Configuration Summary ===\n");
    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    if !prompt_yes_no("\nGenerate project with this configuration?", true) {
        println!("Cancelled.");
        return Ok(());
    }

    if prompt_yes_no("Save configuration to JSON file?", false) {
        let mut json_path = prompt("JSON file path", Some(&format!("{}.json", config.project.name)));
        if !json_path.ends_with(".json") {
            json_path.push_str(".json");
        }
        let json_content = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&json_path, json_content)?;
        println!("  {GREEN}✓ Configuration saved to: {}{RESET}", json_path);
    }

    let root = PathBuf::from(&config.project.name);
    if root.exists() {
        eprintln!("Error: Directory '{}' already exists.", root.display());
        std::process::exit(1);
    }

    let generator = CodeGenerator::new(&config, root);
    generator.generate()
}

pub fn generate_from_json(config_path: &Path) -> io::Result<()> {
    println!("{}", BANNER);

    let content = fs::read_to_string(config_path)?;
    let config: ProjectConfig = serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    println!("Loaded configuration from: {}", config_path.display());
    println!("Project name: {}", config.project.name);
    println!("Domain: {}", config.project.domain);
    println!("Physical layers: {:?}", config.layers.physical.iter().map(|l| &l.name).collect::<Vec<_>>());

    let root = PathBuf::from(&config.project.name);
    if root.exists() {
        eprintln!("Error: Directory '{}' already exists.", root.display());
        std::process::exit(1);
    }

    let generator = CodeGenerator::new(&config, root);
    generator.generate()
}

pub fn export_template(output: Option<&Path>) -> io::Result<()> {
    let template = create_default_template();
    let json = serde_json::to_string_pretty(&template).unwrap();

    match output {
        Some(path) => {
            fs::write(path, &json)?;
            println!("Template exported to: {}", path.display());
        }
        None => {
            println!("{}", json);
        }
    }

    Ok(())
}

pub fn create_default_template() -> ProjectConfig {
    ProjectConfig {
        project: ProjectInfo {
            name: "myAPI".to_string(),
            domain: "com.example.demo".to_string(),
            description: "Spring Boot API".to_string(),
            developer: DeveloperInfo {
                name: "Developer".to_string(),
                email: "dev@example.com".to_string(),
                url: "example.com".to_string(),
            },
        },
        layers: LayersConfig {
            physical: vec![
                PhysicalLayer {
                    name: "core".to_string(),
                    logical: vec![
                        "domain/entities".to_string(),
                        "domain/valueobjects".to_string(),
                    ],
                    dependencies: vec![],
                    is_main: false,
                },
                PhysicalLayer {
                    name: "application".to_string(),
                    logical: vec![
                        "usecases".to_string(),
                        "ports/repositories".to_string(),
                        "ports/services".to_string(),
                    ],
                    dependencies: vec!["core".to_string()],
                    is_main: false,
                },
                PhysicalLayer {
                    name: "infrastructure".to_string(),
                    logical: vec![
                        "adapters/web/controllers".to_string(),
                        "adapters/web/dto".to_string(),
                        "adapters/repositories".to_string(),
                        "adapters/services".to_string(),
                        "config".to_string(),
                    ],
                    dependencies: vec!["core".to_string(), "application".to_string()],
                    is_main: true,
                },
            ],
            entity_location: Some(EntityLocation {
                layer: "core".to_string(),
                logical: "domain/entities".to_string(),
            }),
        },
        entities: vec![
            EntityConfig {
                name: "User".to_string(),
                role: "USER".to_string(),
                authenticatable: false,
                location: None,
            },
        ],
        features: FeaturesConfig {
            spring_security: false,
            example_endpoints: true,
            swagger: true,
            flyway: false,
        },
    }
}
