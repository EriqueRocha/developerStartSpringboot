# developerStartSpringboot

A terminal application for developers who want to quickly bootstrap a Spring Boot API with a clean, layered architecture. Skip the tedious initial setup of creating layers, folders, entities, and authentication.

![dss terminal](terminal.png)

## Features

- **Flexible Architecture**: Choose any number of physical layers (Maven modules) and logical packages
- **Multiple Entities**: Generate as many domain entities as you need
- **Spring Security**: Optional JWT-based authentication
- **Swagger/OpenAPI**: Built-in API documentation
- **Example Endpoints**: Ready-to-test controllers
- **JSON Configuration**: Import/export project configurations

## Quick Start

```bash
# Interactive mode - answer questions to generate your project
dss init

# Generate from JSON configuration file
dss generate -c my-project.json

# Export a configuration template
dss template -o my-template.json
```

## Commands

| Command | Description |
|---------|-------------|
| `dss init` | Interactive mode - guides you through project setup |
| `dss generate -c <file>` | Generate project from JSON configuration |
| `dss template [-o <file>]` | Export default configuration template |
| `dss version` | Show version information |

## JSON Configuration

You can fully customize your project structure using a JSON configuration file:

```json
{
  "project": {
    "name": "myAPI",
    "domain": "com.example.demo",
    "description": "My Spring Boot API",
    "developer": {
      "name": "Your Name",
      "email": "you@example.com",
      "url": "example.com"
    }
  },
  "layers": {
    "physical": [
      {
        "name": "core",
        "logical": ["domain/entities", "domain/valueobjects"],
        "dependencies": [],
        "is_main": false
      },
      {
        "name": "application",
        "logical": ["usecases", "ports/repositories", "ports/services"],
        "dependencies": ["core"],
        "is_main": false
      },
      {
        "name": "infrastructure",
        "logical": ["adapters/web/controllers", "adapters/web/dto", "adapters/repositories", "config"],
        "dependencies": ["core", "application"],
        "is_main": true
      }
    ]
  },
  "entities": [
    { "name": "User", "role": "USER" },
    { "name": "Admin", "role": "ADMIN" }
  ],
  "features": {
    "spring_security": true,
    "example_endpoints": true,
    "swagger": true,
    "flyway": false
  }
}
```

### Configuration Options

#### Physical Layers
Define your Maven modules. Each layer can have:
- `name`: Module name (e.g., "core", "domain", "infrastructure")
- `logical`: List of package paths within the module
- `dependencies`: Other modules this one depends on
- `is_main`: Set to `true` for the module containing the Spring Boot Application class

#### Entities
List of domain entities to generate:
- `name`: Entity name in PascalCase
- `role`: Security role associated with this entity

#### Features
Toggle optional features:
- `spring_security`: JWT authentication and authorization
- `example_endpoints`: Test controllers with `/api/test/*` endpoints
- `swagger`: OpenAPI documentation at `/swagger-ui.html`
- `flyway`: Database migration support

## Supported Architectures

The tool supports any layered architecture:

| Architecture | Layers | Example |
|-------------|--------|---------|
| **Monolithic** | 1 | Single module with all code |
| **Clean Architecture** | 3 | core, application, infrastructure |
| **Hexagonal** | 3+ | domain, ports, adapters |
| **Custom** | N | Any combination you define |

### Default 3-Layer Architecture

| Layer | Responsibility | Depends On |
|-------|---------------|------------|
| **Core** | Domain entities, value objects, business rules | None |
| **Application** | Use cases, ports (interfaces for repositories/services) | Core |
| **Infrastructure** | Controllers, JPA repositories, configurations, external integrations | Core, Application |

## Installation

### Ubuntu / Debian

**Option A - Quick install:**
```bash
curl -fsSL https://eriquerocha.github.io/developerStartSpringboot/install-deb.sh | sudo bash
```

**Option B - Manual install:**
```bash
# Download
wget https://github.com/EriqueRocha/developerStartSpringboot/releases/download/Ubuntu-linux/developerstartspringboot_0.1.0-1_amd64.deb

# Install
sudo dpkg -i developerstartspringboot_0.1.0-1_amd64.deb

# Use
dss init
```

### Fedora / RHEL

**Option A - Direct install:**
```bash
sudo dnf install https://github.com/EriqueRocha/developerStartSpringboot/releases/download/Fedora-linux/developerStartSpringboot-0.1.0-1.x86_64.rpm
```

**Option B - Manual install:**
```bash
# Download
wget https://github.com/EriqueRocha/developerStartSpringboot/releases/download/Fedora-linux/developerStartSpringboot-0.1.0-1.x86_64.rpm

# Install
sudo dnf install ./developerStartSpringboot-0.1.0-1.x86_64.rpm

# Use
dss init
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/EriqueRocha/developerStartSpringboot.git
cd developerStartSpringboot

# Build with Cargo
cargo build --release

# Binary will be at ./target/release/dss
```

## Requirements

- Java 21+ (for generated projects)
- Maven 3.9+ (for generated projects)

## Example Usage

### Interactive Mode

```bash
$ dss init

=== Project Configuration ===

Application name (e.g.: myAPI): orderService
Domain (e.g.: com.example.demo): com.mycompany.orders
Description [Spring Boot API]: Order Management API

=== Physical Layers (Maven Modules) ===

Number of physical layers [3]: 3
Layer 1 name [core]: domain
Logical packages [domain/entities,domain/valueobjects]: entities,valueobjects
...

=== Features ===

Include Spring Security? [y/N]: y
Include example endpoints for testing? [Y/n]: y
Include Swagger/OpenAPI documentation? [Y/n]: y

Generate project with this configuration? [Y/n]: y
```

### From JSON File

```bash
# Export template
dss template -o my-project.json

# Edit the JSON file as needed
nano my-project.json

# Generate project
dss generate -c my-project.json
```

## Generated Project Structure

```
myAPI/
├── pom.xml                          # Parent POM
├── core/
│   ├── pom.xml
│   └── src/main/java/.../core/
│       └── domain/
│           ├── entities/
│           │   └── User.java
│           └── valueobjects/
├── application/
│   ├── pom.xml
│   └── src/main/java/.../application/
│       ├── usecases/
│       │   └── CreateUserUseCase.java
│       └── ports/
│           └── repositories/
│               └── UserRepository.java
└── infrastructure/
    ├── pom.xml
    └── src/main/java/.../infrastructure/
        ├── adapters/
        │   ├── web/
        │   │   ├── controllers/
        │   │   │   ├── UserController.java
        │   │   │   └── TestController.java
        │   │   └── dto/
        │   │       ├── CreateUserRequest.java
        │   │       └── CreateUserResponse.java
        │   └── repositories/
        │       ├── JpaUserRepository.java
        │       ├── entities/
        │       │   └── UserEntity.java
        │       └── jpa/
        │           └── UserJpaRepository.java
        ├── config/
        │   ├── BeanConfiguration.java
        │   ├── doc/
        │   │   └── OpenAPIConfiguration.java
        │   └── security/          # (if spring_security enabled)
        │       ├── SecurityConfig.java
        │       └── JwtAuthenticationFilter.java
        └── MyAPIApplication.java
```

## License

This project is licensed under the GNU Affero General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
