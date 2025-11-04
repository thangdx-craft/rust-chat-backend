# Chat Backend - Realtime Chat Application

A complete microservice backend for a realtime chat application built with Rust, Axum, SeaORM, and WebSockets.

## Features

- ✅ User registration and authentication with JWT
- ✅ Password hashing with Argon2
- ✅ RESTful API endpoints
- ✅ WebSocket support for real-time messaging
- ✅ PostgreSQL database with SeaORM
- ✅ Room-based chat system
- ✅ Docker and docker-compose setup
- ✅ Comprehensive error handling
- ✅ CORS support
- ✅ Structured logging with tracing

## Tech Stack

- **Language**: Rust (latest stable)
- **Framework**: Axum
- **Async Runtime**: Tokio
- **ORM**: SeaORM
- **Database**: PostgreSQL
- **Auth**: JWT (jsonwebtoken)
- **Password Hashing**: Argon2
- **WebSocket**: Axum WebSocket
- **Logging**: tracing + tracing-subscriber
- **Error Handling**: thiserror
- **Containerization**: Docker + docker-compose

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── errors.rs            # Error types and handling
├── utils.rs             # JWT middleware
├── db/
│   └── mod.rs          # Database connection
├── models/
│   ├── user.rs         # User entity and DTOs
│   ├── room.rs         # Room entity and DTOs
│   ├── message.rs      # Message entity and DTOs
│   ├── room_member.rs  # Room membership entity
│   └── mod.rs
├── routes/
│   ├── auth.rs         # Authentication routes
│   ├── room.rs         # Room and message routes
│   ├── websocket.rs    # WebSocket handler
│   └── mod.rs
└── services/
    ├── auth_service.rs     # Authentication logic
    ├── jwt_service.rs      # JWT token management
    ├── message_service.rs  # Message operations
    └── mod.rs
```

## API Endpoints

### Authentication

#### Register
```bash
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure_password",
  "username": "username"
}

Response: { "token": "jwt_token", "user": {...} }
```

#### Login
```bash
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure_password"
}

Response: { "token": "jwt_token", "user": {...} }
```

### Rooms (Protected)

#### Get All Rooms
```bash
GET /rooms
Authorization: Bearer <jwt_token>

Response: [{ "id": 1, "name": "General", "created_at": "..." }, ...]
```

#### Post Message (Testing)
```bash
POST /rooms/:room_id/messages
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "content": "Hello, world!"
}

Response: { "id": 1, "sender_id": 1, "room_id": 1, "content": "...", "created_at": "..." }
```

### WebSocket

```
ws://localhost:3000/ws/:room_id?token=<jwt_token>

Client sends:
{
  "type": "message",
  "content": "Hello!"
}

Server broadcasts:
{
  "type": "message",
  "sender": "user@example.com",
  "sender_id": 1,
  "content": "Hello!"
}
```

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Rust (if running locally without Docker)

### Running with Docker Compose

1. Copy the environment file:
```bash
cp .env.example .env
```

2. Start the services:
```bash
docker-compose up --build
```

The API will be available at `http://localhost:3000`

### Running Locally

1. Install dependencies and set up PostgreSQL

2. Copy and configure `.env`:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. Run database migrations:
```bash
psql -U chatuser -d chatdb -f init.sql
```

4. Build and run:
```bash
cargo build --release
cargo run
```

## Testing

### Test REST API

Make the test script executable and run it:
```bash
chmod +x test_api.sh
./test_api.sh
```

Or use curl directly:
```bash
# Register
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123", "username": "testuser"}'

# Login
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}'

# Get rooms (replace TOKEN with your JWT)
curl -X GET http://localhost:3000/rooms \
  -H "Authorization: Bearer TOKEN"

# Post message
curl -X POST http://localhost:3000/rooms/1/messages \
  -H "Authorization: Bearer TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello from API!"}'
```

### Test WebSocket

1. Install Python websockets:
```bash
pip install websockets
```

2. Get a JWT token first (from register or login)

3. Run the WebSocket test client:
```bash
python test_websocket.py YOUR_JWT_TOKEN 1
```

## Database Schema

### users
- id (SERIAL PRIMARY KEY)
- email (VARCHAR UNIQUE)
- password_hash (VARCHAR)
- username (VARCHAR)
- created_at (TIMESTAMP)

### rooms
- id (SERIAL PRIMARY KEY)
- name (VARCHAR)
- created_at (TIMESTAMP)

### messages
- id (SERIAL PRIMARY KEY)
- sender_id (INTEGER FK -> users)
- room_id (INTEGER FK -> rooms)
- content (TEXT)
- created_at (TIMESTAMP)

### room_members
- id (SERIAL PRIMARY KEY)
- room_id (INTEGER FK -> rooms)
- user_id (INTEGER FK -> users)
- joined_at (TIMESTAMP)

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT signing
- `PORT`: Server port (default: 3000)
- `RUST_LOG`: Logging level (debug, info, warn, error)

## Development

### Building
```bash
cargo build
```

### Running tests
```bash
cargo test
```

### Checking code
```bash
cargo clippy
cargo fmt
```

## Production Considerations

1. **Security**:
   - Change `JWT_SECRET` to a strong random string
   - Use HTTPS in production
   - Implement rate limiting
   - Add input validation

2. **Performance**:
   - Add database connection pooling (already configured in SeaORM)
   - Implement message pagination
   - Add Redis for session management

3. **Monitoring**:
   - Set up structured logging
   - Add health check endpoints
   - Implement metrics collection

## Future Enhancements

- [ ] Typing indicators
- [ ] Read receipts
- [ ] File uploads
- [ ] Push notifications
- [ ] User presence status
- [ ] Direct messages
- [ ] Message editing/deletion
- [ ] Search functionality
- [ ] Rate limiting

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
