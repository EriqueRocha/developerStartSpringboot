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
use crate::utils::write_file;
use super::BaseGenerator;

impl BaseGenerator {
    pub fn generate_application_pom(&self) -> io::Result<()> {
        let content = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <parent>
        <groupId>{domain}</groupId>
        <artifactId>{name}</artifactId>
        <version>1.0.0</version>
    </parent>

    <artifactId>application</artifactId>
    <packaging>jar</packaging>

    <dependencies>
        <dependency>
            <groupId>{domain}</groupId>
            <artifactId>core</artifactId>
            <version>1.0.0</version>
        </dependency>
        <dependency>
            <groupId>jakarta.validation</groupId>
            <artifactId>jakarta.validation-api</artifactId>
        </dependency>
    </dependencies>
</project>"#, domain = self.domain, name = self.name);
        write_file(&self.root.join("application/pom.xml"), &content)
    }

    pub fn generate_exceptions(&self) -> io::Result<()> {
        let pkg = format!("{}.application.exceptions", self.domain);
        let base = self.java_src("application").join("exceptions");

        write_file(&base.join("ApplicationException.java"), &format!(r#"package {pkg};

public abstract class ApplicationException extends RuntimeException {{
    private final String errorCode;

    protected ApplicationException(String message, String errorCode) {{
        super(message);
        this.errorCode = errorCode;
    }}

    public String getErrorCode() {{ return errorCode; }}
}}"#, pkg = pkg))?;

        write_file(&base.join("AuthenticationException.java"), &format!(r#"package {pkg};

public class AuthenticationException extends ApplicationException {{
    public static final String CODE = "AUTHENTICATION_FAILED";

    public AuthenticationException(String message) {{ super(message, CODE); }}
    public AuthenticationException() {{ super("Invalid credentials", CODE); }}
}}"#, pkg = pkg))?;

        write_file(&base.join("ResourceNotFoundException.java"), &format!(r#"package {pkg};

public class ResourceNotFoundException extends ApplicationException {{
    public static final String CODE = "RESOURCE_NOT_FOUND";

    public ResourceNotFoundException(String message) {{ super(message, CODE); }}

    public ResourceNotFoundException(String resource, String field, String value) {{
        super(resource + " not found with " + field + ": " + value, CODE);
    }}
}}"#, pkg = pkg))?;

        write_file(&base.join("ResourceAlreadyExistsException.java"), &format!(r#"package {pkg};

public class ResourceAlreadyExistsException extends ApplicationException {{
    public static final String CODE = "RESOURCE_ALREADY_EXISTS";

    public ResourceAlreadyExistsException(String message) {{ super(message, CODE); }}

    public ResourceAlreadyExistsException(String resource, String field, String value) {{
        super(resource + " already exists with " + field + ": " + value, CODE);
    }}
}}"#, pkg = pkg))
    }

    pub fn generate_dtos(&self) -> io::Result<()> {
        let pkg = format!("{}.application.ports.dto", self.domain);
        let base = self.java_src("application").join("ports/dto");

        write_file(&base.join("LoginRequest.java"), &format!(r#"package {pkg};

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;

public record LoginRequest(
    @NotBlank(message = "Email is required")
    @Email(message = "Email must be valid")
    String email,

    @NotBlank(message = "Password is required")
    String password
) {{}}"#, pkg = pkg))?;

        write_file(&base.join("LoginResponse.java"), &format!(r#"package {pkg};

public record LoginResponse(
    String message,
    String role,
    String email,
    String name,
    String token
) {{}}"#, pkg = pkg))?;

        write_file(&base.join("RegisterClienteRequest.java"), &format!(r#"package {pkg};

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

public record RegisterClienteRequest(
    @NotBlank(message = "Email is required")
    @Email(message = "Email must be valid")
    String email,

    @NotBlank(message = "Password is required")
    @Size(min = 6, message = "Password must be at least 6 characters")
    String password,

    @NotBlank(message = "Name is required")
    String name
) {{}}"#, pkg = pkg))
    }

    pub fn generate_repository_ports(&self) -> io::Result<()> {
        let pkg = format!("{}.application.ports.repositories", self.domain);
        let entities = format!("{}.core.domain.entities", self.domain);
        let base = self.java_src("application").join("ports/repositories");

        write_file(&base.join("AdminRepository.java"), &format!(r#"package {pkg};

import {entities}.Admin;
import java.util.Optional;
import java.util.UUID;

public interface AdminRepository {{
    Admin save(Admin admin);
    Optional<Admin> findById(UUID id);
    Optional<Admin> findByEmail(String email);
    void deleteById(UUID id);
}}"#, pkg = pkg, entities = entities))?;

        write_file(&base.join("ClienteRepository.java"), &format!(r#"package {pkg};

import {entities}.Cliente;
import java.util.Optional;
import java.util.UUID;

public interface ClienteRepository {{
    Cliente save(Cliente cliente);
    Optional<Cliente> findById(UUID id);
    Optional<Cliente> findByEmail(String email);
    void deleteById(UUID id);
    boolean existsByEmail(String email);
}}"#, pkg = pkg, entities = entities))
    }

    pub fn generate_service_ports(&self) -> io::Result<()> {
        let pkg = format!("{}.application.ports.services", self.domain);
        let base = self.java_src("application").join("ports/services");

        write_file(&base.join("TokenService.java"), &format!(r#"package {pkg};

import java.util.UUID;

public interface TokenService {{
    String generateToken(UUID userId, String email, String role);
    boolean validateToken(String token);
    String extractEmail(String token);
    String extractRole(String token);
    UUID extractUserId(String token);
}}"#, pkg = pkg))?;

        write_file(&base.join("PasswordService.java"), &format!(r#"package {pkg};

public interface PasswordService {{
    String encode(String rawPassword);
    boolean matches(String rawPassword, String encodedPassword);
}}"#, pkg = pkg))
    }

    pub fn generate_usecases(&self) -> io::Result<()> {
        let uc = format!("{}.application.usecases", self.domain);
        let dto = format!("{}.application.ports.dto", self.domain);
        let repo = format!("{}.application.ports.repositories", self.domain);
        let svc = format!("{}.application.ports.services", self.domain);
        let entities = format!("{}.core.domain.entities", self.domain);
        let exc = format!("{}.application.exceptions", self.domain);
        let base = self.java_src("application").join("usecases");

        write_file(&base.join("AuthenticateAdminUseCase.java"), &format!(r#"package {uc};

import {dto}.LoginRequest;
import {dto}.LoginResponse;
import {repo}.AdminRepository;
import {svc}.PasswordService;
import {svc}.TokenService;
import {exc}.AuthenticationException;
import {entities}.Admin;

public class AuthenticateAdminUseCase {{
    private final AdminRepository adminRepository;
    private final PasswordService passwordService;
    private final TokenService tokenService;

    public AuthenticateAdminUseCase(AdminRepository adminRepository,
                                    PasswordService passwordService,
                                    TokenService tokenService) {{
        this.adminRepository = adminRepository;
        this.passwordService = passwordService;
        this.tokenService = tokenService;
    }}

    public LoginResponse execute(LoginRequest request) {{
        Admin admin = adminRepository.findByEmail(request.email())
            .orElseThrow(AuthenticationException::new);

        if (!passwordService.matches(request.password(), admin.getPassword())) {{
            throw new AuthenticationException();
        }}

        String token = tokenService.generateToken(admin.getId(), admin.getEmail(), admin.getRole());
        return new LoginResponse("Login successful", admin.getRole(), admin.getEmail(), admin.getName(), token);
    }}
}}"#, uc=uc, dto=dto, repo=repo, svc=svc, exc=exc, entities=entities))?;

        write_file(&base.join("AuthenticateClienteUseCase.java"), &format!(r#"package {uc};

import {dto}.LoginRequest;
import {dto}.LoginResponse;
import {repo}.ClienteRepository;
import {svc}.PasswordService;
import {svc}.TokenService;
import {exc}.AuthenticationException;
import {entities}.Cliente;

public class AuthenticateClienteUseCase {{
    private final ClienteRepository clienteRepository;
    private final PasswordService passwordService;
    private final TokenService tokenService;

    public AuthenticateClienteUseCase(ClienteRepository clienteRepository,
                                      PasswordService passwordService,
                                      TokenService tokenService) {{
        this.clienteRepository = clienteRepository;
        this.passwordService = passwordService;
        this.tokenService = tokenService;
    }}

    public LoginResponse execute(LoginRequest request) {{
        Cliente cliente = clienteRepository.findByEmail(request.email())
            .orElseThrow(AuthenticationException::new);

        if (!passwordService.matches(request.password(), cliente.getPassword())) {{
            throw new AuthenticationException();
        }}

        String token = tokenService.generateToken(cliente.getId(), cliente.getEmail(), cliente.getRole());
        return new LoginResponse("Login successful", cliente.getRole(), cliente.getEmail(), cliente.getName(), token);
    }}
}}"#, uc=uc, dto=dto, repo=repo, svc=svc, exc=exc, entities=entities))?;

        write_file(&base.join("CreateClienteUseCase.java"), &format!(r#"package {uc};

import {dto}.RegisterClienteRequest;
import {dto}.LoginResponse;
import {repo}.ClienteRepository;
import {svc}.PasswordService;
import {svc}.TokenService;
import {exc}.ResourceAlreadyExistsException;
import {entities}.Cliente;

public class CreateClienteUseCase {{
    private final ClienteRepository clienteRepository;
    private final PasswordService passwordService;
    private final TokenService tokenService;

    public CreateClienteUseCase(ClienteRepository clienteRepository,
                                PasswordService passwordService,
                                TokenService tokenService) {{
        this.clienteRepository = clienteRepository;
        this.passwordService = passwordService;
        this.tokenService = tokenService;
    }}

    public LoginResponse execute(RegisterClienteRequest request) {{
        if (clienteRepository.existsByEmail(request.email())) {{
            throw new ResourceAlreadyExistsException("Cliente", "email", request.email());
        }}

        Cliente cliente = new Cliente(
            request.email(),
            passwordService.encode(request.password()),
            request.name()
        );

        Cliente saved = clienteRepository.save(cliente);
        String token = tokenService.generateToken(saved.getId(), saved.getEmail(), saved.getRole());
        return new LoginResponse("Registration successful", saved.getRole(), saved.getEmail(), saved.getName(), token);
    }}
}}"#, uc=uc, dto=dto, repo=repo, svc=svc, exc=exc, entities=entities))
    }
}
