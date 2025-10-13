

## developerStartSpringboot
This is a terminal application designed for developers starting a Spring 
Boot API who want to bypass the tedious initial steps of creating layers, 
folders, and authentication.

![dss terminal](terminal.png)

### How to Use
To get started, simply install the tool and run the command `dss init` in 
your terminal. Answer a few basic questions, and in seconds, you'll have 
the complete foundation for your Spring Boot API.

### The application is generated in three layers: core, application, and infrastructure.

| **Layer** | **Main Responsibility** | **Knows About** | **Example Classes / Components** |
|------------|-------------------------|------------------|-----------------------------------|
| **Core (Domain)** | Contains the core business logic and entities. Defines domain rules, invariants, and pure data models — completely independent of frameworks. | None | `User`, `Order`, `Product`, `Payment`, `Money`, `Email` |
| **Application** | Orchestrates use cases and coordinates the business logic using the domain entities. Handles transactions, validation, and flow control. | Core | `CreateUserService`, `ProcessPaymentUseCase`, `OrderManager`, `DtoMapper` |
| **Infrastructure** | Implements the technical details and integrates with external systems (databases, APIs, file storage, messaging, etc.). Contains controllers, repositories, and configurations. | Application (and sometimes Core) | `UserRepositoryJpa`, `PaymentApiClient`, `EmailServiceImpl`, `UserController`, `SpringConfig` |


## Gustavo Priftis
