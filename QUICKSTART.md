# Quick Start Guide

## Prerequisites

- Docker and Docker Compose (recommended)
- OR Rust 1.75+ and PostgreSQL 14+ (for local development)

## Setup and Run

### Using Docker Compose (Easiest)

1. **Clone or navigate to the project:**
```bash
cd /Users/dangxuanthang/demo/rust-begin/chat-backend
```

2. **Start the application:**
```bash
docker-compose up --build
```

This will:
- Start PostgreSQL database
- Run database migrations automatically
- Build and start the Rust backend
- Expose the API on `http://localhost:3000`

### Local Development (Without Docker)

1. **Install dependencies:**
```bash
# Make sure PostgreSQL is installed and running
brew install postgresql@16  # macOS
```

2. **Setup environment:**
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. **Create database and run migrations:**
```bash
createdb chatdb
psql -U postgres -d chatdb -f init.sql
```

4. **Run the application:**
```bash
cargo run
```

## Testing the API

### 1. Register a User

```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alice@example.com",
    "password": "password123",
    "username": "Alice"
  }'
```

Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "user": {
    "id": 1,
    "email": "alice@example.com",
    "username": "Alice"
  }
}
```

### 2. Login

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alice@example.com",
    "password": "password123"
  }'
```

### 3. Get Rooms (Protected)

```bash
# Save your token from the registration/login response
TOKEN="your_jwt_token_here"

curl -X GET http://localhost:3000/rooms \
  -H "Authorization: Bearer $TOKEN"
```

Response:
```json
[
  {
    "id": 1,
    "name": "General",
    "created_at": "2024-01-01T00:00:00"
  },
  {
    "id": 2,
    "name": "Random",
    "created_at": "2024-01-01T00:00:00"
  }
]
```

### 4. Send a Message (Testing Endpoint)

```bash
curl -X POST http://localhost:3000/rooms/1/messages \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello, World!"
  }'
```

## Testing WebSocket

### Using the Python Test Script

1. **Install websockets:**
```bash
pip install websockets
```

2. **Get a JWT token** (from register/login above)

3. **Run the WebSocket client:**
```bash
python test_websocket.py YOUR_JWT_TOKEN 1
```

This will:
- Connect to room 1
- Send a test message
- Listen for incoming messages from other users

### Using wscat (Alternative)

```bash
# Install wscat
npm install -g wscat

# Connect to WebSocket (replace TOKEN with your JWT)
wscat -c "ws://localhost:3000/ws/1?token=YOUR_JWT_TOKEN"

# Send a message
{"type": "message", "content": "Hello from wscat!"}
```

### Multiple Users Testing

Open 2+ terminal windows and run the Python script with different tokens:

**Terminal 1 (Alice):**
```bash
# Register and get token for Alice
ALICE_TOKEN=$(curl -s -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@example.com","password":"pass123","username":"Alice"}' \
  | jq -r '.token')

python test_websocket.py $ALICE_TOKEN 1
```

**Terminal 2 (Bob):**
```bash
# Register and get token for Bob
BOB_TOKEN=$(curl -s -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"bob@example.com","password":"pass123","username":"Bob"}' \
  | jq -r '.token')

python test_websocket.py $BOB_TOKEN 1
```

Now messages sent by Alice will appear in Bob's terminal and vice versa!

## Using the Bash Test Script

```bash
chmod +x test_api.sh
./test_api.sh
```

This script will:
1. Register a new user
2. Login
3. Get all rooms
4. Post a message to room 1

## Stopping the Application

### Docker Compose:
```bash
docker-compose down
```

### Local:
```bash
# Press Ctrl+C in the terminal running cargo run
```

## Troubleshooting

### Port Already in Use
```bash
# Find process using port 3000
lsof -i :3000
# Kill it
kill -9 <PID>
```

### Database Connection Issues
```bash
# Check if PostgreSQL is running
docker-compose ps
# Or for local PostgreSQL
pg_isready
```

### Rebuild Docker Images
```bash
docker-compose down -v
docker-compose up --build
```

### View Logs
```bash
# All services
docker-compose logs -f

# Just the backend
docker-compose logs -f chat-backend

# Just the database
docker-compose logs -f postgres
```

## Architecture Overview

```
┌─────────────┐
│   Client    │
│  (Browser/  │
│   Mobile)   │
└──────┬──────┘
       │
       │ HTTP/WS
       ▼
┌─────────────────────────────────────────┐
│         Axum Web Server                 │
│  ┌────────────┐  ┌─────────────────┐   │
│  │   Routes   │  │   Middleware    │   │
│  │  - Auth    │  │   - JWT Auth    │   │
│  │  - Rooms   │  │   - CORS        │   │
│  │  - WebSocket│  │   - Logging     │   │
│  └────────────┘  └─────────────────┘   │
│  ┌────────────────────────────────┐    │
│  │        Services Layer          │    │
│  │  - AuthService                 │    │
│  │  - JwtService                  │    │
│  │  - MessageService              │    │
│  └────────────────────────────────┘    │
│  ┌────────────────────────────────┐    │
│  │       SeaORM Models            │    │
│  │  - User                        │    │
│  │  - Room                        │    │
│  │  - Message                     │    │
│  └────────────────────────────────┘    │
└──────────────┬──────────────────────────┘
               │
               │ SQL
               ▼
       ┌───────────────┐
       │  PostgreSQL   │
       │   Database    │
       └───────────────┘
```

## Next Steps

1. ✅ Basic authentication works
2. ✅ REST API endpoints functional
3. ✅ WebSocket real-time messaging operational
4. ✅ Database persistence enabled

**Enhancements to consider:**
- Add room creation/management endpoints
- Implement user profiles
- Add message history pagination
- Implement typing indicators
- Add read receipts
- File upload support
- Push notifications
- Rate limiting
- Admin panel

## Support

For issues or questions:
1. Check logs: `docker-compose logs -f`
2. Verify database: `docker-compose exec postgres psql -U chatuser -d chatdb`
3. Test API endpoints individually
4. Check environment variables in `.env`

Enjoy building your realtime chat application! 🚀
