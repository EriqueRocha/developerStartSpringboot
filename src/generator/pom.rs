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

use std::io;
use crate::config::PhysicalLayer;
use crate::utils::write_file;
use super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub fn generate_root_pom(&self) -> io::Result<()> {
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

    pub fn generate_layer_pom(&self, layer: &PhysicalLayer) -> io::Result<()> {
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

        let mut entity_layers: Vec<String> = Vec::new();
        for entity in &self.config.entities {
            let el = entity.location.as_ref()
                .map(|l| l.layer.clone())
                .or_else(|| self.config.layers.entity_location.as_ref().map(|l| l.layer.clone()));
            if let Some(el) = el {
                if !entity_layers.contains(&el) {
                    entity_layers.push(el);
                }
            }
        }

        let needs_entity_dep = layer.logical.iter().any(|logical| {
            let lower = logical.to_lowercase();
            lower.contains("repositor")
                || lower.contains("controller")
                || lower.contains("usecase")
                || lower.contains("dto")
        });

        if needs_entity_dep {
            for entity_layer in &entity_layers {
                let is_same_layer = entity_layer == &layer.name;
                let already_declared = layer.dependencies.iter().any(|d| d == entity_layer);
                if !is_same_layer && !already_declared {
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
}
