# Migration Guide

## Initial Database Setup

### Option 1: Using Docker Compose (Recommended)

The `init.sql` file will automatically run when the PostgreSQL container starts for the first time:

```bash
docker-compose up -d postgres
```

### Option 2: Manual Setup

If running PostgreSQL locally:

```bash
# Create database
createdb chatdb

# Run migrations
psql -U postgres -d chatdb -f init.sql
```

### Option 3: Using psql directly

```bash
psql -U chatuser -d chatdb -f init.sql
```

## Database Schema

### Users Table
Stores user authentication information.

| Column | Type | Description |
|--------|------|-------------|
| id | SERIAL | Primary key |
| email | VARCHAR(255) | User email (unique) |
| password_hash | VARCHAR(255) | Argon2 hashed password |
| username | VARCHAR(100) | Display name |
| created_at | TIMESTAMP | Account creation time |

### Rooms Table
Stores chat rooms.

| Column | Type | Description |
|--------|------|-------------|
| id | SERIAL | Primary key |
| name | VARCHAR(255) | Room name |
| created_at | TIMESTAMP | Room creation time |

### Messages Table
Stores all chat messages.

| Column | Type | Description |
|--------|------|-------------|
| id | SERIAL | Primary key |
| sender_id | INTEGER | Foreign key to users.id |
| room_id | INTEGER | Foreign key to rooms.id |
| content | TEXT | Message content |
| created_at | TIMESTAMP | Message timestamp |

### Room Members Table
Tracks room membership.

| Column | Type | Description |
|--------|------|-------------|
| id | SERIAL | Primary key |
| room_id | INTEGER | Foreign key to rooms.id |
| user_id | INTEGER | Foreign key to users.id |
| joined_at | TIMESTAMP | Join timestamp |

## Indexes

- `idx_messages_room_id`: Optimizes message queries by room
- `idx_messages_sender_id`: Optimizes message queries by sender
- `idx_messages_created_at`: Optimizes time-based queries
- `idx_room_members_user_id`: Optimizes user membership lookups
- `idx_room_members_room_id`: Optimizes room member lists

## Sample Data

The migration includes 3 sample rooms:
- General
- Random
- Tech Talk
