You are a senior Rust backend engineer. Build a complete microservice backend for a Realtime Chat Application using the following stack:

**Tech Stack**
- Language: Rust (latest stable)
- Framework: Axum
- Async runtime: Tokio
- ORM: SeaORM or Diesel (choose the most stable for Axum)
- Database: PostgreSQL
- Auth: JWT (jsonwebtoken crate)
- Realtime: WebSocket (tokio_tungstenite or axum::extract::ws)
- Containerization: Docker + docker-compose (for app + postgres)
- Config management: dotenv or figment
- Logging: tracing + tracing-subscriber
- Error handling: thiserror
- Dependency management: cargo

**Functional Requirements**
1. Users can register and login with email + password (hashed with argon2)
2. Issue JWT token upon successful login; token required for protected routes
3. Establish WebSocket connection for authenticated users
4. Broadcast messages to all connected users in the same room
5. Store messages in PostgreSQL with fields: id, sender_id, room_id, content, created_at
6. Each user can join multiple rooms
7. REST endpoints:
   - POST /auth/register
   - POST /auth/login
   - GET /rooms
   - POST /rooms/:room_id/messages (for testing)
8. WebSocket endpoint: `/ws/:room_id`
   - Client sends `{ "type": "message", "content": "..." }`
   - Server broadcasts `{ "type": "message", "sender": "user", "content": "..." }`

**Project Structure**
src/
├── main.rs
├── config.rs
├── db/
│ └── mod.rs
├── models/
│ ├── user.rs
│ ├── room.rs
│ └── message.rs
├── routes/
│ ├── auth.rs
│ ├── room.rs
│ ├── websocket.rs
│ └── mod.rs
├── services/
│ ├── auth_service.rs
│ ├── jwt_service.rs
│ ├── message_service.rs
│ └── mod.rs
├── errors.rs
└── utils.rs

**Implementation Details**
- Use `async` throughout (no blocking).
- Use dependency injection pattern for database pool (Arc<DatabaseConnection> or PgPool).
- JWT middleware for protecting routes.
- Use `tower_http::cors` for handling CORS.
- Create `Dockerfile` and `docker-compose.yml` for local development.
- Write sample `.env` file with variables: `DATABASE_URL`, `JWT_SECRET`, `RUST_LOG`, `PORT`.
- Include example `curl` commands or WebSocket test script (Python or Node).

**Output**
- Generate a production-ready codebase with all files and modules.
- Include explanations or comments in code for clarity.
- Ensure it compiles successfully with `cargo build` and runs with `docker-compose up`.

**Goal**
Deliver a fully functional, modular, and clean Rust backend for a realtime chat microservice that can be extended easily (e.g. add notifications, typing indicators later).