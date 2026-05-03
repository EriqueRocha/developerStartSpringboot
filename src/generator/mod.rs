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

pub mod pom;
pub mod java;
pub mod files;

use std::fs;
use std::io;
use std::path::PathBuf;

use crate::config::{ProjectConfig, PhysicalLayer, EntityConfig};
use crate::utils::domain_to_path;

pub struct CodeGenerator<'a> {
    pub config: &'a ProjectConfig,
    pub root: PathBuf,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(config: &'a ProjectConfig, root: PathBuf) -> Self {
        Self { config, root }
    }

    pub fn generate(&self) -> io::Result<()> {
        println!("\nGenerating project: {}\n", self.root.display());

        self.generate_root_pom()?;
        self.generate_gitignore()?;
        self.generate_gitattributes()?;
        self.generate_mvnw()?;

        for layer in &self.config.layers.physical {
            self.generate_layer(layer)?;
        }

        println!("\nProject generated successfully at: {}", self.root.display());
        Ok(())
    }

    pub fn generate_layer(&self, layer: &PhysicalLayer) -> io::Result<()> {
        let layer_root = self.root.join(&layer.name);

        self.generate_layer_pom(layer)?;

        let base_path = layer_root
            .join("src/main/java")
            .join(domain_to_path(&self.config.project.domain))
            .join(&layer.name);

        for logical in &layer.logical {
            let package_path = base_path.join(logical.replace("/", std::path::MAIN_SEPARATOR_STR));
            fs::create_dir_all(&package_path)?;

            self.generate_logical_content(layer, logical, &package_path)?;
        }

        let test_path = layer_root
            .join("src/test/java")
            .join(domain_to_path(&self.config.project.domain))
            .join(&layer.name);
        fs::create_dir_all(&test_path)?;

        let resources_path = layer_root.join("src/main/resources");
        fs::create_dir_all(&resources_path)?;
        self.generate_application_properties(layer, &resources_path)?;

        if layer.is_main {
            self.generate_application_class(layer)?;

            if self.config.features.spring_security {
                let has_config = layer.logical.iter().any(|l| l.to_lowercase() == "config");
                if !has_config {
                    let config_path = base_path.join("config");
                    fs::create_dir_all(&config_path)?;
                    self.generate_security_config(layer, &config_path)?;
                }
            }

            if self.config.features.example_endpoints {
                let has_controllers = layer.logical.iter().any(|l| l.to_lowercase().contains("controllers"));
                if !has_controllers {
                    let controllers_path = base_path.join("controllers");
                    fs::create_dir_all(&controllers_path)?;
                    self.generate_example_controller(layer, "controllers", &controllers_path)?;
                }
            }

            if self.config.features.spring_security {
                let has_authenticatable = self.config.entities.iter().any(|e| e.authenticatable);
                let has_controllers = layer.logical.iter().any(|l| l.to_lowercase().contains("controllers"));
                if has_authenticatable && !has_controllers {
                    let auth_path = base_path.join("auth");
                    fs::create_dir_all(&auth_path)?;
                    for entity in self.config.entities.iter().filter(|e| e.authenticatable) {
                        self.generate_dto(layer, "auth", &auth_path, entity)?;
                        self.generate_auth_dto(layer, "auth", &auth_path, entity)?;
                        self.generate_auth_controller(layer, "auth", &auth_path, entity)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn generate_logical_content(&self, layer: &PhysicalLayer, logical: &str, path: &std::path::Path) -> io::Result<()> {
        let logical_lower = logical.to_lowercase();

        let entities_here: Vec<&EntityConfig> = self.config.entities.iter().filter(|e| {
            let loc = e.location.as_ref().or(self.config.layers.entity_location.as_ref());
            match loc {
                Some(l) => l.layer == layer.name && l.logical == logical,
                None => logical_lower.contains("entities"),
            }
        }).collect();

        if !entities_here.is_empty() {
            for entity in &entities_here {
                self.generate_entity(layer, logical, path, entity)?;
            }
        }

        if logical_lower.contains("repositor") {
            for entity in &self.config.entities {
                if logical_lower.contains("adapters") || logical_lower.contains("adapter") {
                    self.generate_repository_adapter(layer, logical, path, entity)?;
                } else {
                    self.generate_repository_port(layer, logical, path, entity)?;
                }
            }
        }

        if logical_lower.contains("controllers") || logical_lower.contains("web") && logical_lower.contains("controllers") {
            for entity in &self.config.entities {
                self.generate_controller(layer, logical, path, entity)?;
                if entity.authenticatable && self.config.features.spring_security {
                    self.generate_auth_controller(layer, logical, path, entity)?;
                }
            }
            if self.config.features.example_endpoints {
                self.generate_example_controller(layer, logical, path)?;
            }
        }

        if logical_lower.contains("dto") {
            for entity in &self.config.entities {
                self.generate_dto(layer, logical, path, entity)?;
                if entity.authenticatable && self.config.features.spring_security {
                    self.generate_auth_dto(layer, logical, path, entity)?;
                }
            }
        }

        if logical_lower == "config" && layer.is_main {
            self.generate_config_files(layer, path)?;
        }

        if logical_lower.contains("usecases") || logical_lower.contains("usecase") {
            for entity in &self.config.entities {
                self.generate_usecase(layer, logical, path, entity)?;
            }
        }

        Ok(())
    }

    pub fn get_package(&self, layer: &PhysicalLayer, logical: &str) -> String {
        format!("{}.{}.{}",
            self.config.project.domain,
            layer.name,
            logical.replace("/", ".")
        )
    }

    pub fn find_entity_package(&self, entity: &EntityConfig) -> String {
        if let Some(loc) = entity.location.as_ref().or(self.config.layers.entity_location.as_ref()) {
            return format!("{}.{}.{}",
                self.config.project.domain,
                loc.layer,
                loc.logical.replace("/", ".")
            );
        }
        for layer in &self.config.layers.physical {
            for logical in &layer.logical {
                if logical.contains("entities") {
                    return format!("{}.{}.{}",
                        self.config.project.domain,
                        layer.name,
                        logical.replace("/", ".")
                    );
                }
            }
        }
        format!("{}.domain.entities", self.config.project.domain)
    }

    pub fn find_port_repositories_package(&self) -> String {
        for layer in &self.config.layers.physical {
            for logical in &layer.logical {
                if logical.contains("ports") && logical.to_lowercase().contains("repositor") {
                    return format!("{}.{}.{}",
                        self.config.project.domain,
                        layer.name,
                        logical.replace("/", ".")
                    );
                }
            }
        }
        for layer in &self.config.layers.physical {
            for logical in &layer.logical {
                if logical.to_lowercase().contains("repositor") {
                    return format!("{}.{}.{}",
                        self.config.project.domain,
                        layer.name,
                        logical.replace("/", ".")
                    );
                }
            }
        }
        format!("{}.application.ports.repositories", self.config.project.domain)
    }

    pub fn find_security_package(&self) -> String {
        self.config.layers.physical.iter()
            .find(|l| l.is_main)
            .map(|l| format!("{}.{}.config.security", self.config.project.domain, l.name))
            .unwrap_or_default()
    }
}
