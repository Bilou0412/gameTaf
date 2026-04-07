# GT - Multiplayer Quiz Game

Classic trivia game playable in terminal via UDP on local network (LAN).

## Quick Install (One-liner)

```bash
curl -fsSL https://raw.githubusercontent.com/Bilou0412/gameTaf/main/install.sh | bash
```

## After Installation

Start server:
```bash
gt server
```

Join game:
```bash
gt client
```

See [Installation Guide](INSTALL.md) for more details.

## Game Flow

1. Server hosts the quiz questions
2. Clients connect and answer questions
3. Score is tracked per player
4. Game ends after all questions are answered

## Architecture

- **Server (server.rs)**: Hosts quiz questions and manages player scores
- **Client (client.rs)**: Player interface for answering questions
- **Modules**:
  - `game::Message`: Quiz question and answer protocol
  - `renderer::Renderer`: ASCII display for game state

## Network Configuration

Server listens on `0.0.0.0:9999`

To play on different computers:
1. Find server IP: `ipconfig` (Windows) or `ifconfig` (Linux/Mac)
2. Run clients with that IP

## Features

- Multiplayer trivia quiz
- Real-time UDP communication
- ASCII text-based interface
- Automatic score tracking
- Simple binary question protocol

## Troubleshooting

**Clients can't connect?**
- Ensure server is running (gt server)
- Check IP address (use 127.0.0.1 on same machine)
- Verify firewall allows UDP port 9999

## License

Free to use and modify.
