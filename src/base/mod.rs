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

pub mod core;
pub mod application;
pub mod infrastructure;

use std::fs;
use std::io;
use std::path::PathBuf;

use crate::utils::{domain_to_path, to_app_name_clean, write_file};

pub struct BaseGenerator {
    pub root: PathBuf,
    pub name: String,
    pub domain: String,
    pub description: String,
    pub dev_name: String,
    pub dev_email: String,
    pub dev_url: String,
}

impl BaseGenerator {
    pub fn new(root: PathBuf, name: String, domain: String, description: String,
           dev_name: String, dev_email: String, dev_url: String) -> Self {
        Self { root, name, domain, description, dev_name, dev_email, dev_url }
    }

    pub fn dp(&self) -> String { domain_to_path(&self.domain) }
    pub fn app_class(&self) -> String { to_app_name_clean(&self.name) }

    pub fn java_src(&self, module: &str) -> PathBuf {
        self.root.join(module).join("src/main/java").join(self.dp()).join(module)
    }

    pub fn generate(&self) -> io::Result<()> {
        self.generate_root_pom()?;
        self.generate_core_pom()?;
        self.generate_admin_entity()?;
        self.generate_cliente_entity()?;
        self.generate_application_pom()?;
        self.generate_exceptions()?;
        self.generate_dtos()?;
        self.generate_repository_ports()?;
        self.generate_service_ports()?;
        self.generate_usecases()?;
        self.generate_infrastructure_pom()?;
        self.generate_jpa_entities()?;
        self.generate_jpa_repositories()?;
        self.generate_repository_adapters()?;
        self.generate_auth_controller()?;
        self.generate_bean_configuration()?;
        self.generate_security()?;
        self.generate_application_class()?;
        self.generate_application_properties()?;
        self.generate_gitignore()?;
        self.generate_gitattributes()?;
        self.generate_mvnw()
    }

    pub fn generate_root_pom(&self) -> io::Result<()> {
        let content = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>3.5.6</version>
        <relativePath/>
    </parent>

    <groupId>{domain}</groupId>
    <artifactId>{name}</artifactId>
    <version>1.0.0</version>
    <packaging>pom</packaging>
    <description>{description}</description>

    <modules>
        <module>core</module>
        <module>application</module>
        <module>infrastructure</module>
    </modules>

    <properties>
        <java.version>21</java.version>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <developers>
        <developer>
            <name>{dev_name}</name>
            <email>{dev_email}</email>
            <url>{dev_url}</url>
        </developer>
    </developers>
</project>"#,
            domain = self.domain, name = self.name, description = self.description,
            dev_name = self.dev_name, dev_email = self.dev_email, dev_url = self.dev_url,
        );
        write_file(&self.root.join("pom.xml"), &content)
    }

    pub fn generate_gitignore(&self) -> io::Result<()> {
        let content = "HELP.md\ntarget/\n*.class\n*.jar\n.idea\n*.iml\n.DS_Store\n";
        write_file(&self.root.join(".gitignore"), content)
    }

    pub fn generate_gitattributes(&self) -> io::Result<()> {
        write_file(&self.root.join(".gitattributes"), "* text=auto eol=lf\n*.bat text eol=crlf\n")
    }

    pub fn generate_mvnw(&self) -> io::Result<()> {
        let wrapper_path = self.root.join(".mvn/wrapper");
        fs::create_dir_all(&wrapper_path)?;
        write_file(&wrapper_path.join("maven-wrapper.properties"),
            "distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.6/apache-maven-3.9.6-bin.zip\n")?;

        let mvnw = r#"#!/bin/sh
set -e
MAVEN_PROJECTBASEDIR="${MAVEN_PROJECTBASEDIR:-$(cd "$(dirname "$0")" && pwd)}"
WRAPPER_PROPERTIES="$MAVEN_PROJECTBASEDIR/.mvn/wrapper/maven-wrapper.properties"
if [ -r "$WRAPPER_PROPERTIES" ]; then
    DISTRIBUTION_URL=$(grep "^distributionUrl" "$WRAPPER_PROPERTIES" | cut -d= -f2- | tr -d '\r')
else
    DISTRIBUTION_URL="https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.6/apache-maven-3.9.6-bin.zip"
fi
DIST_FILE=$(basename "$DISTRIBUTION_URL")
DIST_NAME=$(echo "$DIST_FILE" | sed 's/-bin\.zip$//')
MAVEN_HOME="${HOME}/.m2/wrapper/dists/${DIST_NAME}"
if [ ! -f "$MAVEN_HOME/bin/mvn" ]; then
    echo "Downloading Maven ${DIST_NAME}..."
    TMP_DIR=$(mktemp -d)
    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "$DISTRIBUTION_URL" -o "$TMP_DIR/$DIST_FILE"
    elif command -v wget > /dev/null 2>&1; then
        wget -q "$DISTRIBUTION_URL" -O "$TMP_DIR/$DIST_FILE"
    else
        echo "Error: curl or wget required." >&2; exit 1
    fi
    mkdir -p "$MAVEN_HOME"
    unzip -q "$TMP_DIR/$DIST_FILE" -d "$TMP_DIR/extract"
    mv "$TMP_DIR/extract/${DIST_NAME}"/* "$MAVEN_HOME/"
    rm -rf "$TMP_DIR"
fi
exec "$MAVEN_HOME/bin/mvn" "$@"
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
        write_file(&self.root.join("mvnw.cmd"), "@echo off\r\necho Use ./mvnw on Unix or install Maven manually.\r\n")
    }
}

pub fn run_base() -> io::Result<()> {
    use crate::constants::*;
    use crate::ui::{prompt, prompt_required, print_section};

    println!("{}", BANNER);
    print_section("Base Project");

    let app_name = prompt_required("Application name (e.g.: myAPI)").replace(' ', "-");
    let domain   = prompt_required("Domain (e.g.: com.example.demo)");
    let description = prompt("Description", Some("Spring Boot API"));

    print_section("Developer");
    let dev_name  = prompt("Name", Some("Developer"));
    let dev_email = prompt("Email", Some("dev@example.com"));
    let dev_url   = prompt("URL", Some("example.com"));

    let root = PathBuf::from(&app_name);
    if root.exists() {
        eprintln!("  {YELLOW}! Directory '{app_name}' already exists.{RESET}");
        std::process::exit(1);
    }

    println!("\n  {CYAN}Generating: {BOLD}{app_name}{RESET}");
    println!("  {CYAN}Entities:   Admin (ADMIN)  ·  Cliente (CLIENT){RESET}");
    println!("  {CYAN}Auth:       JWT cookie  ·  BCrypt{RESET}\n");

    BaseGenerator::new(root, app_name.clone(), domain, description, dev_name, dev_email, dev_url)
        .generate()?;

    println!("\n  {GREEN}{BOLD}✓ '{app_name}' generated!{RESET}");
    println!("  {CYAN}→ cd {app_name} && ./mvnw spring-boot:run{RESET}\n");
    Ok(())
}
