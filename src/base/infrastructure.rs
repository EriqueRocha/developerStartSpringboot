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
    pub fn generate_infrastructure_pom(&self) -> io::Result<()> {
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

    <artifactId>infrastructure</artifactId>
    <packaging>jar</packaging>

    <dependencies>
        <dependency>
            <groupId>{domain}</groupId>
            <artifactId>core</artifactId>
            <version>1.0.0</version>
        </dependency>
        <dependency>
            <groupId>{domain}</groupId>
            <artifactId>application</artifactId>
            <version>1.0.0</version>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-security</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-data-jpa</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-validation</artifactId>
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
        </dependency>
        <dependency>
            <groupId>org.springdoc</groupId>
            <artifactId>springdoc-openapi-starter-webmvc-ui</artifactId>
            <version>2.3.0</version>
        </dependency>
    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
        </plugins>
    </build>
</project>"#, domain = self.domain, name = self.name);
        write_file(&self.root.join("infrastructure/pom.xml"), &content)
    }

    pub fn generate_jpa_entities(&self) -> io::Result<()> {
        let pkg = format!("{}.infrastructure.adapters.repositories.entities", self.domain);
        let base = self.java_src("infrastructure").join("adapters/repositories/entities");

        write_file(&base.join("BaseEntity.java"), &format!(r#"package {pkg};

import jakarta.persistence.*;
import java.time.LocalDateTime;
import java.util.UUID;

@MappedSuperclass
public abstract class BaseEntity {{
    @Id
    private UUID id;

    @Column(name = "created_at", nullable = false, updatable = false)
    private LocalDateTime createdAt;

    @Column(name = "updated_at", nullable = false)
    private LocalDateTime updatedAt;

    public BaseEntity() {{ this.id = UUID.randomUUID(); }}

    @PrePersist
    protected void onCreate() {{
        LocalDateTime now = LocalDateTime.now();
        this.createdAt = now;
        this.updatedAt = now;
    }}

    @PreUpdate
    protected void onUpdate() {{ this.updatedAt = LocalDateTime.now(); }}

    public UUID getId() {{ return id; }}
    public LocalDateTime getCreatedAt() {{ return createdAt; }}
    public LocalDateTime getUpdatedAt() {{ return updatedAt; }}
    public void setId(UUID id) {{ this.id = id; }}
}}"#, pkg = pkg))?;

        write_file(&base.join("AdminEntity.java"), &format!(r#"package {pkg};

import jakarta.persistence.*;

@Entity
@Table(name = "admins")
public class AdminEntity extends BaseEntity {{
    @Column(unique = true, nullable = false)
    private String email;

    @Column(nullable = false)
    private String password;

    @Column(nullable = false)
    private String name;

    public String getEmail() {{ return email; }}
    public void setEmail(String email) {{ this.email = email; }}
    public String getPassword() {{ return password; }}
    public void setPassword(String password) {{ this.password = password; }}
    public String getName() {{ return name; }}
    public void setName(String name) {{ this.name = name; }}
}}"#, pkg = pkg))?;

        write_file(&base.join("ClienteEntity.java"), &format!(r#"package {pkg};

import jakarta.persistence.*;

@Entity
@Table(name = "clientes")
public class ClienteEntity extends BaseEntity {{
    @Column(unique = true, nullable = false)
    private String email;

    @Column(nullable = false)
    private String password;

    @Column(nullable = false)
    private String name;

    public String getEmail() {{ return email; }}
    public void setEmail(String email) {{ this.email = email; }}
    public String getPassword() {{ return password; }}
    public void setPassword(String password) {{ this.password = password; }}
    public String getName() {{ return name; }}
    public void setName(String name) {{ this.name = name; }}
}}"#, pkg = pkg))
    }

    pub fn generate_jpa_repositories(&self) -> io::Result<()> {
        let pkg = format!("{}.infrastructure.adapters.repositories.jpa", self.domain);
        let entities = format!("{}.infrastructure.adapters.repositories.entities", self.domain);
        let base = self.java_src("infrastructure").join("adapters/repositories/jpa");

        write_file(&base.join("AdminJpaRepository.java"), &format!(r#"package {pkg};

import {entities}.AdminEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;
import java.util.Optional;
import java.util.UUID;

@Repository
public interface AdminJpaRepository extends JpaRepository<AdminEntity, UUID> {{
    Optional<AdminEntity> findByEmail(String email);
}}"#, pkg = pkg, entities = entities))?;

        write_file(&base.join("ClienteJpaRepository.java"), &format!(r#"package {pkg};

import {entities}.ClienteEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;
import java.util.Optional;
import java.util.UUID;

@Repository
public interface ClienteJpaRepository extends JpaRepository<ClienteEntity, UUID> {{
    Optional<ClienteEntity> findByEmail(String email);
    boolean existsByEmail(String email);
}}"#, pkg = pkg, entities = entities))
    }

    pub fn generate_repository_adapters(&self) -> io::Result<()> {
        let pkg = format!("{}.infrastructure.adapters.repositories", self.domain);
        let jpa = format!("{}.infrastructure.adapters.repositories.jpa", self.domain);
        let ent = format!("{}.infrastructure.adapters.repositories.entities", self.domain);
        let port = format!("{}.application.ports.repositories", self.domain);
        let dom = format!("{}.core.domain.entities", self.domain);
        let base = self.java_src("infrastructure").join("adapters/repositories");

        write_file(&base.join("JpaAdminRepository.java"), &format!(r#"package {pkg};

import {port}.AdminRepository;
import {dom}.Admin;
import {jpa}.AdminJpaRepository;
import {ent}.AdminEntity;
import org.springframework.stereotype.Component;
import java.util.Optional;
import java.util.UUID;

@Component
public class JpaAdminRepository implements AdminRepository {{
    private final AdminJpaRepository jpaRepository;

    public JpaAdminRepository(AdminJpaRepository jpaRepository) {{
        this.jpaRepository = jpaRepository;
    }}

    @Override
    public Admin save(Admin admin) {{
        return toDomain(jpaRepository.save(toEntity(admin)));
    }}

    @Override
    public Optional<Admin> findById(UUID id) {{
        return jpaRepository.findById(id).map(this::toDomain);
    }}

    @Override
    public Optional<Admin> findByEmail(String email) {{
        return jpaRepository.findByEmail(email).map(this::toDomain);
    }}

    @Override
    public void deleteById(UUID id) {{
        jpaRepository.deleteById(id);
    }}

    private Admin toDomain(AdminEntity e) {{
        return new Admin(e.getId(), e.getEmail(), e.getPassword(), e.getName());
    }}

    private AdminEntity toEntity(Admin a) {{
        AdminEntity e = new AdminEntity();
        if (a.getId() != null) e.setId(a.getId());
        e.setEmail(a.getEmail());
        e.setPassword(a.getPassword());
        e.setName(a.getName());
        return e;
    }}
}}"#, pkg=pkg, port=port, dom=dom, jpa=jpa, ent=ent))?;

        write_file(&base.join("JpaClienteRepository.java"), &format!(r#"package {pkg};

import {port}.ClienteRepository;
import {dom}.Cliente;
import {jpa}.ClienteJpaRepository;
import {ent}.ClienteEntity;
import org.springframework.stereotype.Component;
import java.util.Optional;
import java.util.UUID;

@Component
public class JpaClienteRepository implements ClienteRepository {{
    private final ClienteJpaRepository jpaRepository;

    public JpaClienteRepository(ClienteJpaRepository jpaRepository) {{
        this.jpaRepository = jpaRepository;
    }}

    @Override
    public Cliente save(Cliente cliente) {{
        return toDomain(jpaRepository.save(toEntity(cliente)));
    }}

    @Override
    public Optional<Cliente> findById(UUID id) {{
        return jpaRepository.findById(id).map(this::toDomain);
    }}

    @Override
    public Optional<Cliente> findByEmail(String email) {{
        return jpaRepository.findByEmail(email).map(this::toDomain);
    }}

    @Override
    public void deleteById(UUID id) {{
        jpaRepository.deleteById(id);
    }}

    @Override
    public boolean existsByEmail(String email) {{
        return jpaRepository.existsByEmail(email);
    }}

    private Cliente toDomain(ClienteEntity e) {{
        return new Cliente(e.getId(), e.getEmail(), e.getPassword(), e.getName());
    }}

    private ClienteEntity toEntity(Cliente c) {{
        ClienteEntity e = new ClienteEntity();
        if (c.getId() != null) e.setId(c.getId());
        e.setEmail(c.getEmail());
        e.setPassword(c.getPassword());
        e.setName(c.getName());
        return e;
    }}
}}"#, pkg=pkg, port=port, dom=dom, jpa=jpa, ent=ent))
    }

    pub fn generate_auth_controller(&self) -> io::Result<()> {
        let pkg = format!("{}.infrastructure.adapters.web.controllers", self.domain);
        let dto = format!("{}.application.ports.dto", self.domain);
        let uc = format!("{}.application.usecases", self.domain);
        let base = self.java_src("infrastructure").join("adapters/web/controllers");

        write_file(&base.join("AuthController.java"), &format!(r#"package {pkg};

import {dto}.LoginRequest;
import {dto}.LoginResponse;
import {dto}.RegisterClienteRequest;
import {uc}.AuthenticateAdminUseCase;
import {uc}.AuthenticateClienteUseCase;
import {uc}.CreateClienteUseCase;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.servlet.http.Cookie;
import jakarta.servlet.http.HttpServletResponse;
import jakarta.validation.Valid;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/auth")
@Tag(name = "Authentication")
public class AuthController {{
    private final AuthenticateAdminUseCase authenticateAdminUseCase;
    private final AuthenticateClienteUseCase authenticateClienteUseCase;
    private final CreateClienteUseCase createClienteUseCase;

    public AuthController(AuthenticateAdminUseCase authenticateAdminUseCase,
                          AuthenticateClienteUseCase authenticateClienteUseCase,
                          CreateClienteUseCase createClienteUseCase) {{
        this.authenticateAdminUseCase = authenticateAdminUseCase;
        this.authenticateClienteUseCase = authenticateClienteUseCase;
        this.createClienteUseCase = createClienteUseCase;
    }}

    @PostMapping("/admin/login")
    @Operation(summary = "Admin login")
    public ResponseEntity<LoginResponse> adminLogin(@Valid @RequestBody LoginRequest request,
                                                     HttpServletResponse response) {{
        LoginResponse res = authenticateAdminUseCase.execute(request);
        addAuthCookie(response, res.token());
        return ResponseEntity.ok(res);
    }}

    @PostMapping("/cliente/login")
    @Operation(summary = "Cliente login")
    public ResponseEntity<LoginResponse> clienteLogin(@Valid @RequestBody LoginRequest request,
                                                       HttpServletResponse response) {{
        LoginResponse res = authenticateClienteUseCase.execute(request);
        addAuthCookie(response, res.token());
        return ResponseEntity.ok(res);
    }}

    @PostMapping("/cliente/register")
    @Operation(summary = "Cliente registration")
    public ResponseEntity<LoginResponse> registerCliente(@Valid @RequestBody RegisterClienteRequest request,
                                                          HttpServletResponse response) {{
        LoginResponse res = createClienteUseCase.execute(request);
        addAuthCookie(response, res.token());
        return ResponseEntity.status(201).body(res);
    }}

    @PostMapping("/logout")
    @Operation(summary = "Logout")
    public ResponseEntity<Void> logout(HttpServletResponse response) {{
        Cookie cookie = new Cookie("token", "");
        cookie.setHttpOnly(true);
        cookie.setMaxAge(0);
        cookie.setPath("/");
        response.addCookie(cookie);
        return ResponseEntity.noContent().build();
    }}

    @GetMapping("/validate")
    @Operation(summary = "Validate token")
    public ResponseEntity<Boolean> validate() {{
        return ResponseEntity.ok(true);
    }}

    private void addAuthCookie(HttpServletResponse response, String token) {{
        Cookie cookie = new Cookie("token", token);
        cookie.setHttpOnly(true);
        cookie.setMaxAge(24 * 60 * 60);
        cookie.setPath("/");
        response.addCookie(cookie);
    }}
}}"#, pkg=pkg, dto=dto, uc=uc))
    }

    pub fn generate_bean_configuration(&self) -> io::Result<()> {
        let pkg = format!("{}.infrastructure.config", self.domain);
        let uc = format!("{}.application.usecases", self.domain);
        let repo = format!("{}.application.ports.repositories", self.domain);
        let svc = format!("{}.application.ports.services", self.domain);
        let base = self.java_src("infrastructure").join("config");

        write_file(&base.join("BeanConfiguration.java"), &format!(r#"package {pkg};

import {uc}.AuthenticateAdminUseCase;
import {uc}.AuthenticateClienteUseCase;
import {uc}.CreateClienteUseCase;
import {repo}.AdminRepository;
import {repo}.ClienteRepository;
import {svc}.PasswordService;
import {svc}.TokenService;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class BeanConfiguration {{

    @Bean
    public AuthenticateAdminUseCase authenticateAdminUseCase(AdminRepository adminRepository,
                                                              PasswordService passwordService,
                                                              TokenService tokenService) {{
        return new AuthenticateAdminUseCase(adminRepository, passwordService, tokenService);
    }}

    @Bean
    public AuthenticateClienteUseCase authenticateClienteUseCase(ClienteRepository clienteRepository,
                                                                   PasswordService passwordService,
                                                                   TokenService tokenService) {{
        return new AuthenticateClienteUseCase(clienteRepository, passwordService, tokenService);
    }}

    @Bean
    public CreateClienteUseCase createClienteUseCase(ClienteRepository clienteRepository,
                                                      PasswordService passwordService,
                                                      TokenService tokenService) {{
        return new CreateClienteUseCase(clienteRepository, passwordService, tokenService);
    }}
}}"#, pkg=pkg, uc=uc, repo=repo, svc=svc))
    }

    pub fn generate_security(&self) -> io::Result<()> {
        let pkg = format!("{}.infrastructure.config.security", self.domain);
        let svc = format!("{}.application.ports.services", self.domain);
        let base = self.java_src("infrastructure").join("config/security");

        write_file(&base.join("JwtTokenService.java"), &format!(r#"package {pkg};

import {svc}.TokenService;
import io.jsonwebtoken.Claims;
import io.jsonwebtoken.JwtException;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.SignatureAlgorithm;
import io.jsonwebtoken.security.Keys;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;
import java.util.Date;
import java.util.UUID;

@Service
public class JwtTokenService implements TokenService {{
    @Value("${{jwt.secret:change-this-secret-key-in-production}}")
    private String secret;

    @Value("${{jwt.expiration:86400000}}")
    private long expiration;

    @Override
    public String generateToken(UUID userId, String email, String role) {{
        Date now = new Date();
        return Jwts.builder()
            .setSubject(email)
            .claim("userId", userId.toString())
            .claim("role", role)
            .setIssuedAt(now)
            .setExpiration(new Date(now.getTime() + expiration))
            .signWith(Keys.hmacShaKeyFor(secret.getBytes()), SignatureAlgorithm.HS512)
            .compact();
    }}

    @Override
    public boolean validateToken(String token) {{
        try {{
            Jwts.parserBuilder()
                .setSigningKey(Keys.hmacShaKeyFor(secret.getBytes()))
                .build()
                .parseClaimsJws(token);
            return true;
        }} catch (JwtException | IllegalArgumentException e) {{
            return false;
        }}
    }}

    @Override
    public String extractEmail(String token) {{ return getClaims(token).getSubject(); }}

    @Override
    public String extractRole(String token) {{ return getClaims(token).get("role", String.class); }}

    @Override
    public UUID extractUserId(String token) {{
        return UUID.fromString(getClaims(token).get("userId", String.class));
    }}

    private Claims getClaims(String token) {{
        return Jwts.parserBuilder()
            .setSigningKey(Keys.hmacShaKeyFor(secret.getBytes()))
            .build()
            .parseClaimsJws(token)
            .getBody();
    }}
}}"#, pkg=pkg, svc=svc))?;

        write_file(&base.join("BCryptPasswordService.java"), &format!(r#"package {pkg};

import {svc}.PasswordService;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.stereotype.Service;

@Service
public class BCryptPasswordService implements PasswordService {{
    private final BCryptPasswordEncoder encoder = new BCryptPasswordEncoder();

    @Override
    public String encode(String rawPassword) {{ return encoder.encode(rawPassword); }}

    @Override
    public boolean matches(String rawPassword, String encodedPassword) {{
        return encoder.matches(rawPassword, encodedPassword);
    }}
}}"#, pkg=pkg, svc=svc))?;

        write_file(&base.join("AuthenticatedUser.java"), &format!(r#"package {pkg};

import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.userdetails.UserDetails;
import java.util.Collection;
import java.util.List;
import java.util.UUID;

public record AuthenticatedUser(UUID id, String email, String role) implements UserDetails {{

    @Override
    public Collection<? extends GrantedAuthority> getAuthorities() {{
        return List.of(new SimpleGrantedAuthority("ROLE_" + role));
    }}

    @Override public String getPassword() {{ return null; }}
    @Override public String getUsername() {{ return email; }}
    @Override public boolean isAccountNonExpired() {{ return true; }}
    @Override public boolean isAccountNonLocked() {{ return true; }}
    @Override public boolean isCredentialsNonExpired() {{ return true; }}
    @Override public boolean isEnabled() {{ return true; }}
}}"#, pkg=pkg))?;

        write_file(&base.join("AuthenticatedUserProvider.java"), &format!(r#"package {pkg};

import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Component;

@Component
public class AuthenticatedUserProvider {{

    public AuthenticatedUser getCurrentUser() {{
        Object principal = SecurityContextHolder.getContext().getAuthentication().getPrincipal();
        if (principal instanceof AuthenticatedUser user) {{
            return user;
        }}
        throw new IllegalStateException("No authenticated user in context");
    }}
}}"#, pkg=pkg))?;

        write_file(&base.join("JwtAuthenticationFilter.java"), &format!(r#"package {pkg};

import {svc}.TokenService;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.Cookie;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.web.authentication.WebAuthenticationDetailsSource;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;
import java.io.IOException;
import java.util.UUID;

@Component
public class JwtAuthenticationFilter extends OncePerRequestFilter {{
    private final TokenService tokenService;

    public JwtAuthenticationFilter(TokenService tokenService) {{
        this.tokenService = tokenService;
    }}

    @Override
    protected void doFilterInternal(HttpServletRequest request,
                                    HttpServletResponse response,
                                    FilterChain filterChain) throws ServletException, IOException {{
        String token = extractFromCookie(request);

        if (token != null && tokenService.validateToken(token)) {{
            String email = tokenService.extractEmail(token);
            String role = tokenService.extractRole(token);
            UUID userId = tokenService.extractUserId(token);

            AuthenticatedUser user = new AuthenticatedUser(userId, email, role);
            UsernamePasswordAuthenticationToken auth =
                new UsernamePasswordAuthenticationToken(user, null, user.getAuthorities());
            auth.setDetails(new WebAuthenticationDetailsSource().buildDetails(request));
            SecurityContextHolder.getContext().setAuthentication(auth);
        }}

        filterChain.doFilter(request, response);
    }}

    private String extractFromCookie(HttpServletRequest request) {{
        Cookie[] cookies = request.getCookies();
        if (cookies != null) {{
            for (Cookie c : cookies) {{
                if ("token".equals(c.getName())) return c.getValue();
            }}
        }}
        return null;
    }}
}}"#, pkg=pkg, svc=svc))?;

        write_file(&base.join("SecurityConfig.java"), &format!(r#"package {pkg};

import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.CorsConfigurationSource;
import org.springframework.web.cors.UrlBasedCorsConfigurationSource;
import java.util.List;

@Configuration
@EnableWebSecurity
public class SecurityConfig {{
    private final JwtAuthenticationFilter jwtAuthenticationFilter;

    @Value("${{cors.allowed-origins:http://localhost:3000,http://localhost:5173}}")
    private String allowedOrigins;

    public SecurityConfig(JwtAuthenticationFilter jwtAuthenticationFilter) {{
        this.jwtAuthenticationFilter = jwtAuthenticationFilter;
    }}

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {{
        http
            .csrf(csrf -> csrf.disable())
            .cors(cors -> cors.configurationSource(corsConfigurationSource()))
            .sessionManagement(session -> session.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
            .authorizeHttpRequests(authz -> authz
                .requestMatchers("/swagger-ui/**", "/v3/api-docs/**").permitAll()
                .requestMatchers("/api/auth/**").permitAll()
                .requestMatchers("/api/admin/**").hasRole("ADMIN")
                .anyRequest().authenticated()
            )
            .addFilterBefore(jwtAuthenticationFilter, UsernamePasswordAuthenticationFilter.class);
        return http.build();
    }}

    @Bean
    public CorsConfigurationSource corsConfigurationSource() {{
        CorsConfiguration config = new CorsConfiguration();
        config.setAllowedOrigins(List.of(allowedOrigins.split(",")));
        config.setAllowedMethods(List.of("GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"));
        config.setAllowedHeaders(List.of("*"));
        config.setAllowCredentials(true);
        UrlBasedCorsConfigurationSource source = new UrlBasedCorsConfigurationSource();
        source.registerCorsConfiguration("/**", config);
        return source;
    }}
}}"#, pkg=pkg))
    }

    pub fn generate_application_class(&self) -> io::Result<()> {
        let pkg = format!("{}.infrastructure", self.domain);
        let app = self.app_class();
        let domain = &self.domain;
        let content = format!(r#"package {pkg};

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.domain.EntityScan;
import org.springframework.data.jpa.repository.config.EnableJpaRepositories;

@SpringBootApplication(scanBasePackages = "{domain}")
@EntityScan("{domain}.infrastructure.adapters.repositories.entities")
@EnableJpaRepositories("{domain}.infrastructure.adapters.repositories.jpa")
public class {app}Application {{
    public static void main(String[] args) {{
        SpringApplication.run({app}Application.class, args);
    }}
}}"#, pkg=pkg, domain=domain, app=app);
        write_file(&self.java_src("infrastructure").join(format!("{app}Application.java")), &content)
    }

    pub fn generate_application_properties(&self) -> io::Result<()> {
        let db = self.name.to_lowercase().replace('-', "_");
        let content = format!(r#"spring.application.name={name}

# Database
spring.datasource.url=jdbc:postgresql://${{DATABASE_HOST:localhost}}:${{DATABASE_PORT:5432}}/${{DATABASE_NAME:{db}}}
spring.datasource.username=${{DATABASE_USER:postgres}}
spring.datasource.password=${{DATABASE_PASSWORD:postgres}}
spring.datasource.driver-class-name=org.postgresql.Driver

# JPA
spring.jpa.hibernate.ddl-auto=update
spring.jpa.show-sql=false
spring.jpa.properties.hibernate.dialect=org.hibernate.dialect.PostgreSQLDialect
spring.jpa.properties.hibernate.format_sql=true

# JWT
jwt.secret=${{JWT_SECRET:change-this-secret-key-in-production}}
jwt.expiration=86400000

# CORS
cors.allowed-origins=${{CORS_ALLOWED_ORIGINS:http://localhost:3000,http://localhost:5173}}

# Swagger
springdoc.swagger-ui.path=/swagger-ui.html"#, name=self.name, db=db);
        let path = self.root.join("infrastructure/src/main/resources/application.properties");
        write_file(&path, &content)
    }
}
