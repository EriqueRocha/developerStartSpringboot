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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
    pub layers: LayersConfig,
    pub entities: Vec<EntityConfig>,
    pub features: FeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub domain: String,
    pub description: String,
    pub developer: DeveloperInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperInfo {
    pub name: String,
    pub email: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayersConfig {
    pub physical: Vec<PhysicalLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_location: Option<EntityLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityLocation {
    pub layer: String,
    pub logical: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalLayer {
    pub name: String,
    pub logical: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub is_main: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityConfig {
    pub name: String,
    #[serde(default = "default_role")]
    pub role: String,
    #[serde(default)]
    pub authenticatable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EntityLocation>,
}

fn default_role() -> String {
    "USER".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    #[serde(default)]
    pub spring_security: bool,
    #[serde(default)]
    pub example_endpoints: bool,
    #[serde(default = "default_true")]
    pub swagger: bool,
    #[serde(default)]
    pub flyway: bool,
}

fn default_true() -> bool {
    true
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        FeaturesConfig {
            spring_security: false,
            example_endpoints: false,
            swagger: true,
            flyway: false,
        }
    }
}
