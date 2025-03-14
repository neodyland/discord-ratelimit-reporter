# Discord Rate Limit Reporter

A lightweight server/client system that monitors Discord API rate limits and notifies you via a Discord webhook when limits are reached.

## Overview

Discord Rate Limit Reporter consists of two main components:

1. **Client**: Monitors Discord API rate limits by making periodic requests
2. **Server**: Receives reports from clients and sends notifications via Discord webhooks

This architecture enables centralized monitoring of Discord API rate limit states from multiple nodes or servers, providing immediate notifications when limits are exceeded.

## Key Features

- Monitor Discord API rate limits in distributed systems
- Immediate notifications when rate limits are exceeded
- API key authentication
- Docker and multi-architecture support (amd64/arm64)
- Simple configuration and deployment

## Quick Start

### Run with Docker Compose

```yaml
services:
  discord-ratelimit-server:
    image: ghcr.io/neodyland/discord-ratelimit-reporter/server:latest
    container_name: discord-ratelimit-server
    restart: unless-stopped
    ports:
      - "3000:3000"
    environment:
      - API_KEY=your-api-key
      - BIND=0.0.0.0:3000
      - DISCORD_WEBHOOK_URL=your-discord-webhook-url
      - DISCORD_WEBHOOK_MESSAGE=@everyone
```

```yml
services:
  discord-ratelimit-client:
    image: ghcr.io/neodyland/discord-ratelimit-reporter/client:latest
    container_name: discord-ratelimit-client
    restart: unless-stopped
    environment:
      - API_URL=http://discord-ratelimit-server:3000
      - API_KEY=your-api-key
      - NODE_NAME=your-node-name
```

### Environment Variables

#### Server Configuration (.env.server.example)

```bash
API_KEY="your-api-key"
BIND="0.0.0.0:3000"
DISCORD_WEBHOOK_URL="your-discord-webhook-url"
DISCORD_WEBHOOK_MESSAGE="@everyone"
```

#### Client Configuration (.env.client.example)

```bash
API_URL="http://localhost:3000"
API_KEY="your-api-key"
NODE_NAME="your-node-name"
```

Make sure to use the same API key for both the server and client.

## How It Works

### Client

The client sends requests to the Discord API every 60 seconds and checks the response. When it detects a rate limit (receiving a "You are being blocked" message), the client sends a report to the configured server.

```rust
let response = reqwest::get("https://discord.com/api/v10/invites/discord-developers").await?;
let body = response.json::<serde_json::Value>().await?;
if let Some(message) = body.get("message") {
    if message.as_str().unwrap().starts_with("You are being blocked") && !exceeded {
        // Report to server when rate limit detected
    }
}
```

### Server

The server receives reports from clients and sends notifications using Discord webhooks.

```rust
async fn ratelimit(
    headers: axum::http::HeaderMap,
    Json(body): Json<RatelimitRequest>,
)-> impl IntoResponse {
    // Verify API key authentication
    // Send notification to Discord webhook
}
```

## Security

- Communication between server and client is protected by an API key
- Containers run as non-privileged users

## Build Information

This project is written in Rust and automatically built and deployed using GitHub Actions CI/CD pipelines. It supports both x86_64 and aarch64 architectures.

## Contributing

Please report bugs and request features via GitHub Issues. Pull requests are welcome.

## License

This project is open source. See the license file in the repository for details.
