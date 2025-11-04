#!/bin/bash

# API Testing Script for Chat Backend

BASE_URL="http://localhost:3000"

echo "=== Chat Backend API Test ==="
echo ""

# 1. Register a new user
echo "1. Registering user..."
REGISTER_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "username": "testuser"
  }')

echo "Register Response: $REGISTER_RESPONSE"
TOKEN=$(echo $REGISTER_RESPONSE | grep -o '"token":"[^"]*' | cut -d'"' -f4)
echo "Token: $TOKEN"
echo ""

# 2. Login
echo "2. Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }')

echo "Login Response: $LOGIN_RESPONSE"
TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*' | cut -d'"' -f4)
echo ""

# 3. Get rooms
echo "3. Getting rooms..."
ROOMS_RESPONSE=$(curl -s -X GET "$BASE_URL/rooms" \
  -H "Authorization: Bearer $TOKEN")

echo "Rooms Response: $ROOMS_RESPONSE"
echo ""

# 4. Post a message to room 1
echo "4. Posting message to room 1..."
MESSAGE_RESPONSE=$(curl -s -X POST "$BASE_URL/rooms/1/messages" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello from API test!"
  }')

echo "Message Response: $MESSAGE_RESPONSE"
echo ""

echo "=== Test Complete ==="
