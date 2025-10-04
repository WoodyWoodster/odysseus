# Odysseus

Event ticketing platform built with Rust microservices, showcasing distributed systems patterns and hexagonal architecture.

## Features

- Event creation and management
- Multiple ticket types and pricing tiers
- Ticket inventory management
- Seat selection and reservations
- Payment processing (Stripe integration)
- Purchase confirmation emails
- QR code ticket generation
- Event reminders and notifications
- Refunds and cancellations
- Search and discovery
- Analytics and reporting
- User authentication and profiles

## Services

- **users/** - User management & authentication
- **events/** - Event creation, management, venue info
- **tickets/** - Inventory, reservations, seat management
- **payments/** - Payment processing, Stripe integration
- **notifications/** - Email/SMS confirmations, reminders

## Architecture

**Hexagonal Architecture (Ports & Adapters)**
- Domain-driven design with clear separation of concerns
- Business logic isolated from external dependencies
- Dependency inversion principle

## Microservices Patterns

- **Hexagonal Architecture** - Ports & adapters for clean separation
- **Event-Driven Architecture** - Asynchronous service communication
- **Saga Pattern** - Distributed transaction coordination
- **API Gateway** - Traefik for routing and load balancing
- **Circuit Breaker** - Fault tolerance and resilience
- **Eventual Consistency** - Distributed data management

## Technical Highlights

- **Race Condition Handling** - Safe concurrent ticket purchases
- **Idempotent Operations** - Prevent duplicate bookings
- **Distributed Transactions** - Multi-service saga orchestration
- **SeaORM Migrations** - Version-controlled schema evolution
- **OpenAPI Documentation** - Auto-generated Swagger UI per service

## Tech Stack

- **Language**: Rust
- **Web Framework**: Actix-web
- **ORM**: SeaORM
- **Database**: PostgreSQL
- **API Gateway**: Traefik
- **API Documentation**: OpenAPI 3.0 / Swagger UI
- **Containerization**: Docker & Docker Compose

## Structure

```
odysseus/
├── Cargo.toml
├── docker-compose.yml
├── shared/
│   └── src/
│       └── lib.rs
└── services/
    ├── users/
    │   ├── migration/
    │   └── src/
    │       ├── domain/
    │       │   ├── entities/
    │       │   ├── repositories/
    │       │   ├── use_cases/
    │       │   └── errors.rs
    │       └── adapters/
    │           ├── inbound/http/
    │           └── outbound/persistence/
    ├── events/
    ├── tickets/
    ├── payments/
    └── notifications/
```

Each service follows hexagonal architecture with:
- **domain/** - Business logic, entities, use cases
- **adapters/inbound/** - HTTP handlers, DTOs
- **adapters/outbound/** - Database repositories, external APIs
- **migration/** - SeaORM schema migrations

## Getting Started

### Prerequisites
- Docker & Docker Compose
- Rust (for local development)

### Running with Docker Compose

```bash
# Start all services
docker-compose up

# Access services via Traefik
# Users API: http://localhost/api/users
# Events API: http://localhost/api/events
# Tickets API: http://localhost/api/tickets

# Swagger UI (per service)
# Users: http://localhost/api/users/swagger-ui/
# Events: http://localhost/api/events/swagger-ui/
# Tickets: http://localhost/api/tickets/swagger-ui/
```

### Local Development

```bash
# Run database
docker-compose up postgres

# Run migrations
cd services/users
sea-orm-cli migrate up

# Run service
cargo run
```

### Database Migrations

```bash
# Create new migration
cd services/users
sea-orm-cli migrate generate <migration_name>

# Apply migrations
sea-orm-cli migrate up

# Rollback migration
sea-orm-cli migrate down
```
