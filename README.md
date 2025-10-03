# Odysseus

Rust microservices monorepo built with Cargo workspaces.

## Architecture

**Hexagonal Architecture (Ports & Adapters)**
- Domain-driven design with clear separation of concerns
- Business logic isolated from external dependencies
- Dependency inversion principle

## Tech Stack

- **Language**: Rust
- **Web Framework**: Actix-web
- **ORM**: SeaORM
- **Database**: PostgreSQL
- **API Documentation**: OpenAPI 3.0 / Swagger UI
- **Containerization**: Docker & Docker Compose

## Structure

```
odysseus/
├── Cargo.toml
├── shared/
│   └── src/
│       └── lib.rs
└── services/
    └── users/
        └── src/
            ├── domain/
            │   ├── entities/
            │   ├── repositories/
            │   ├── commands/
            │   ├── use_cases/
            │   └── errors.rs
            └── adapters/
                ├── inbound/http/
                └── outbound/persistence/
```
