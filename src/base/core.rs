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
    pub fn generate_core_pom(&self) -> io::Result<()> {
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

    <artifactId>core</artifactId>
    <packaging>jar</packaging>
</project>"#, domain = self.domain, name = self.name);
        write_file(&self.root.join("core/pom.xml"), &content)
    }

    pub fn generate_admin_entity(&self) -> io::Result<()> {
        let pkg = format!("{}.core.domain.entities", self.domain);
        let content = format!(r#"package {pkg};

import java.util.Objects;
import java.util.UUID;

public class Admin {{
    private UUID id;
    private String email;
    private String password;
    private String name;

    public Admin() {{}}

    public Admin(String email, String password, String name) {{
        this.id = UUID.randomUUID();
        this.email = email;
        this.password = password;
        this.name = name;
    }}

    public Admin(UUID id, String email, String password, String name) {{
        this.id = id;
        this.email = email;
        this.password = password;
        this.name = name;
    }}

    public UUID getId() {{ return id; }}
    public String getEmail() {{ return email; }}
    public String getPassword() {{ return password; }}
    public String getName() {{ return name; }}
    public String getRole() {{ return "ADMIN"; }}

    public void setEmail(String email) {{ this.email = email; }}
    public void setPassword(String password) {{ this.password = password; }}
    public void setName(String name) {{ this.name = name; }}

    @Override
    public boolean equals(Object o) {{
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Admin admin = (Admin) o;
        return Objects.equals(id, admin.id) && Objects.equals(email, admin.email);
    }}

    @Override
    public int hashCode() {{ return Objects.hash(id, email); }}
}}"#, pkg = pkg);
        write_file(&self.java_src("core").join("domain/entities/Admin.java"), &content)
    }

    pub fn generate_cliente_entity(&self) -> io::Result<()> {
        let pkg = format!("{}.core.domain.entities", self.domain);
        let content = format!(r#"package {pkg};

import java.util.Objects;
import java.util.UUID;

public class Cliente {{
    private UUID id;
    private String email;
    private String password;
    private String name;

    public Cliente() {{}}

    public Cliente(String email, String password, String name) {{
        this.id = UUID.randomUUID();
        this.email = email;
        this.password = password;
        this.name = name;
    }}

    public Cliente(UUID id, String email, String password, String name) {{
        this.id = id;
        this.email = email;
        this.password = password;
        this.name = name;
    }}

    public UUID getId() {{ return id; }}
    public String getEmail() {{ return email; }}
    public String getPassword() {{ return password; }}
    public String getName() {{ return name; }}
    public String getRole() {{ return "CLIENT"; }}

    public void setEmail(String email) {{ this.email = email; }}
    public void setPassword(String password) {{ this.password = password; }}
    public void setName(String name) {{ this.name = name; }}

    @Override
    public boolean equals(Object o) {{
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Cliente c = (Cliente) o;
        return Objects.equals(id, c.id) && Objects.equals(email, c.email);
    }}

    @Override
    public int hashCode() {{ return Objects.hash(id, email); }}
}}"#, pkg = pkg);
        write_file(&self.java_src("core").join("domain/entities/Cliente.java"), &content)
    }
}
