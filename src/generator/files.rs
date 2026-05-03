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

use crate::config::PhysicalLayer;
use crate::utils::{to_pascal_case, to_app_name_clean, domain_to_path, write_file};
use super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub fn generate_config_files(&self, layer: &PhysicalLayer, path: &Path) -> io::Result<()> {
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

    pub fn generate_security_config(&self, layer: &PhysicalLayer, path: &Path) -> io::Result<()> {
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

    pub fn generate_application_class(&self, layer: &PhysicalLayer) -> io::Result<()> {
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

    pub fn generate_application_properties(&self, layer: &PhysicalLayer, path: &Path) -> io::Result<()> {
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

    pub fn generate_gitignore(&self) -> io::Result<()> {
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

    pub fn generate_gitattributes(&self) -> io::Result<()> {
        let content = r#"* text=auto eol=lf
*.bat text eol=crlf
*.cmd text eol=crlf
"#;

        write_file(&self.root.join(".gitattributes"), content)
    }

    pub fn generate_mvnw(&self) -> io::Result<()> {
        let wrapper_path = self.root.join(".mvn/wrapper");
        fs::create_dir_all(&wrapper_path)?;

        let wrapper_props = r#"distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.6/apache-maven-3.9.6-bin.zip
wrapperUrl=https://repo.maven.apache.org/maven2/org/apache/maven/wrapper/maven-wrapper/3.2.0/maven-wrapper-3.2.0.jar
"#;
        write_file(&wrapper_path.join("maven-wrapper.properties"), wrapper_props)?;

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
        echo "Error: curl or wget is required to download Maven." >&2
        exit 1
    fi
    mkdir -p "$MAVEN_HOME"
    unzip -q "$TMP_DIR/$DIST_FILE" -d "$TMP_DIR/extract"
    mv "$TMP_DIR/extract/${DIST_NAME}"/* "$MAVEN_HOME/"
    rm -rf "$TMP_DIR"
    echo "Maven downloaded to $MAVEN_HOME"
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

        let mvnw_cmd = r#"@echo off
setlocal

set "MAVEN_PROJECTBASEDIR=%~dp0"
set "WRAPPER_PROPERTIES=%MAVEN_PROJECTBASEDIR%.mvn\wrapper\maven-wrapper.properties"
set "DISTRIBUTION_URL=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.6/apache-maven-3.9.6-bin.zip"

for /f "tokens=2 delims==" %%i in ('findstr "^distributionUrl" "%WRAPPER_PROPERTIES%" 2^>nul') do set "DISTRIBUTION_URL=%%i"

for %%f in ("%DISTRIBUTION_URL%") do set "DIST_FILE=%%~nxf"
set "DIST_NAME=%DIST_FILE:-bin.zip=%"
set "MAVEN_HOME=%USERPROFILE%\.m2\wrapper\dists\%DIST_NAME%"

if not exist "%MAVEN_HOME%\bin\mvn.cmd" (
    echo Downloading Maven %DIST_NAME%...
    powershell -Command "Invoke-WebRequest -Uri '%DISTRIBUTION_URL%' -OutFile '%TEMP%\%DIST_FILE%'"
    powershell -Command "Expand-Archive -Path '%TEMP%\%DIST_FILE%' -DestinationPath '%USERPROFILE%\.m2\wrapper\dists' -Force"
    echo Maven downloaded.
)

"%MAVEN_HOME%\bin\mvn.cmd" %*
"#;
        write_file(&self.root.join("mvnw.cmd"), mvnw_cmd)
    }
}
