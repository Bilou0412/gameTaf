# GT - Multiplayer Pong Game

Classic Pong game playable in terminal via UDP on local network (LAN).

## Quick Install (One-liner)

```bash
curl -fsSL https://raw.githubusercontent.com/Bilou0412/gameTaf/main/install.sh | bash
```

## After Installation

```bash
gt
```

Interactive menu to choose:
- Start Server
- Join Game (Client)
- Show Controls

See [Installation Guide](INSTALL.md) for more details.

## Game Controls

| Key | Action |
|-----|--------|
| **W/Z** | Move paddle up |
| **S** | Move paddle down |
| **Q** | Quit game |

## Architecture

- **Server (server.rs)**: Manages game logic and state synchronization
- **Client (client.rs)**: Player interface and network communication
- **Modules**:
  - `game::GameState`: Physics and collision detection
  - `renderer::Renderer`: ASCII display rendering
  - `input::InputHandler`: Keyboard input handling

## Network Configuration

Server listens on `0.0.0.0:9999`

To play on different computers:
1. Find server IP: `ipconfig` (Windows) or `ifconfig` (Linux/Mac)
2. Run clients with that IP

## Features

- Classic 2-player Pong
- Real-time UDP communication
- ASCII rendering with borders
- Automatic scoring system
- Realistic ball physics
- 80x24 character display

## Troubleshooting

**Clients can't connect?**
- Ensure server is running
- Check IP address (use `localhost` on same machine)
- Check firewall (UDP port 9999)

**Display looks corrupted?**
- Expand terminal to at least 80x24 characters
- Use a terminal supporting Unicode

## License

Free to use and modify.
