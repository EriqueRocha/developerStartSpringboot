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
use std::path::Path;

use crate::config::{PhysicalLayer, EntityConfig};
use crate::utils::{to_pascal_case, to_camel_case, to_snake_case, write_file};
use super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub fn generate_entity(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
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

    pub fn generate_repository_port(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let package = self.get_package(layer, logical);
        let entity_package = self.find_entity_package(entity);

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

    pub fn generate_repository_adapter(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let table_name = to_snake_case(&entity.name);
        let package = self.get_package(layer, logical);

        let entity_package = self.find_entity_package(entity);
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

    pub fn generate_controller(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
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

    pub fn generate_example_controller(&self, layer: &PhysicalLayer, logical: &str, path: &Path) -> io::Result<()> {
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

    pub fn generate_dto(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
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

    pub fn generate_auth_controller(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
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

    pub fn generate_auth_dto(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
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

    pub fn generate_usecase(&self, layer: &PhysicalLayer, logical: &str, path: &Path, entity: &EntityConfig) -> io::Result<()> {
        let pascal = to_pascal_case(&entity.name);
        let camel = to_camel_case(&entity.name);
        let package = self.get_package(layer, logical);

        let entity_package = self.find_entity_package(entity);
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
}
