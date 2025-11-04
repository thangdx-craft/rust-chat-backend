#!/usr/bin/env python3
"""
WebSocket Test Client for Chat Backend
Requires: pip install websockets
"""

import asyncio
import websockets
import json
import sys

async def test_websocket(token, room_id=1):
    uri = f"ws://localhost:3000/ws/{room_id}?token={token}"
    
    print(f"Connecting to {uri}...")
    
    async with websockets.connect(uri) as websocket:
        print("Connected!")
        
        # Send a test message
        test_message = {
            "type": "message",
            "content": "Hello from WebSocket test client!"
        }
        
        await websocket.send(json.dumps(test_message))
        print(f"Sent: {test_message}")
        
        # Listen for messages
        print("\nListening for messages (press Ctrl+C to stop)...")
        try:
            async for message in websocket:
                data = json.loads(message)
                print(f"\nReceived: {data}")
        except KeyboardInterrupt:
            print("\nDisconnecting...")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python test_websocket.py <JWT_TOKEN> [room_id]")
        print("\nFirst, get a JWT token by registering or logging in:")
        print('  curl -X POST http://localhost:3000/auth/login \\')
        print('    -H "Content-Type: application/json" \\')
        print('    -d \'{"email": "test@example.com", "password": "password123"}\'')
        sys.exit(1)
    
    token = sys.argv[1]
    room_id = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    
    asyncio.run(test_websocket(token, room_id))
