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
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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

const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

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

#[derive(Parser)]
#[command(name = "dss")]
#[command(about = "Spring Boot project generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Generate {
        #[arg(short, long)]
        config: PathBuf,
    },
    Template {
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Version,
}

fn prompt(label: &str, default: Option<&str>) -> String {
    let mut input = String::new();
    let default_display = default.map(|d| format!(" {}[{}]{}", YELLOW, d, RESET)).unwrap_or_default();
    print!("  {CYAN}{BOLD}>{RESET} {}{}: ", label, default_display);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();
    if s.is_empty() { default.unwrap_or("").to_string() } else { s }
}

fn prompt_required(label: &str) -> String {
    loop {
        let input = prompt(label, None);
        if !input.trim().is_empty() {
            return input;
        }
        println!("  {YELLOW}! This field is required. Please fill it in.{RESET}");
    }
}

fn prompt_yes_no(label: &str, default: bool) -> bool {
    let default_str = if default { "Y/n" } else { "y/N" };
    let input = prompt(label, Some(default_str)).to_lowercase();
    match input.as_str() {
        "y" | "yes" | "s" | "sim" => true,
        "n" | "no" | "nao" | "não" => false,
        _ => default,
    }
}

fn prompt_number(label: &str, default: usize) -> usize {
    let input = prompt(label, Some(&default.to_string()));
    input.parse().unwrap_or(default)
}

enum KeyEvent { Up, Down, Enter, Other }

fn raw_mode(enable: bool) {
    let args: &[&str] = if enable {
        &["-icanon", "-echo"]
    } else {
        &["icanon", "echo"]
    };
    let _ = std::process::Command::new("stty").args(args).status();
}

fn read_key() -> KeyEvent {
    let mut buf = [0u8; 1];
    if io::stdin().read(&mut buf).unwrap_or(0) == 0 {
        return KeyEvent::Other;
    }
    match buf[0] {
        0x1b => {
            let mut seq = [0u8; 2];
            if io::stdin().read(&mut seq).unwrap_or(0) == 2 {
                match seq { [b'[', b'A'] => KeyEvent::Up, [b'[', b'B'] => KeyEvent::Down, _ => KeyEvent::Other }
            } else { KeyEvent::Other }
        }
        b'\r' | b'\n' => KeyEvent::Enter,
        b'k' => KeyEvent::Up,
        b'j' => KeyEvent::Down,
        _ => KeyEvent::Other,
    }
}

fn redraw_selector(items: &[String], selected: usize) {
    print!("\x1b[{}A", items.len());
    for (i, item) in items.iter().enumerate() {
        print!("\x1b[2K\r");
        if i == selected {
            println!("  {GREEN}{BOLD}▶ {item}{RESET}");
        } else {
            println!("    {item}");
        }
    }
    io::stdout().flush().unwrap();
}

fn select_interactive(title: &str, items: &[String]) -> usize {
    println!("\n  {CYAN}{BOLD}{title}{RESET}");
    println!("  {YELLOW}(↑/↓ or k/j to navigate, Enter to confirm){RESET}\n");

    for (i, item) in items.iter().enumerate() {
        if i == 0 {
            println!("  {GREEN}{BOLD}▶ {item}{RESET}");
        } else {
            println!("    {item}");
        }
    }
    io::stdout().flush().unwrap();

    raw_mode(true);
    let mut selected = 0usize;

    loop {
        match read_key() {
            KeyEvent::Up => {
                if selected > 0 {
                    selected -= 1;
                    redraw_selector(items, selected);
                }
            }
            KeyEvent::Down => {
                if selected < items.len() - 1 {
                    selected += 1;
                    redraw_selector(items, selected);
                }
            }
            KeyEvent::Enter => break,
            KeyEvent::Other => {}
        }
    }

    raw_mode(false);
    println!();
    selected
}

fn print_section(title: &str) {
    println!("\n{MAGENTA}{BOLD}══════════════════════════════════════════════════════════════{RESET}");
    println!("{MAGENTA}{BOLD}  {}{RESET}", title);
    println!("{MAGENTA}{BOLD}══════════════════════════════════════════════════════════════{RESET}\n");
}

fn print_subsection(title: &str) {
    println!("\n  {GREEN}{BOLD}── {} ──{RESET}\n", title);
}

fn print_info(message: &str) {
    println!("  {CYAN}ℹ {}{RESET}", message);
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_is_lower_or_digit = false;

    for ch in s.chars() {
        if ch.is_alphanumeric() {
            if ch.is_uppercase() {
                if prev_is_lower_or_digit && !out.is_empty() {
                    out.push('_');
                }
                for lc in ch.to_lowercase() {
                    out.push(lc);
                }
                prev_is_lower_or_digit = false;
            } else {
                out.push(ch);
                prev_is_lower_or_digit = true;
            }
        } else {
            if !out.ends_with('_') && !out.is_empty() {
                out.push('_');
            }
            prev_is_lower_or_digit = false;
        }
    }

    out.trim_matches('_')
        .split('_')
        .filter(|seg| !seg.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn to_pascal_case(s: &str) -> String {
    let parts = s.split(|c: char| !c.is_alphanumeric())
        .filter(|p| !p.is_empty());
    let mut out = String::new();
    for p in parts {
        let mut chs = p.chars();
        if let Some(first) = chs.next() {
            out.push_str(&first.to_uppercase().to_string());
            out.push_str(&chs.as_str().to_lowercase());
        }
    }
    if out.is_empty() { s.to_string() } else { out }
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_lowercase(), chars.as_str()),
        None => String::new(),
    }
}

fn to_app_name_clean(name: &str) -> String {
    let tokens: Vec<String> = name
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| {
            let mut chs = t.chars();
            match chs.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chs.as_str().to_lowercase()),
                None => String::new(),
            }
        })
        .collect();
    let mut out = tokens.join("");
    if out.is_empty() { out = "App".to_string(); }
    out
}

fn domain_to_path(domain: &str) -> String {
    domain.split('.').collect::<Vec<_>>().join("/")
}

fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    println!("  Created: {}", path.display());
    Ok(())
}

fn build_config_interactive() -> ProjectConfig {
    println!("{}", BANNER);

    print_section("Project Configuration");

    let app_name = prompt_required("Application name (e.g.: myAPI)");
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

        let idx = select_interactive("Where should entities be placed?", &options);
        let (chosen_layer, chosen_logical) = option_map[idx].clone();
        println!("  {GREEN}✓ Entities will be placed in: {chosen_layer} › {chosen_logical}{RESET}");

        let num_entities = prompt_number("Number of entities to create", 1);
        let mut entities: Vec<EntityConfig> = Vec::new();
        for i in 0..num_entities {
            print_subsection(&format!("Entity {}", i + 1));
            let entity_name = prompt(&format!("Entity {} name", i + 1), Some("User"));
            let role = prompt("Role for this entity", Some("USER"));
            entities.push(EntityConfig { name: entity_name, role, authenticatable: false });
        }

        let location = EntityLocation { layer: chosen_layer, logical: chosen_logical };
        (entities, Some(location))
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

struct CodeGenerator<'a> {
    config: &'a ProjectConfig,
    root: PathBuf,
}

impl<'a> CodeGenerator<'a> {
    fn new(config: &'a ProjectConfig, root: PathBuf) -> Self {
        Self { config, root }
    }

    fn generate(&self) -> io::Result<()> {
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

    fn generate_root_pom(&self) -> io::Result<()> {
        let modules: Vec<String> = self.config.layers.physical
            .iter()
            .map(|l| format!("        <module>{}</module>", l.name))
            .collect();

        let content = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0
         http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>{domain}</groupId>
    <artifactId>{name}</artifactId>
    <version>1.0.0</version>
    <packaging>pom</packaging>

    <name>{name}</name>
    <description>{description}</description>

    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>3.5.6</version>
        <relativePath/>
    </parent>

    <properties>
        <java.version>21</java.version>
        <maven.compiler.source>21</maven.compiler.source>
        <maven.compiler.target>21</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <modules>
{modules}
    </modules>

    <developers>
        <developer>
            <name>{dev_name}</name>
            <email>{dev_email}</email>
            <url>{dev_url}</url>
        </developer>
    </developers>

    <dependencyManagement>
        <dependencies>
        </dependencies>
    </dependencyManagement>
</project>
"#,
            domain = self.config.project.domain,
            name = self.config.project.name,
            description = self.config.project.description,
            modules = modules.join("\n"),
            dev_name = self.config.project.developer.name,
            dev_email = self.config.project.developer.email,
            dev_url = self.config.project.developer.url,
        );

        write_file(&self.root.join("pom.xml"), &content)
    }

    fn generate_layer(&self, layer: &PhysicalLayer) -> io::Result<()> {
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

    fn generate_layer_pom(&self, layer: &PhysicalLayer) -> io::Result<()> {
        let layer_root = self.root.join(&layer.name);

        let mut deps = String::new();

        for dep in &layer.dependencies {
            deps.push_str(&format!(r#"
        <dependency>
            <groupId>{}</groupId>
            <artifactId>{}</artifactId>
            <version>1.0.0</version>
        </dependency>"#,
                self.config.project.domain, dep
            ));
        }

        if let Some(loc) = &self.config.layers.entity_location {
            let entity_layer = &loc.layer;
            let is_same_layer = entity_layer == &layer.name;
            let already_declared = layer.dependencies.iter().any(|d| d == entity_layer);

            if !is_same_layer && !already_declared {
                let needs_entity_dep = layer.logical.iter().any(|logical| {
                    let lower = logical.to_lowercase();
                    lower.contains("repositor")
                        || lower.contains("controller")
                        || lower.contains("usecase")
                        || lower.contains("dto")
                });

                if needs_entity_dep {
                    deps.push_str(&format!(r#"
        <dependency>
            <groupId>{}</groupId>
            <artifactId>{}</artifactId>
            <version>1.0.0</version>
        </dependency>"#,
                        self.config.project.domain, entity_layer
                    ));
                }
            }
        }

        if layer.is_main {
            deps.push_str(r#"
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-data-jpa</artifactId>
        </dependency>
        <dependency>
            <groupId>org.postgresql</groupId>
            <artifactId>postgresql</artifactId>
            <scope>runtime</scope>
        </dependency>
        <dependency>
            <groupId>com.h2database</groupId>
            <artifactId>h2</artifactId>
            <scope>runtime</scope>
        </dependency>"#);

            if self.config.features.spring_security {
                deps.push_str(r#"
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-security</artifactId>
        </dependency>
        <dependency>
            <groupId>io.jsonwebtoken</groupId>
            <artifactId>jjwt-api</artifactId>
            <version>0.11.5</version>
        </dependency>
        <dependency>
            <groupId>io.jsonwebtoken</groupId>
            <artifactId>jjwt-impl</artifactId>
            <version>0.11.5</version>
            <scope>runtime</scope>
        </dependency>
        <dependency>
            <groupId>io.jsonwebtoken</groupId>
            <artifactId>jjwt-jackson</artifactId>
            <version>0.11.5</version>
            <scope>runtime</scope>
        </dependency>"#);
            }

            if self.config.features.swagger {
                deps.push_str(r#"
        <dependency>
            <groupId>org.springdoc</groupId>
            <artifactId>springdoc-openapi-starter-webmvc-ui</artifactId>
            <version>2.8.3</version>
        </dependency>"#);
            }

            if self.config.features.flyway {
                deps.push_str(r#"
        <dependency>
            <groupId>org.flywaydb</groupId>
            <artifactId>flyway-core</artifactId>
        </dependency>
        <dependency>
            <groupId>org.flywaydb</groupId>
            <artifactId>flyway-database-postgresql</artifactId>
            <scope>runtime</scope>
        </dependency>"#);
            }
        }

        let build_section = if layer.is_main {
            r#"
    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
        </plugins>
    </build>"#
        } else {
            ""
        };

        let content = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0
         http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <parent>
        <groupId>{domain}</groupId>
        <artifactId>{parent_name}</artifactId>
        <version>1.0.0</version>
    </parent>

    <artifactId>{layer_name}</artifactId>

    <dependencies>{deps}
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-test</artifactId>
            <scope>test</scope>
        </dependency>
    </dependencies>
{build_section}
</project>
"#,
            domain = self.config.project.domain,
            parent_name = self.config.project.name,
            layer_name = layer.name,
            deps = deps,
            build_section = build_section,
        );

        write_file(&layer_root.join("pom.xml"), &content)
    }

    fn generate_logical_content(&self, layer: &PhysicalLayer, logical: &str, path: &Path) -> io::Result<()> {
        let logical_lower = logical.to_lowercase();

        let is_entity_location = match &self.config.layers.entity_location {
            Some(loc) => loc.layer == layer.name && loc.logical == logical,
            None => logical_lower.contains("entities"),
        };
        if is_entity_location {
            for entity in &self.config.entities {
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

    fn get_package(&self, layer: &PhysicalLayer, logical: &str) -> String {
        format!("{}.{}.{}",
            self.config.project.domain,
            layer.name,
            logical.replace("/", ".")
        )
    }

    fn find_entity_package(&self) -> String {
        if let Some(loc) = &self.config.layers.entity_location {
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

    fn find_port_repositories_package(&self) -> String {
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

    fn generate_entity(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let package = self.get_package(layer, logical);

        let content = format!(r#"package {package};

import java.time.LocalDateTime;
import java.util.Objects;

public class {pascal} {{
    private Long id;
    private String email;
    private String password;
    private String name;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    public {pascal}() {{}}

    public {pascal}(String email, String password, String name) {{
        this.email = email;
        this.password = password;
        this.name = name;
        this.createdAt = LocalDateTime.now();
        this.updatedAt = LocalDateTime.now();
    }}

    public {pascal}(Long id, String email, String password, String name, LocalDateTime createdAt, LocalDateTime updatedAt) {{
        this.id = id;
        this.email = email;
        this.password = password;
        this.name = name;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }}

    public String getRole() {{
        return "{role}";
    }}

    public Long getId() {{ return id; }}
    public void setId(Long id) {{ this.id = id; }}

    public String getEmail() {{ return email; }}
    public void setEmail(String email) {{ this.email = email; }}

    public String getPassword() {{ return password; }}
    public void setPassword(String password) {{ this.password = password; }}

    public String getName() {{ return name; }}
    public void setName(String name) {{ this.name = name; }}

    public LocalDateTime getCreatedAt() {{ return createdAt; }}
    public void setCreatedAt(LocalDateTime createdAt) {{ this.createdAt = createdAt; }}

    public LocalDateTime getUpdatedAt() {{ return updatedAt; }}
    public void setUpdatedAt(LocalDateTime updatedAt) {{ this.updatedAt = updatedAt; }}

    @Override
    public boolean equals(Object o) {{
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        {pascal} that = ({pascal}) o;
        return Objects.equals(id, that.id) && Objects.equals(email, that.email);
    }}

    @Override
    public int hashCode() {{
        return Objects.hash(id, email);
    }}
}}
"#,
            package = package,
            pascal = pascal,
            role = entity.role,
        );

        write_file(&path.join(format!("{}.java", pascal)), &content)
    }

    fn generate_repository_port(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let package = self.get_package(layer, logical);
        let entity_package = self.find_entity_package();

        let content = format!(r#"package {package};

import {entity_package}.{pascal};
import java.util.Optional;

public interface {pascal}Repository {{
    {pascal} save({pascal} entity);
    Optional<{pascal}> findById(Long id);
    Optional<{pascal}> findByEmail(String email);
    void deleteById(Long id);
}}
"#,
            package = package,
            entity_package = entity_package,
            pascal = pascal,
        );

        write_file(&path.join(format!("{}Repository.java", pascal)), &content)
    }

    fn generate_repository_adapter(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let table_name = to_snake_case(&entity.name);
        let package = self.get_package(layer, logical);

        let entity_package = self.find_entity_package();
        let port_package = self.find_port_repositories_package();

        let jpa_entity_content = format!(r#"package {package}.entities;

import jakarta.persistence.*;
import java.time.LocalDateTime;

@Entity
@Table(name = "{table_name}")
public class {pascal}Entity {{
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(unique = true, nullable = false)
    private String email;

    @Column(nullable = false)
    private String password;

    private String name;

    @Column(name = "created_at")
    private LocalDateTime createdAt;

    @Column(name = "updated_at")
    private LocalDateTime updatedAt;

    public {pascal}Entity() {{}}

    public {pascal}Entity(Long id, String email, String password, String name, LocalDateTime createdAt, LocalDateTime updatedAt) {{
        this.id = id;
        this.email = email;
        this.password = password;
        this.name = name;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }}

    public Long getId() {{ return id; }}
    public void setId(Long id) {{ this.id = id; }}

    public String getEmail() {{ return email; }}
    public void setEmail(String email) {{ this.email = email; }}

    public String getPassword() {{ return password; }}
    public void setPassword(String password) {{ this.password = password; }}

    public String getName() {{ return name; }}
    public void setName(String name) {{ this.name = name; }}

    public LocalDateTime getCreatedAt() {{ return createdAt; }}
    public void setCreatedAt(LocalDateTime createdAt) {{ this.createdAt = createdAt; }}

    public LocalDateTime getUpdatedAt() {{ return updatedAt; }}
    public void setUpdatedAt(LocalDateTime updatedAt) {{ this.updatedAt = updatedAt; }}
}}
"#,
            package = package,
            table_name = table_name,
            pascal = pascal,
        );

        let entities_path = path.join("entities");
        fs::create_dir_all(&entities_path)?;
        write_file(&entities_path.join(format!("{}Entity.java", pascal)), &jpa_entity_content)?;

        let jpa_repo_content = format!(r#"package {package}.jpa;

import {package}.entities.{pascal}Entity;
import org.springframework.data.jpa.repository.JpaRepository;
import java.util.Optional;

public interface {pascal}JpaRepository extends JpaRepository<{pascal}Entity, Long> {{
    Optional<{pascal}Entity> findByEmail(String email);
}}
"#,
            package = package,
            pascal = pascal,
        );

        let jpa_path = path.join("jpa");
        fs::create_dir_all(&jpa_path)?;
        write_file(&jpa_path.join(format!("{}JpaRepository.java", pascal)), &jpa_repo_content)?;

        let repo_impl_content = format!(r#"package {package};

import {entity_package}.{pascal};
import {port_package}.{pascal}Repository;
import {package}.entities.{pascal}Entity;
import {package}.jpa.{pascal}JpaRepository;
import org.springframework.stereotype.Repository;
import java.util.Optional;

@Repository
public class Jpa{pascal}Repository implements {pascal}Repository {{
    private final {pascal}JpaRepository jpaRepository;

    public Jpa{pascal}Repository({pascal}JpaRepository jpaRepository) {{
        this.jpaRepository = jpaRepository;
    }}

    @Override
    public {pascal} save({pascal} domain) {{
        {pascal}Entity entity = toEntity(domain);
        {pascal}Entity saved = jpaRepository.save(entity);
        return toDomain(saved);
    }}

    @Override
    public Optional<{pascal}> findById(Long id) {{
        return jpaRepository.findById(id).map(this::toDomain);
    }}

    @Override
    public Optional<{pascal}> findByEmail(String email) {{
        return jpaRepository.findByEmail(email).map(this::toDomain);
    }}

    @Override
    public void deleteById(Long id) {{
        jpaRepository.deleteById(id);
    }}

    private {pascal}Entity toEntity({pascal} domain) {{
        return new {pascal}Entity(
            domain.getId(),
            domain.getEmail(),
            domain.getPassword(),
            domain.getName(),
            domain.getCreatedAt(),
            domain.getUpdatedAt()
        );
    }}

    private {pascal} toDomain({pascal}Entity entity) {{
        return new {pascal}(
            entity.getId(),
            entity.getEmail(),
            entity.getPassword(),
            entity.getName(),
            entity.getCreatedAt(),
            entity.getUpdatedAt()
        );
    }}
}}
"#,
            package = package,
            entity_package = entity_package,
            port_package = port_package,
            pascal = pascal,
        );

        write_file(&path.join(format!("Jpa{}Repository.java", pascal)), &repo_impl_content)
    }

    fn generate_controller(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let camel = to_camel_case(&entity.name);
        let package = self.get_package(layer, logical);

        let dto_package = package.replace(".controllers", ".dto");

        let content = format!(r#"package {package};

import {dto_package}.Create{pascal}Request;
import {dto_package}.Create{pascal}Response;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/{camel}s")
public class {pascal}Controller {{

    @PostMapping
    public ResponseEntity<Create{pascal}Response> create(@RequestBody Create{pascal}Request request) {{
        // TODO: Implement creation logic
        return ResponseEntity.ok(new Create{pascal}Response(1L, request.email(), request.name()));
    }}

    @GetMapping("/{{id}}")
    public ResponseEntity<Create{pascal}Response> getById(@PathVariable Long id) {{
        // TODO: Implement get by id logic
        return ResponseEntity.ok(new Create{pascal}Response(id, "example@email.com", "Example"));
    }}

    @DeleteMapping("/{{id}}")
    public ResponseEntity<Void> delete(@PathVariable Long id) {{
        // TODO: Implement delete logic
        return ResponseEntity.noContent().build();
    }}
}}
"#,
            package = package,
            dto_package = dto_package,
            pascal = pascal,
            camel = camel,
        );

        write_file(&path.join(format!("{}Controller.java", pascal)), &content)
    }

    fn generate_example_controller(&self, layer: &PhysicalLayer, logical: &str, path: &Path) -> io::Result<()> {
        let package = self.get_package(layer, logical);

        let security_imports = if self.config.features.spring_security {
            "import org.springframework.security.core.Authentication;\n"
        } else {
            ""
        };

        let auth_param = if self.config.features.spring_security {
            "Authentication authentication"
        } else {
            ""
        };

        let auth_name = if self.config.features.spring_security {
            "authentication.getName()"
        } else {
            "\"Guest\""
        };

        let content = format!(r#"package {package};

{security_imports}import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/test")
public class TestController {{

    @GetMapping("/hello")
    public String hello({auth_param}) {{
        return "Hello, " + {auth_name} + "!";
    }}

    @GetMapping("/public")
    public String publicEndpoint() {{
        return "This is a public endpoint!";
    }}

    @GetMapping("/health")
    public String health() {{
        return "OK";
    }}
}}
"#,
            package = package,
            security_imports = security_imports,
            auth_param = auth_param,
            auth_name = auth_name,
        );

        write_file(&path.join("TestController.java"), &content)
    }

    fn generate_dto(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let package = self.get_package(layer, logical);

        let request_content = format!(r#"package {package};

public record Create{pascal}Request(
    String email,
    String password,
    String name
) {{}}
"#,
            package = package,
            pascal = pascal,
        );

        let response_content = format!(r#"package {package};

public record Create{pascal}Response(
    Long id,
    String email,
    String name
) {{}}
"#,
            package = package,
            pascal = pascal,
        );

        write_file(&path.join(format!("Create{}Request.java", pascal)), &request_content)?;
        write_file(&path.join(format!("Create{}Response.java", pascal)), &response_content)
    }

    fn find_security_package(&self) -> String {
        self.config.layers.physical.iter()
            .find(|l| l.is_main)
            .map(|l| format!("{}.{}.config.security", self.config.project.domain, l.name))
            .unwrap_or_default()
    }

    fn generate_auth_controller(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let camel = to_camel_case(&entity.name);
        let package = self.get_package(layer, logical);
        let role = &entity.role;
        let security_package = self.find_security_package();

        let dto_package = if logical.to_lowercase().contains("controllers") {
            package.replace(".controllers", ".dto")
        } else {
            package.clone()
        };

        let dto_imports = if dto_package != package {
            format!(
                "import {dto}.Create{pascal}Request;\nimport {dto}.Create{pascal}Response;\nimport {dto}.Login{pascal}Request;\nimport {dto}.Login{pascal}Response;\n",
                dto = dto_package,
                pascal = pascal,
            )
        } else {
            String::new()
        };

        let content = format!(r#"package {package};

{dto_imports}import {security_package}.JwtService;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/auth/{camel}")
public class {pascal}AuthController {{

    private final JwtService jwtService;

    public {pascal}AuthController(JwtService jwtService) {{
        this.jwtService = jwtService;
    }}

    @PostMapping("/register")
    public ResponseEntity<Create{pascal}Response> register(@RequestBody Create{pascal}Request request) {{
        // TODO: validate and persist entity using the appropriate use case
        return ResponseEntity.status(201).body(new Create{pascal}Response(1L, request.email(), request.name()));
    }}

    @PostMapping("/login")
    public ResponseEntity<Login{pascal}Response> login(@RequestBody Login{pascal}Request request) {{
        // TODO: validate credentials against the repository before issuing token
        String token = jwtService.generateToken(request.email(), "{role}");
        return ResponseEntity.ok(new Login{pascal}Response(token, "{role}"));
    }}
}}
"#,
            package = package,
            dto_imports = dto_imports,
            security_package = security_package,
            pascal = pascal,
            camel = camel,
            role = role,
        );

        write_file(&path.join(format!("{}AuthController.java", pascal)), &content)
    }

    fn generate_auth_dto(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let package = self.get_package(layer, logical);

        let login_request = format!(r#"package {package};

public record Login{pascal}Request(String email, String password) {{}}
"#,
            package = package,
            pascal = pascal,
        );

        let login_response = format!(r#"package {package};

public record Login{pascal}Response(String token, String role) {{}}
"#,
            package = package,
            pascal = pascal,
        );

        write_file(&path.join(format!("Login{}Request.java", pascal)), &login_request)?;
        write_file(&path.join(format!("Login{}Response.java", pascal)), &login_response)
    }

    fn generate_usecase(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let camel = to_camel_case(&entity.name);
        let package = self.get_package(layer, logical);

        let entity_package = self.find_entity_package();
        let port_package = self.find_port_repositories_package();

        let content = format!(r#"package {package};

import {entity_package}.{pascal};
import {port_package}.{pascal}Repository;

public class Create{pascal}UseCase {{
    private final {pascal}Repository {camel}Repository;

    public Create{pascal}UseCase({pascal}Repository {camel}Repository) {{
        this.{camel}Repository = {camel}Repository;
    }}

    public {pascal} execute(String email, String password, String name) {{
        {pascal} {camel} = new {pascal}(email, password, name);
        return {camel}Repository.save({camel});
    }}
}}
"#,
            package = package,
            entity_package = entity_package,
            port_package = port_package,
            pascal = pascal,
            camel = camel,
        );

        write_file(&path.join(format!("Create{}UseCase.java", pascal)), &content)
    }

    fn generate_config_files(&self, layer: &PhysicalLayer, path: &Path) -> io::Result<()> {
        let bean_package = self.get_package(layer, "config");

        let mut bean_imports = String::new();
        let mut bean_definitions = String::new();

        let app_layer = self.config.layers.physical.iter()
            .find(|l| l.logical.iter().any(|log| log.contains("usecases")))
            .map(|l| &l.name);

        if let Some(app) = app_layer {
            for entity in &self.config.entities {
                let pascal = to_pascal_case(&entity.name);

                bean_imports.push_str(&format!(
                    "import {}.{}.usecases.Create{}UseCase;\n",
                    self.config.project.domain, app, pascal
                ));
                bean_imports.push_str(&format!(
                    "import {}.{}.ports.repositories.{}Repository;\n",
                    self.config.project.domain, app, pascal
                ));

                bean_definitions.push_str(&format!(r#"
    @Bean
    public Create{pascal}UseCase create{pascal}UseCase({pascal}Repository repository) {{
        return new Create{pascal}UseCase(repository);
    }}
"#,
                    pascal = pascal,
                ));
            }
        }

        let bean_content = format!(r#"package {bean_package};

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
{bean_imports}
@Configuration
public class BeanConfiguration {{
{bean_definitions}}}
"#,
            bean_package = bean_package,
            bean_imports = bean_imports,
            bean_definitions = bean_definitions,
        );

        write_file(&path.join("BeanConfiguration.java"), &bean_content)?;

        if self.config.features.swagger {
            let swagger_content = format!(r#"package {bean_package}.doc;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.info.Contact;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class OpenAPIConfiguration {{

    @Bean
    public OpenAPI customOpenAPI() {{
        return new OpenAPI()
            .info(new Info()
                .title("{app_name} API")
                .version("1.0.0")
                .description("{description}")
                .contact(new Contact()
                    .name("{dev_name}")
                    .email("{dev_email}")
                    .url("{dev_url}")));
    }}
}}
"#,
                bean_package = bean_package,
                app_name = self.config.project.name,
                description = self.config.project.description,
                dev_name = self.config.project.developer.name,
                dev_email = self.config.project.developer.email,
                dev_url = self.config.project.developer.url,
            );

            let doc_path = path.join("doc");
            fs::create_dir_all(&doc_path)?;
            write_file(&doc_path.join("OpenAPIConfiguration.java"), &swagger_content)?;
        }

        if self.config.features.spring_security {
            self.generate_security_config(layer, path)?;
        }

        Ok(())
    }

    fn generate_security_config(&self, layer: &PhysicalLayer, path: &Path) -> io::Result<()> {
        let package = self.get_package(layer, "config.security");
        let security_path = path.join("security");
        fs::create_dir_all(&security_path)?;

        let security_config = format!(r#"package {package};

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;

@Configuration
@EnableWebSecurity
public class SecurityConfig {{
    private final JwtAuthenticationFilter jwtAuthenticationFilter;

    public SecurityConfig(JwtAuthenticationFilter jwtAuthenticationFilter) {{
        this.jwtAuthenticationFilter = jwtAuthenticationFilter;
    }}

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {{
        http
            .csrf(csrf -> csrf.disable())
            .sessionManagement(session -> session.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
            .authorizeHttpRequests(authz -> authz
                .requestMatchers("/v3/api-docs/**", "/swagger-ui.html", "/swagger-ui/**").permitAll()
                .requestMatchers("/auth/**").permitAll()
                .requestMatchers("/api/test/public", "/api/test/health").permitAll()
                .anyRequest().authenticated()
            )
            .addFilterBefore(jwtAuthenticationFilter, UsernamePasswordAuthenticationFilter.class);

        return http.build();
    }}

    @Bean
    public PasswordEncoder passwordEncoder() {{
        return new BCryptPasswordEncoder();
    }}
}}
"#,
            package = package,
        );

        write_file(&security_path.join("SecurityConfig.java"), &security_config)?;

        let jwt_service = format!(r#"package {package};

import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.SignatureAlgorithm;
import io.jsonwebtoken.io.Decoders;
import io.jsonwebtoken.security.Keys;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.security.Key;
import java.util.Date;

@Service
public class JwtService {{

    @Value("${{jwt.secret}}")
    private String secretKey;

    @Value("${{jwt.expiration}}")
    private Long expiration;

    private Key getSigningKey() {{
        byte[] keyBytes = Decoders.BASE64.decode(secretKey);
        return Keys.hmacShaKeyFor(keyBytes);
    }}

    public String generateToken(String subject, String role) {{
        return Jwts.builder()
            .setSubject(subject)
            .claim("role", role)
            .setIssuedAt(new Date())
            .setExpiration(new Date(System.currentTimeMillis() + expiration))
            .signWith(getSigningKey(), SignatureAlgorithm.HS256)
            .compact();
    }}

    public String extractSubject(String token) {{
        return Jwts.parserBuilder()
            .setSigningKey(getSigningKey())
            .build()
            .parseClaimsJws(token)
            .getBody()
            .getSubject();
    }}

    public boolean isTokenValid(String token) {{
        try {{
            Jwts.parserBuilder().setSigningKey(getSigningKey()).build().parseClaimsJws(token);
            return true;
        }} catch (Exception e) {{
            return false;
        }}
    }}
}}
"#,
            package = package,
        );

        write_file(&security_path.join("JwtService.java"), &jwt_service)?;

        let jwt_filter = format!(r#"package {package};

import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import java.io.IOException;
import java.util.List;

@Component
public class JwtAuthenticationFilter extends OncePerRequestFilter {{

    private final JwtService jwtService;

    public JwtAuthenticationFilter(JwtService jwtService) {{
        this.jwtService = jwtService;
    }}

    @Override
    protected void doFilterInternal(HttpServletRequest request, HttpServletResponse response, FilterChain filterChain)
            throws ServletException, IOException {{

        String authHeader = request.getHeader("Authorization");

        if (authHeader != null && authHeader.startsWith("Bearer ")) {{
            String token = authHeader.substring(7);

            if (jwtService.isTokenValid(token)) {{
                String subject = jwtService.extractSubject(token);
                var authorities = List.of(new SimpleGrantedAuthority("ROLE_USER"));
                var auth = new UsernamePasswordAuthenticationToken(subject, null, authorities);
                SecurityContextHolder.getContext().setAuthentication(auth);
            }}
        }}

        filterChain.doFilter(request, response);
    }}
}}
"#,
            package = package,
        );

        write_file(&security_path.join("JwtAuthenticationFilter.java"), &jwt_filter)
    }

    fn generate_application_class(&self, layer: &PhysicalLayer) -> io::Result<()> {
        let app_name_clean = to_app_name_clean(&self.config.project.name);
        let package = format!("{}.{}", self.config.project.domain, layer.name);

        let layer_root = self.root.join(&layer.name);
        let path = layer_root
            .join("src/main/java")
            .join(domain_to_path(&self.config.project.domain))
            .join(&layer.name);

        let content = format!(r#"package {package};

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.domain.EntityScan;
import org.springframework.context.annotation.ComponentScan;
import org.springframework.data.jpa.repository.config.EnableJpaRepositories;

@SpringBootApplication
@ComponentScan(basePackages = "{domain}")
@EntityScan(basePackages = "{domain}")
@EnableJpaRepositories(basePackages = "{domain}")
public class {app_name_clean}Application {{

    public static void main(String[] args) {{
        SpringApplication.run({app_name_clean}Application.class, args);
    }}
}}
"#,
            package = package,
            domain = self.config.project.domain,
            app_name_clean = app_name_clean,
        );

        write_file(&path.join(format!("{}Application.java", app_name_clean)), &content)
    }

    fn generate_application_properties(&self, layer: &PhysicalLayer, path: &Path) -> io::Result<()> {
        if !layer.is_main {
            return Ok(());
        }

        let mut content = format!(r#"# Application Configuration
spring.application.name={app_name}

# Server Configuration
server.port=8080

# Database Configuration (H2 for development)
spring.datasource.url=jdbc:h2:mem:testdb
spring.datasource.driverClassName=org.h2.Driver
spring.datasource.username=sa
spring.datasource.password=
spring.h2.console.enabled=true

# JPA Configuration
spring.jpa.hibernate.ddl-auto=create-drop
spring.jpa.show-sql=true
spring.jpa.properties.hibernate.format_sql=true

# PostgreSQL Configuration (uncomment for production)
# spring.datasource.url=jdbc:postgresql://localhost:5432/{app_name_lower}
# spring.datasource.username=postgres
# spring.datasource.password=postgres
# spring.jpa.hibernate.ddl-auto=validate
"#,
            app_name = self.config.project.name,
            app_name_lower = self.config.project.name.to_lowercase(),
        );

        if self.config.features.swagger {
            content.push_str(r#"
# Swagger/OpenAPI Configuration
springdoc.api-docs.enabled=true
springdoc.swagger-ui.enabled=true
springdoc.swagger-ui.path=/swagger-ui.html
"#);
        }

        if self.config.features.flyway {
            content.push_str(r#"
# Flyway Configuration
spring.flyway.enabled=true
spring.flyway.locations=classpath:db/migration
"#);

            let migration_path = path.join("db/migration");
            fs::create_dir_all(&migration_path)?;
        }

        if self.config.features.spring_security {
            content.push_str(r#"
# JWT Configuration — replace the secret with a secure random Base64-encoded key in production
jwt.secret=bXktc3VwZXItc2VjcmV0LWtleS1mb3ItaHMyNTYtYWxnb3JpdGhtLWNoYW5nZS1pbi1wcm9kdWN0aW9u
jwt.expiration=86400000
"#);
        }

        write_file(&path.join("application.properties"), &content)
    }

    fn generate_gitignore(&self) -> io::Result<()> {
        let content = r#"# Compiled class files
*.class

# Log files
*.log

# Package files
*.jar
*.war
*.nar
*.ear
*.zip
*.tar.gz
*.rar

# Maven
target/
pom.xml.tag
pom.xml.releaseBackup
pom.xml.versionsBackup
pom.xml.next
release.properties

# IDE
.idea/
*.iml
*.iws
*.ipr
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Application
application-local.properties
application-*.yml
!application.yml

# Secrets
*.env
.env.local
"#;

        write_file(&self.root.join(".gitignore"), content)
    }

    fn generate_gitattributes(&self) -> io::Result<()> {
        let content = r#"* text=auto eol=lf
*.bat text eol=crlf
*.cmd text eol=crlf
"#;

        write_file(&self.root.join(".gitattributes"), content)
    }

    fn generate_mvnw(&self) -> io::Result<()> {
        let wrapper_path = self.root.join(".mvn/wrapper");
        fs::create_dir_all(&wrapper_path)?;

        let wrapper_props = r#"distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.6/apache-maven-3.9.6-bin.zip
wrapperUrl=https://repo.maven.apache.org/maven2/org/apache/maven/wrapper/maven-wrapper/3.2.0/maven-wrapper-3.2.0.jar
"#;
        write_file(&wrapper_path.join("maven-wrapper.properties"), wrapper_props)?;

        let mvnw = r#"#!/bin/sh
exec mvn "$@"
"#;
        let mvnw_path = self.root.join("mvnw");
        write_file(&mvnw_path, mvnw)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&mvnw_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&mvnw_path, perms)?;
        }

        let mvnw_cmd = r#"@echo off
mvn %*
"#;
        write_file(&self.root.join("mvnw.cmd"), mvnw_cmd)
    }
}

fn create_default_template() -> ProjectConfig {
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

fn init_interactive() -> io::Result<()> {
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

fn generate_from_json(config_path: &Path) -> io::Result<()> {
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

fn export_template(output: Option<&Path>) -> io::Result<()> {
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

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init => init_interactive(),
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
